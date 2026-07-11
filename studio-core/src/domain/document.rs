// src/domain/document.rs

use super::styles::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Pdf,
    Html,
    Docx, // Future
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentRequest {
    pub template_id: String,
    pub data: serde_json::Value,
    pub content: Vec<Node>,

    #[serde(default)]
    pub page: Option<PageSettings>,

    #[serde(default)]
    pub format: OutputFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "paragraph")]
    Paragraph {
        content: Vec<InlineContent>,
        alignment: Option<String>,
    },
    #[serde(rename = "heading")]
    Heading {
        level: u8,
        content: Vec<InlineContent>,
        alignment: Option<String>,
    },
    #[serde(rename = "bullet_list")]
    BulletList { items: Vec<Vec<InlineContent>> },
    #[serde(rename = "table")]
    Table {
        headers: Option<Vec<String>>,
        rows: Option<Vec<Vec<TableCellContent>>>,
        loop_data: Option<String>,
        row_template: Option<Vec<TableCellContent>>,
        footer: Option<Vec<TableCellContent>>,
        style: Option<TableStyle>,
    },
    #[serde(rename = "page_break")]
    PageBreak,
    #[serde(rename = "spacer")]
    Spacer { height: String },
    #[serde(rename = "image")]
    Image {
        src: String,
        width: Option<String>,
        height: Option<String>,
        alignment: Option<String>,
    },
    #[serde(rename = "shape")]
    Shape {
        kind: String,
        width: String,
        height: String,
        fill: Option<String>,
        rotate: Option<String>,
    },
    #[serde(rename = "placed")]
    Placed {
        anchor: Option<String>,
        dx: Option<String>,
        dy: Option<String>,
        content: Box<Node>,
    },
    #[serde(rename = "columns")]
    Columns {
        items: Vec<Vec<Node>>,
        column_widths: Option<Vec<String>>,
        gutter: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InlineContent {
    Text(TextNode),
    Variable(VariableNode),
    PageNumber(PageNumberNode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNumberNode {
    pub page_number: bool,
    pub format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextNode {
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub text: String,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub font_family: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableNode {
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub key: String,
}
