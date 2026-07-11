// src/domain/styles.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageSettings {
    pub header: Option<PageHeaderFooter>,
    pub footer: Option<PageHeaderFooter>,
    pub background: Option<Vec<super::document::Node>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageHeaderFooter {
    pub content: Vec<super::document::InlineContent>,
    pub alignment: Option<String>,
    pub first_page_only: Option<bool>,
    pub skip_first_page: Option<bool>,
    pub first_page_content: Option<Vec<super::document::InlineContent>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TableStyle {
    pub width: Option<String>,
    pub columns: Option<Vec<String>>,
    pub inset: Option<String>,
    pub stroke: Option<String>,
    pub header_bg: Option<String>,
    pub column_align: Option<Vec<String>>,
    pub repeat_header: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TableCellContent {
    Variable { key: String, bold: Option<bool> },
    Text { text: String, bold: Option<bool> },
}
