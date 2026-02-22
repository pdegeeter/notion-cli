use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::multipart;
use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use std::time::Duration;
use tokio::fs;

const NOTION_API_BASE: &str = "https://api.notion.com";
const NOTION_VERSION: &str = "2025-09-03";
const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF_MS: u64 = 500;

pub struct NotionClient {
    client: Client,
    base_url: String,
    dry_run: bool,
}

impl NotionClient {
    pub fn new(token: &str) -> Result<Self> {
        Self::with_base_url(token, NOTION_API_BASE)
    }

    pub fn with_base_url(token: &str, base_url: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .context("Invalid API token format")?,
        );
        headers.insert(
            "Notion-Version",
            HeaderValue::from_static(NOTION_VERSION),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
            dry_run: false,
        })
    }

    pub fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }

    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);

        self.send_with_retry(|| {
            let mut req = self.client.get(&url);
            if !query.is_empty() {
                req = req.query(query);
            }
            req
        }, "GET", path)
        .await
    }

    pub async fn post(&self, path: &str, body: Option<&Value>) -> Result<Value> {
        if self.dry_run {
            return self.print_dry_run("POST", path, body);
        }

        let url = format!("{}{}", self.base_url, path);

        self.send_with_retry(|| {
            let mut req = self.client.post(&url);
            if let Some(body) = body {
                req = req.json(body);
            }
            req
        }, "POST", path)
        .await
    }

    pub async fn patch(&self, path: &str, body: &Value) -> Result<Value> {
        if self.dry_run {
            return self.print_dry_run("PATCH", path, Some(body));
        }

        let url = format!("{}{}", self.base_url, path);

        self.send_with_retry(|| {
            self.client.patch(&url).json(body)
        }, "PATCH", path)
        .await
    }

    pub async fn delete(&self, path: &str) -> Result<Value> {
        if self.dry_run {
            return self.print_dry_run::<Value>("DELETE", path, None);
        }

        let url = format!("{}{}", self.base_url, path);

        self.send_with_retry(|| {
            self.client.delete(&url)
        }, "DELETE", path)
        .await
    }

    pub async fn post_multipart(
        &self,
        path: &str,
        file_path: &Path,
        part_number: Option<u32>,
    ) -> Result<Value> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        let file_bytes = fs::read(file_path)
            .await
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        if self.dry_run {
            eprintln!("[dry-run] POST {}{}", self.base_url, path);
            eprintln!(
                "[dry-run] File: {} ({} bytes)",
                file_path.display(),
                file_bytes.len()
            );
            if let Some(pn) = part_number {
                eprintln!("[dry-run] Part number: {}", pn);
            }
            return Ok(serde_json::json!({
                "dry_run": true,
                "method": "POST",
                "path": path,
                "file": file_path.display().to_string(),
                "file_size": file_bytes.len(),
            }));
        }

        let url = format!("{}{}", self.base_url, path);
        let mime = mime_from_filename(&file_name);

        let mut attempt = 0;
        loop {
            let file_part = multipart::Part::bytes(file_bytes.clone())
                .file_name(file_name.clone())
                .mime_str(&mime)
                .context("Invalid MIME type")?;

            let mut form = multipart::Form::new().part("file", file_part);
            if let Some(pn) = part_number {
                form = form.text("part_number", pn.to_string());
            }

            let response = self
                .client
                .post(&url)
                .multipart(form)
                .send()
                .await
                .with_context(|| format!("POST {}", path))?;

            let status = response.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && attempt < MAX_RETRIES {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());

                let wait_ms = retry_after
                    .map(|s| s * 1000)
                    .unwrap_or(INITIAL_BACKOFF_MS * 2u64.pow(attempt));

                eprintln!(
                    "Rate limited (429). Retrying in {}ms (attempt {}/{})",
                    wait_ms,
                    attempt + 1,
                    MAX_RETRIES
                );
                tokio::time::sleep(Duration::from_millis(wait_ms)).await;
                attempt += 1;
                continue;
            }

            return self.handle_response_with_status(status, response).await;
        }
    }

    fn print_dry_run<T: serde::Serialize>(&self, method: &str, path: &str, body: Option<&T>) -> Result<Value> {
        eprintln!("[dry-run] {} {}{}", method, self.base_url, path);
        if let Some(body) = body {
            eprintln!("[dry-run] Body: {}", serde_json::to_string_pretty(body)?);
        }
        Ok(serde_json::json!({"dry_run": true, "method": method, "path": path}))
    }

    async fn send_with_retry<F>(&self, build_request: F, method: &str, path: &str) -> Result<Value>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let mut attempt = 0;
        loop {
            let response = build_request()
                .send()
                .await
                .with_context(|| format!("{} {}", method, path))?;

            let status = response.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && attempt < MAX_RETRIES {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());

                let wait_ms = retry_after
                    .map(|s| s * 1000)
                    .unwrap_or(INITIAL_BACKOFF_MS * 2u64.pow(attempt));

                eprintln!(
                    "Rate limited (429). Retrying in {}ms (attempt {}/{})",
                    wait_ms,
                    attempt + 1,
                    MAX_RETRIES
                );
                tokio::time::sleep(Duration::from_millis(wait_ms)).await;
                attempt += 1;
                continue;
            }

            return self.handle_response_with_status(status, response).await;
        }
    }

    async fn handle_response_with_status(
        &self,
        status: reqwest::StatusCode,
        response: reqwest::Response,
    ) -> Result<Value> {
        let body: Value = response
            .json()
            .await
            .context("Failed to parse response as JSON")?;

        if !status.is_success() {
            let message = body["message"]
                .as_str()
                .unwrap_or("Unknown error");
            let code = body["code"].as_str().unwrap_or("unknown");
            anyhow::bail!("Notion API error ({}): [{}] {}", status, code, message);
        }

        Ok(body)
    }
}

fn mime_from_filename(filename: &str) -> String {
    let ext = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "csv" => "text/csv",
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "zip" => "application/zip",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio;

    #[test]
    fn test_new_client_with_valid_token() {
        let client = NotionClient::new("ntn_test_token");
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_client_with_empty_token() {
        // Empty token is valid (will fail at API level, not at construction)
        let client = NotionClient::new("");
        assert!(client.is_ok());
    }

    #[test]
    fn test_with_base_url() {
        let client = NotionClient::with_base_url("token", "http://localhost:8080").unwrap();
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_default_base_url() {
        let client = NotionClient::new("token").unwrap();
        assert_eq!(client.base_url, "https://api.notion.com");
    }

    #[test]
    fn test_notion_version_constant() {
        assert_eq!(NOTION_VERSION, "2025-09-03");
    }

    #[tokio::test]
    async fn test_get_with_mock_server() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/users/me")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"object":"user","id":"abc-123","name":"Test Bot","type":"bot"}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("test_token", &server.url()).unwrap();
        let result = client.get("/v1/users/me", &[]).await.unwrap();

        assert_eq!(result["object"], "user");
        assert_eq!(result["name"], "Test Bot");
        assert_eq!(result["type"], "bot");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_with_query_params() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/users")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("page_size".into(), "10".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"results":[],"has_more":false}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.get("/v1/users", &[("page_size", "10")]).await.unwrap();

        assert_eq!(result["has_more"], false);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_with_body() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/search")
            .match_body(mockito::Matcher::Json(json!({"query": "test"})))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"results":[{"id":"page-1"}],"has_more":false}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let body = json!({"query": "test"});
        let result = client.post("/v1/search", Some(&body)).await.unwrap();

        assert_eq!(result["results"][0]["id"], "page-1");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_without_body() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/endpoint")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.post("/v1/endpoint", None).await.unwrap();

        assert_eq!(result["ok"], true);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_patch_request() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PATCH", "/v1/pages/page-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"page-123","properties":{}}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let body = json!({"properties": {"Name": {"title": [{"text": {"content": "Updated"}}]}}});
        let result = client.patch("/v1/pages/page-123", &body).await.unwrap();

        assert_eq!(result["id"], "page-123");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete_request() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/v1/blocks/block-456")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"block-456","archived":true}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.delete("/v1/blocks/block-456").await.unwrap();

        assert_eq!(result["archived"], true);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_handle_api_error_401() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/users/me")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"object":"error","status":401,"code":"unauthorized","message":"API token is invalid."}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("bad_token", &server.url()).unwrap();
        let result = client.get("/v1/users/me", &[]).await;

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unauthorized"));
        assert!(err.contains("API token is invalid"));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_handle_api_error_404() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/pages/nonexistent")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"object":"error","status":404,"code":"object_not_found","message":"Could not find page."}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.get("/v1/pages/nonexistent", &[]).await;

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("object_not_found"));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_handle_api_error_with_missing_fields() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/test")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"unexpected":"format"}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.get("/v1/test", &[]).await;

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown error"));
        mock.assert_async().await;
    }

    #[test]
    fn test_dry_run_default_is_false() {
        let client = NotionClient::new("token").unwrap();
        assert!(!client.dry_run);
    }

    #[test]
    fn test_set_dry_run() {
        let mut client = NotionClient::new("token").unwrap();
        client.set_dry_run(true);
        assert!(client.dry_run);
    }

    #[tokio::test]
    async fn test_dry_run_post_does_not_send_request() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/pages")
            .expect(0)
            .create_async()
            .await;

        let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
        client.set_dry_run(true);
        let body = json!({"properties": {}});
        let result = client.post("/v1/pages", Some(&body)).await.unwrap();

        assert_eq!(result["dry_run"], true);
        assert_eq!(result["method"], "POST");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_dry_run_patch_does_not_send_request() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PATCH", "/v1/pages/p1")
            .expect(0)
            .create_async()
            .await;

        let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
        client.set_dry_run(true);
        let body = json!({});
        let result = client.patch("/v1/pages/p1", &body).await.unwrap();

        assert_eq!(result["dry_run"], true);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_dry_run_delete_does_not_send_request() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/v1/blocks/b1")
            .expect(0)
            .create_async()
            .await;

        let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
        client.set_dry_run(true);
        let result = client.delete("/v1/blocks/b1").await.unwrap();

        assert_eq!(result["dry_run"], true);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_retry_on_429_then_success() {
        let mut server = mockito::Server::new_async().await;
        let mock_429 = server
            .mock("GET", "/v1/test")
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_header("retry-after", "0")
            .with_body(r#"{"object":"error","status":429,"code":"rate_limited","message":"Rate limited"}"#)
            .expect(1)
            .create_async()
            .await;
        let mock_200 = server
            .mock("GET", "/v1/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .expect(1)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client.get("/v1/test", &[]).await.unwrap();

        assert_eq!(result["ok"], true);
        mock_429.assert_async().await;
        mock_200.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_multipart_with_mock_server() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/file_uploads/upload-1/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"upload-1","status":"uploaded"}"#)
            .create_async()
            .await;

        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.png");
        tokio::fs::write(&file_path, b"fake png content").await.unwrap();

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = client
            .post_multipart("/v1/file_uploads/upload-1/send", &file_path, None)
            .await
            .unwrap();

        assert_eq!(result["id"], "upload-1");
        assert_eq!(result["status"], "uploaded");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_multipart_dry_run() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/file_uploads/upload-1/send")
            .expect(0)
            .create_async()
            .await;

        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        tokio::fs::write(&file_path, b"hello").await.unwrap();

        let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
        client.set_dry_run(true);
        let result = client
            .post_multipart("/v1/file_uploads/upload-1/send", &file_path, Some(1))
            .await
            .unwrap();

        assert_eq!(result["dry_run"], true);
        assert_eq!(result["method"], "POST");
        assert_eq!(result["file_size"], 5);
        mock.assert_async().await;
    }

    #[test]
    fn test_mime_from_filename() {
        assert_eq!(super::mime_from_filename("photo.png"), "image/png");
        assert_eq!(super::mime_from_filename("doc.pdf"), "application/pdf");
        assert_eq!(super::mime_from_filename("data.csv"), "text/csv");
        assert_eq!(super::mime_from_filename("unknown.xyz"), "application/octet-stream");
        assert_eq!(super::mime_from_filename("IMAGE.JPG"), "image/jpeg");
    }
}
