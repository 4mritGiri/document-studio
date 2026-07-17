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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SecurityConfig {
    pub user_password: Option<String>, // Password required to OPEN the document
    pub owner_password: Option<String>, // Password required to CHANGE permissions (print/copy)

    /// Permission flags — only meaningful to a viewer once a password is
    /// set, and only enforceable via the owner password (a user who knows
    /// only the user password is bound by these; whoever knows the owner
    /// password can always override them, per the PDF spec). Default is
    /// deliberately restrictive (false) rather than permissive: previously
    /// this was hardcoded to grant ALL permissions unconditionally, which
    /// meant an owner_password provided no actual restriction, only a
    /// password gate on opening the file.
    #[serde(default)]
    pub allow_printing: bool,
    #[serde(default)]
    pub allow_copying: bool,
    #[serde(default)]
    pub allow_modification: bool,
    #[serde(default)]
    pub allow_annotation: bool,
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

    #[serde(default)]
    pub security: Option<SecurityConfig>,
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

    /// Deprecated in favor of `Qr` (see below) — kept only so existing
    /// templates don't break. `data` here is a plain, already-resolved
    /// string (the caller must substitute any dynamic values themselves
    /// before sending the request), unlike `Qr`'s `content` field which
    /// supports `{"key": "..."}` variable resolution server-side.
    #[serde(rename = "qr_code")]
    QrCode {
        data: String,
        width: Option<String>,
        alignment: Option<String>,
    },

    #[serde(rename = "qr")]
    Qr {
        /// The QR payload — same InlineContent mechanism as paragraph text, so
        /// you can mix literal text with {"key": "..."} variable references
        /// (e.g. build a UPI/payment URL from data fields).
        content: Vec<InlineContent>,
        size: Option<String>,             // e.g. "3cm", default "3cm"
        fill: Option<String>,             // module color, default "#000000"
        background: Option<String>,       // default "#ffffff"
        error_correction: Option<String>, // "low" | "medium" | "quartile" | "high"
        alignment: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextNode {
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub text: String,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub font_family: Option<String>,

    /// e.g. "14pt", "1.2em". Validated via safe_typst_token before use —
    /// never trust this directly into generated source.
    pub size: Option<String>,
    /// e.g. "#1a1a1a", "red". Validated via color_expr before use.
    pub color: Option<String>,
    pub underline: Option<bool>,
    pub strike: Option<bool>,
    /// If set, wraps this text run in a hyperlink. Validated/escaped as a
    /// Typst string literal (escape_typst_string_literal), not markup text.
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableNode {
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub key: String,
}
