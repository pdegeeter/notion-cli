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
#[path = "client_tests.rs"]
mod tests;
