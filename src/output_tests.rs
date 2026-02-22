use super::*;
use serde_json::json;

#[test]
fn test_output_format_parse_pretty() {
    let format: OutputFormat = "pretty".parse().unwrap();
    assert!(matches!(format, OutputFormat::Pretty));
}

#[test]
fn test_output_format_parse_json() {
    let format: OutputFormat = "json".parse().unwrap();
    assert!(matches!(format, OutputFormat::Json));
}

#[test]
fn test_output_format_parse_raw() {
    let format: OutputFormat = "raw".parse().unwrap();
    assert!(matches!(format, OutputFormat::Raw));
}

#[test]
fn test_output_format_parse_case_insensitive() {
    let format: OutputFormat = "PRETTY".parse().unwrap();
    assert!(matches!(format, OutputFormat::Pretty));

    let format: OutputFormat = "Json".parse().unwrap();
    assert!(matches!(format, OutputFormat::Json));
}

#[test]
fn test_output_format_parse_invalid() {
    let result: std::result::Result<OutputFormat, _> = "xml".parse();
    assert!(result.is_err());
}

#[test]
fn test_output_format_default_is_pretty() {
    let format = OutputFormat::default();
    assert!(matches!(format, OutputFormat::Pretty));
}

#[test]
fn test_print_result_raw_is_compact() {
    let value = json!({"key": "value"});
    // Raw should produce compact JSON (no newlines within)
    let raw = serde_json::to_string(&value).unwrap();
    assert!(!raw.contains('\n'));
    assert!(raw.contains("\"key\":\"value\""));
}

#[test]
fn test_print_result_pretty_is_indented() {
    let value = json!({"key": "value"});
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    assert!(pretty.contains('\n'));
    assert!(pretty.contains("  "));
}

#[test]
fn test_print_result_handles_nested_json() {
    let value = json!({
        "results": [
            {"id": "abc", "type": "page"},
            {"id": "def", "type": "database"}
        ],
        "has_more": false
    });
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    assert!(pretty.contains("\"results\""));
    assert!(pretty.contains("\"has_more\""));
}

#[test]
fn test_print_result_raw_format() {
    let value = json!({"key": "value"});
    let result = print_result(&value, &OutputFormat::Raw);
    assert!(result.is_ok());
}

#[test]
fn test_print_result_json_format() {
    let value = json!({"key": "value"});
    let result = print_result(&value, &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_print_result_pretty_format() {
    let value = json!({"key": "value"});
    let result = print_result(&value, &OutputFormat::Pretty);
    assert!(result.is_ok());
}

#[test]
fn test_print_error_does_not_panic() {
    print_error("test error message");
}

#[test]
fn test_print_success_does_not_panic() {
    print_success("test success message");
}

#[test]
fn test_print_info_does_not_panic() {
    print_info("test info message");
}
