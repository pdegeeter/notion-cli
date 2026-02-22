use anyhow::Result;
use colored::Colorize;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Json,
    Raw,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(OutputFormat::Pretty),
            "json" => Ok(OutputFormat::Json),
            "raw" => Ok(OutputFormat::Raw),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub fn print_result(value: &Value, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Raw => {
            println!("{}", serde_json::to_string(value)?);
        }
        OutputFormat::Json | OutputFormat::Pretty => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
    }
    Ok(())
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", "→".blue().bold(), msg);
}

#[cfg(test)]
#[path = "output_tests.rs"]
mod tests;
