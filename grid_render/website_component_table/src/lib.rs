mod common;
mod custom_element;
mod table_column_definition;
mod table_style;
pub use website_base::struct_node::HtmlNodeRender;
pub use website_base::document::HtmlNode;

#[derive(Debug, Clone)]
pub struct TableBuilder {
    /// Table styling options
    /// example: 
    /// ```ignore
    /// style: TableStyle::default_dark()
    /// ```
    /// or
    /// ```ignore
    /// style: Default::default()
    /// ```
    pub style: TableStyle,
    /// Enable dynamic rendering for large tables using virtual scrolling.
    /// 
    /// When true, only visible rows/columns are rendered in the DOM.
    pub dynamic_table_render: bool,
    pub columns: Vec<TableColumnDefinition>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone)]
pub struct InnerTableBuilder {
    pub col_span: usize,
    /// The starting column index in the parent table where this inner table should be placed.
    /// (0-based index).
    pub start_col_index: usize,
    pub style: TableStyle,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone)]
pub struct TableStyle {
    pub css_entries: Vec<&'static str>,
    /// Cell margin in pixels
    pub margin_px: usize,
    /// Cell padding in pixels
    pub padding_px: usize,
}

/// Column definition - can be a leaf column or a group of columns
#[derive(Debug, Clone)]
pub enum TableColumnDefinition {
    Column(TableColumn),
    Group(TableColumnGroup),
}

/// A group header that contains nested columns
#[derive(Debug, Clone)]
pub struct TableColumnGroup {
    /// Display text for the group header
    pub text: String,
    /// Optional unique identifier
    pub name: String,
    /// Child columns (can be nested groups or leaf columns)
    pub columns: Vec<TableColumnDefinition>,
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub name: String,
    pub text: String,
    pub width_px: usize,
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableValue>,
}

#[derive(Debug, Clone)]
pub enum TableValue {
    Null,
    Int(i128),
    Float(f64),
    String(String),
    Index(usize),
    Boolean(bool),
    Array(InnerTableBuilder),
}
