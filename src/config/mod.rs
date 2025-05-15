
use std::fs;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Span {
    pub row: usize,
    pub col: usize,
    pub span: usize,
    pub alignment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TableConfig {
    pub alignment: Vec<String>,
    pub width: Option<String>,
    pub centering: Option<bool>,
    pub horizontal_borders: Vec<usize>,
    pub vertical_borders: Vec<usize>,
    pub multicolumns: Vec<Span>,
    pub multirows: Vec<Span>,
    pub caption: Option<String>,
}

pub fn load_config(path: &str) -> Result<TableConfig, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: TableConfig = serde_json::from_str(&content)?;
    Ok(config)
}