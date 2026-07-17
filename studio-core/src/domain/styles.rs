// src/domain/styles.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageSettings {
    pub header: Option<PageHeaderFooter>,
    pub footer: Option<PageHeaderFooter>,
    pub background: Option<Vec<super::document::Node>>,
    pub watermark: Option<WatermarkSettings>,
    pub default_font: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatermarkSettings {
    pub text: String,
    pub opacity: Option<f32>,      // 0.0 to 1.0 (default 0.2)
    pub angle: Option<f32>,        // degrees (default -45.0)
    pub position: Option<String>,  // e.g., "top-left", "center"
    pub font_size: Option<String>, // e.g., "50pt"
    pub color: Option<String>,     // e.g., "gray", "#ff0000"
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
    /// Alternating row background — e.g. `["#ffffff", "#f3f4f6"]`. Applies
    /// to loop_data-generated rows (the common case: invoice line items).
    pub striped_rows: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TableCellContent {
    Variable {
        key: String,
        bold: Option<bool>,
        #[serde(default)]
        colspan: Option<u32>,
        #[serde(default)]
        rowspan: Option<u32>,
    },
    Text {
        text: String,
        bold: Option<bool>,
        #[serde(default)]
        colspan: Option<u32>,
        #[serde(default)]
        rowspan: Option<u32>,
    },

    // Excel-like calculations
    Formula {
        formula: String,        // e.g., "=qty * price" or "=sum(total)"
        format: Option<String>, // e.g., "NPR {value:,.2f}"
        bold: Option<bool>,
        #[serde(default)]
        colspan: Option<u32>,
        #[serde(default)]
        rowspan: Option<u32>,
    },
}
