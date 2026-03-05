use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup,
    TableRow, TableStyle, TableValue,
};

use crate::bigquery::{base::{TableFieldSchema, TableSchema}, jobs::GetQueryResultsResponse};

// ── VS Code theme-aware table styles ─────────────────────────────────────────────
/// Main result table: uses VS Code editor colours so it adapts to dark/light themes.
fn vscode_main_style() -> TableStyle {
    TableStyle {
        margin_px: 1,
        padding_px: 6,
        css_entries: vec![
            "th div.text { padding: 3px 6px; }",
            "th, td.index {
              background-color: var(--vscode-banner-background, #2a2d2e);
              color: var(--vscode-editor-foreground, #cccccc);
              border: 1px solid var(--vscode-settings-checkboxBorder, #454545);
              text-align: left;
              font-weight: 600;
            }",
            "td {
              background-color: var(--vscode-editor-background, #1e1e1e);
              color: var(--vscode-editor-foreground, #cccccc);
              border: 1px solid var(--vscode-settings-checkboxBorder, #454545);
              padding: 3px 6px;
            }",
            "td.null { font-style: italic; color: var(--vscode-disabledForeground, #888888); }",
            "td.boolean { color: var(--vscode-debugIcon-startForeground, #89d185); font-weight: 500; }",
            "td.number { text-align: right; color: var(--vscode-symbolIcon-numberForeground, #b5cea8); }",
            "td.string { color: var(--vscode-editor-foreground, #cccccc); }",
            "td.array { padding: 0; }",
            "td.array div div.ias {
              padding: 3px 6px;
              background-color: var(--vscode-settings-checkboxBorder, #454545);
              font-style: italic;
              color: var(--vscode-disabledForeground, #888888);
              text-align: center;
              font-size: 0.9em;
            }",
            "tr:hover td { background-color: var(--vscode-list-hoverBackground, #2a2d2e); }",
            "td.array div { max-height: 200px; overflow-y: auto; overflow-x: clip; }",
        ],
    }
}

/// Nested array table (level 1): uses a slightly recessed VS Code palette to visually
/// distinguish sub-arrays from the outer table while still following the active theme.
fn vscode_nested_style() -> TableStyle {
    TableStyle {
        margin_px: 1,
        padding_px: 6,
        css_entries: vec![
            "th div.text { padding: 3px 6px; }",
            "th, td.index {
              background-color: var(--vscode-editorWidget-background, #252526);
              color: var(--vscode-editor-foreground, #cccccc);
              border: 1px solid var(--vscode-editorWidget-border, #454545);
              text-align: left;
              font-weight: 600;
            }",
            "td {
              background-color: var(--vscode-sideBar-background, #252526);
              color: var(--vscode-sideBar-foreground, #cccccc);
              border: 1px solid var(--vscode-editorWidget-border, #3c3c3c);
              padding: 3px 6px;
            }",
            "td.null { font-style: italic; color: var(--vscode-disabledForeground, #888888); }",
            "td.boolean { color: var(--vscode-debugIcon-startForeground, #89d185); font-weight: 500; }",
            "td.number { text-align: right; color: var(--vscode-symbolIcon-numberForeground, #b5cea8); }",
            "td.string { color: var(--vscode-sideBar-foreground, #cccccc); }",
            "td.array { padding: 0; }",
            "td.array div div.ias {
              padding: 3px 6px;
              background-color: var(--vscode-editorGroupHeader-tabsBackground, #252526);
              font-style: italic;
              color: var(--vscode-disabledForeground, #888888);
              text-align: center;
              font-size: 0.9em;
            }",
            "tr:hover td { background-color: var(--vscode-list-activeSelectionBackground, #37373d); }",
            "td.array div { max-height: 150px; overflow-y: auto; overflow-x: clip; }",
        ],
    }
}

// ── Dynamic column-width constants ────────────────────────────────────────────────
/// Estimated pixels per character (~13 px UI font).
const CHAR_WIDTH_PX: usize = 8;
/// Horizontal cell padding: `padding: 5px 15px` → 30 px (15 × 2).
const CELL_PADDING_PX: usize = 30;
const MIN_COL_WIDTH_PX: usize = 80;
const MAX_COL_WIDTH_PX: usize = 500;

fn clamp_width(px: usize) -> usize {
    px.max(MIN_COL_WIDTH_PX).min(MAX_COL_WIDTH_PX)
}

/// Approximate rendered width of a cell value in characters.
fn value_char_len(v: &TableValue) -> usize {
    match v {
        TableValue::String(s) => s.chars().count(),
        TableValue::Boolean(_) => 5, // "false"
        TableValue::Null => 4,        // "null"
        TableValue::Index(n) => n.to_string().len(),
        TableValue::Int(n) => n.to_string().len(),
        TableValue::Float(f) => f.to_string().len(),
        TableValue::Array(_) => 0,    // rendered as inner table; skip
    }
}

/// Set `width_px` on every column starting from `start_col` using the wider of
/// header vs. content.  Columns before `start_col` are left untouched.
fn patch_column_widths_from(
    columns: &mut Vec<TableColumnDefinition>,
    rows: &[TableRow],
    start_col: usize,
) {
    for (col_idx, col_def) in columns.iter_mut().enumerate() {
        if col_idx < start_col {
            continue;
        }
        match col_def {
            TableColumnDefinition::Column(col) => {
                let header_chars = col.text.chars().count();
                let max_content_chars = rows
                    .iter()
                    .filter_map(|r| r.cells.get(col_idx))
                    .map(value_char_len)
                    .max()
                    .unwrap_or(0);
                let desired =
                    header_chars.max(max_content_chars) * CHAR_WIDTH_PX + CELL_PADDING_PX;
                col.width_px = clamp_width(desired);
            }
            TableColumnDefinition::Group(group) => {
                // RECORD fields: the row cell is a TableValue::Array (inner table).
                // Size sub-columns from their header text since there is no flat content.
                patch_group_header_widths(&mut group.columns);
            }
        }
    }
}

/// Resize every column including the leading row-index column (position 0).
fn patch_column_widths(columns: &mut Vec<TableColumnDefinition>, rows: &[TableRow]) {
    patch_column_widths_from(columns, rows, 0);
}

/// Resize every column starting from position 0 — use for tables that have no
/// leading index column (e.g. error and DML result tables).
pub(crate) fn patch_all_column_widths(
    columns: &mut Vec<TableColumnDefinition>,
    rows: &[TableRow],
) {
    patch_column_widths_from(columns, rows, 0);
}

fn patch_group_header_widths(cols: &mut Vec<TableColumnDefinition>) {
    for col_def in cols.iter_mut() {
        match col_def {
            TableColumnDefinition::Column(col) => {
                let desired = col.text.chars().count() * CHAR_WIDTH_PX + CELL_PADDING_PX;
                col.width_px = clamp_width(desired);
            }
            TableColumnDefinition::Group(g) => patch_group_header_widths(&mut g.columns),
        }
    }
}

impl GetQueryResultsResponse {
    pub(crate) fn to_table_builder(&self, row_index: usize) -> TableBuilder {
        let mut columns = get_columns(&self.schema);
        let rows = get_rows(&self.rows, &self.schema, row_index);
        patch_column_widths(&mut columns, &rows);
        TableBuilder {
            style: vscode_main_style(),
            dynamic_table_render: false,
            columns,
            rows,
        }
    }
}

impl crate::bigquery::tables::Table {
    pub(crate) fn to_table_builder(
        &self,
        rows: &Option<Vec<serde_json::Value>>,
        row_index: usize,
    ) -> TableBuilder {
        let mut columns = get_columns(&self.schema);
        let built_rows = get_rows(rows, &self.schema, row_index);
        patch_column_widths(&mut columns, &built_rows);
        TableBuilder {
            style: vscode_main_style(),
            dynamic_table_render: false,
            columns,
            rows: built_rows,
        }
    }
}

fn get_columns(schema: &Option<crate::bigquery::base::TableSchema>) -> Vec<TableColumnDefinition> {
    let column_row = TableColumnDefinition::Column(TableColumn {
        text: "#".to_string(),
        name: "index".to_string(),
        width_px: 50,
    });

    if let Some(schema) = schema {
        let mut columns = vec![column_row];
        columns.extend(
            schema
                .fields
                .iter()
                .map(|field| field.to_table_column_definition()),
        );
        columns
    } else {
        vec![column_row]
    }
}

fn get_rows(
    rows: &Option<Vec<serde_json::Value>>,
    schema: &Option<TableSchema>,
    row_index: usize,
) -> Vec<TableRow> {
    let fields: &[TableFieldSchema] = schema
        .as_ref()
        .map(|s| s.fields.as_slice())
        .unwrap_or(&[]);

    if let Some(rows) = rows {
        rows.iter()
            .enumerate()
            .map(|(index, row)| json_value_to_row(row, fields, row_index + index))
            .collect()
    } else {
        vec![]
    }
}

fn json_value_to_row(
    value: &serde_json::Value,
    fields: &[TableFieldSchema],
    row_index: usize,
) -> TableRow {
    let f = if let Some(obj) = value.as_object() {
        if let Some(f) = obj.get("f") {
            match f {
                serde_json::Value::Array(arr) => Some(arr),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    let cells = if let Some(f) = f {
        let mut cells = vec![TableValue::Index(row_index)];
        // leaf_start tracks the 0-based flat leaf index of the current field.
        // The index column occupies flat leaf 0, so data fields start at 1.
        let mut leaf_start: usize = 1;
        for (i, cell) in f.iter().enumerate() {
            let field = fields.get(i);
            let field_type = field.map(|f| f.r#type.as_str()).unwrap_or("");
            let nested_fields = field.and_then(|f| f.fields.as_deref()).unwrap_or(&[]);
            let mode = field.and_then(|f| f.mode.as_deref()).unwrap_or("");
            let leaf_count = if nested_fields.is_empty() {
                1
            } else {
                count_leaf_fields(nested_fields)
            };
            cells.extend(flatten_value_to_cells(cell, field_type, nested_fields, leaf_start, mode));
            leaf_start += leaf_count;
        }
        cells
    } else {
        vec![]
    };

    TableRow { cells }
}

/// Format a BigQuery TIMESTAMP (seconds since Unix epoch, possibly in scientific notation)
/// as an ISO 8601 string using the JS Date API.
#[cfg(target_arch = "wasm32")]
fn format_timestamp(s: &str) -> String {
    if let Ok(seconds) = s.parse::<f64>() {
        let ms = seconds * 1000.0;
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
        if let Some(iso) = date.to_iso_string().as_string() {
            return iso;
        }
    }
    s.to_string()
}

/// Non-wasm fallback: return the raw timestamp string as-is.
#[cfg(not(target_arch = "wasm32"))]
fn format_timestamp(s: &str) -> String {
    s.to_string()
}

/// Flatten a cell value into one or more `TableValue` cells.
///
/// Non-repeated STRUCT fields (mode != "REPEATED") are recursively expanded so
/// their sub-field values become individual cells aligned with the leaf columns
/// produced by the matching `TableColumnDefinition::Group`.
///
/// REPEATED fields (arrays) and scalar fields produce a single cell as before.
fn flatten_value_to_cells(
    value: &serde_json::Value,
    field_type: &str,
    nested_fields: &[TableFieldSchema],
    start_col_index: usize,
    mode: &str,
) -> Vec<TableValue> {
    let v = value.pointer("/v").unwrap_or_default();

    // Non-repeated STRUCT with sub-fields: flatten into individual cells
    if mode != "REPEATED" && !nested_fields.is_empty() {
        if let serde_json::Value::Object(_) = v {
            if let Some(f_array) = v.pointer("/f").and_then(|f| f.as_array()) {
                let mut cells = Vec::new();
                let mut sub_start = start_col_index;
                for (i, cell) in f_array.iter().enumerate() {
                    let nf = nested_fields.get(i);
                    let nf_type = nf.map(|f| f.r#type.as_str()).unwrap_or("");
                    let nf_nested = nf.and_then(|f| f.fields.as_deref()).unwrap_or(&[]);
                    let nf_mode = nf.and_then(|f| f.mode.as_deref()).unwrap_or("");
                    let leaf_count = if nf_nested.is_empty() {
                        1
                    } else {
                        count_leaf_fields(nf_nested)
                    };
                    // Recursively flatten nested structs
                    cells.extend(flatten_value_to_cells(cell, nf_type, nf_nested, sub_start, nf_mode));
                    sub_start += leaf_count;
                }
                return cells;
            }
        }
        // NULL struct: emit Null for each leaf column
        if v.is_null() {
            let leaf_count = count_leaf_fields(nested_fields);
            return vec![TableValue::Null; leaf_count];
        }
    }

    // Default: single cell via the normal conversion
    vec![json_value_to_table_value(value, field_type, nested_fields, start_col_index)]
}

fn json_value_to_table_value(
    value: &serde_json::Value,
    field_type: &str,
    nested_fields: &[TableFieldSchema],
    start_col_index: usize,
) -> TableValue {
    let v = value.pointer("/v").unwrap_or_default();

    match v {
        serde_json::Value::Null => TableValue::Null,
        serde_json::Value::Bool(b) => TableValue::Boolean(b.clone()),
        serde_json::Value::Number(n) => {
            if field_type == "TIMESTAMP" {
                TableValue::String(format_timestamp(&n.to_string()))
            } else {
                TableValue::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => {
            if field_type == "TIMESTAMP" {
                TableValue::String(format_timestamp(s))
            } else {
                TableValue::String(s.clone())
            }
        }
        serde_json::Value::Array(arr) => {
            // col_span must equal the number of leaf columns so the <td colspan="N">
            // spans exactly the N sub-column headers produced by the Group definition.
            let col_span = count_leaf_fields(nested_fields).max(1);

            let rows = if nested_fields.is_empty() {
                // Simple repeated field: ARRAY<FLOAT64>, ARRAY<STRING>, etc.
                // Each element has the shape {"v": <primitive>} — no "/f" layer.
                arr.iter()
                    .filter_map(|item| {
                        let v = item.pointer("/v")?;
                        let cell = match v {
                            serde_json::Value::String(s) => {
                                if field_type == "TIMESTAMP" {
                                    TableValue::String(format_timestamp(s))
                                } else {
                                    TableValue::String(s.clone())
                                }
                            }
                            serde_json::Value::Number(n) => {
                                if field_type == "TIMESTAMP" {
                                    TableValue::String(format_timestamp(&n.to_string()))
                                } else {
                                    TableValue::String(n.to_string())
                                }
                            }
                            serde_json::Value::Bool(b) => TableValue::Boolean(*b),
                            serde_json::Value::Null => TableValue::Null,
                            _ => TableValue::String(v.to_string()),
                        };
                        Some(TableRow { cells: vec![cell] })
                    })
                    .collect()
            } else {
                // Complex repeated field: ARRAY<STRUCT<...>>
                // Each element has the shape {"v": {"f": [...]}}
                arr.iter()
                    .filter_map(|v| v.pointer("/v/f"))
                    .filter(|v| v.is_array())
                    .map(|v| {
                        let mut cells = Vec::new();
                        let mut sub_start = start_col_index;
                        for (i, cell) in v.as_array().unwrap_or(&vec![]).iter().enumerate() {
                            let nf = nested_fields.get(i);
                            let nf_type = nf.map(|f| f.r#type.as_str()).unwrap_or("");
                            let nf_nested = nf.and_then(|f| f.fields.as_deref()).unwrap_or(&[]);
                            let nf_mode = nf.and_then(|f| f.mode.as_deref()).unwrap_or("");
                            let leaf_count = if nf_nested.is_empty() { 1 } else { count_leaf_fields(nf_nested) };
                            cells.extend(flatten_value_to_cells(cell, nf_type, nf_nested, sub_start, nf_mode));
                            sub_start += leaf_count;
                        }
                        TableRow { cells }
                    })
                    .collect()
            };

            let inner_table = InnerTableBuilder {
                style: vscode_nested_style(),
                rows,
                col_span,
                start_col_index,
            };
            TableValue::Array(inner_table)
        }
        serde_json::Value::Object(_obj) => {
            // Non-repeated STRUCTs are handled by flatten_value_to_cells
            // before reaching here. If we get here it's an unexpected object
            // shape — fallback to string representation.
            TableValue::String(v.to_string())
        }
    }
}

impl TableFieldSchema {
    pub(crate) fn to_table_column_definition(&self) -> TableColumnDefinition {
        if let Some(fields) = &self.fields {
            // RECORD (STRUCT) field: emit a Group so sub-field names appear as
            // sub-column headers. The matching row cell is a TableValue::Array
            // with col_span = number of leaf fields, so <td colspan="N"> aligns
            // with the N leaf header columns produced by this Group.
            TableColumnDefinition::Group(TableColumnGroup {
                text: self.name.clone(),
                name: self.name.clone(),
                columns: fields
                    .iter()
                    .map(|field| field.to_table_column_definition())
                    .collect(),
            })
        } else {
            TableColumnDefinition::Column(TableColumn {
                text: self.name.clone(),
                name: self.name.clone(),
                width_px: 100,
            })
        }
    }
}

/// Count the total number of leaf (non-group) columns a field produces,
/// recursing into nested RECORD sub-fields.
fn count_leaf_fields(fields: &[TableFieldSchema]) -> usize {
    fields
        .iter()
        .map(|f| match &f.fields {
            Some(sub) => count_leaf_fields(sub),
            None => 1,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::bigquery::base::TableFieldSchema;
    use crate::bigquery::jobs::GetQueryResultsResponse;
    use serde_json::Value;
    use website_component_table::{TableColumnDefinition, TableValue};

    fn load_query_results(contents: &str) -> GetQueryResultsResponse {
        serde_json::from_str::<GetQueryResultsResponse>(contents).unwrap()
    }

    /// Assert that a slice of flat actual cells matches the expected JSON cell,
    /// consuming leaf cells from `actual_cells` starting at `offset`.
    /// Returns the number of actual cells consumed.
    fn assert_cells_match(
        actual_cells: &[TableValue],
        offset: usize,
        expected_cell: &Value,
        field: &TableFieldSchema,
    ) -> usize {
        let expected_value = expected_cell.pointer("/v").unwrap_or(&Value::Null);
        let mode = field.mode.as_deref().unwrap_or("");
        let nested = field.fields.as_deref().unwrap_or(&[]);

        // Non-repeated STRUCT with sub-fields: was flattened into multiple cells
        if mode != "REPEATED" && !nested.is_empty() {
            if let Some(f_array) = expected_value.pointer("/f").and_then(|f| f.as_array()) {
                let mut consumed = 0;
                for (i, sub_cell) in f_array.iter().enumerate() {
                    if let Some(nf) = nested.get(i) {
                        consumed +=
                            assert_cells_match(actual_cells, offset + consumed, sub_cell, nf);
                    }
                }
                return consumed;
            }
            // NULL struct: each leaf should be Null
            if expected_value.is_null() {
                let leaf_count = super::count_leaf_fields(nested);
                for j in 0..leaf_count {
                    assert!(
                        matches!(actual_cells[offset + j], TableValue::Null),
                        "expected Null for NULL struct leaf at offset {}",
                        offset + j
                    );
                }
                return leaf_count;
            }
        }

        // Single cell comparison
        let actual = &actual_cells[offset];
        match expected_value {
            Value::Null => assert!(
                matches!(actual, TableValue::Null),
                "expected Null at offset {}",
                offset
            ),
            Value::Bool(expected) => match actual {
                TableValue::Boolean(b) => assert_eq!(b, expected),
                _ => panic!("expected boolean at offset {}", offset),
            },
            Value::Number(expected) => match actual {
                TableValue::String(s) => assert_eq!(s, &expected.to_string()),
                _ => panic!("expected numeric string at offset {}", offset),
            },
            Value::String(expected) => match actual {
                TableValue::String(s) => assert_eq!(s, expected),
                _ => panic!("expected string at offset {}", offset),
            },
            Value::Array(expected_rows) => match actual {
                TableValue::Array(actual_table) => {
                    assert_eq!(actual_table.rows.len(), expected_rows.len());
                    if nested.is_empty() {
                        // Simple repeated field (ARRAY<scalar>): each element is {"v": scalar}
                        for (row_idx, row_value) in expected_rows.iter().enumerate() {
                            let actual_row = &actual_table.rows[row_idx];
                            assert_eq!(actual_row.cells.len(), 1, "simple array row should have 1 cell");
                            let expected_str = row_value
                                .pointer("/v")
                                .and_then(|v| v.as_str())
                                .expect("expected simple array element /v to be string");
                            match &actual_row.cells[0] {
                                TableValue::String(s) => assert_eq!(s, expected_str),
                                _ => panic!("expected string cell in simple array row {}", row_idx),
                            }
                        }
                    } else {
                        // ARRAY<STRUCT>: each element is {"v": {"f": [...]}}
                        for (row_idx, row_value) in expected_rows.iter().enumerate() {
                            let expected_inner_cells = row_value
                                .pointer("/v/f")
                                .and_then(|v| v.as_array())
                                .expect("expected nested row to have /v/f array");
                            let actual_row = &actual_table.rows[row_idx];
                            // Inner rows may also have flattened sub-structs
                            let mut inner_offset = 0;
                            for (ci, inner_cell) in expected_inner_cells.iter().enumerate() {
                                if let Some(nf) = nested.get(ci) {
                                    inner_offset += assert_cells_match(
                                        &actual_row.cells,
                                        inner_offset,
                                        inner_cell,
                                        nf,
                                    );
                                }
                            }
                            assert_eq!(
                                inner_offset,
                                actual_row.cells.len(),
                                "inner row cell count mismatch at row {}",
                                row_idx
                            );
                        }
                    }
                }
                _ => panic!("expected array at offset {}", offset),
            },
            Value::Object(_) => match actual {
                TableValue::String(s) => assert_eq!(s, &expected_value.to_string()),
                _ => panic!("expected object-string at offset {}", offset),
            },
        }
        1
    }

    fn assert_table_builder_matches_response(
        response: &GetQueryResultsResponse,
        row_index: usize,
    ) {
        let table_builder = response.to_table_builder(row_index);

        if let Some(schema) = &response.schema {
            // +1 for the leading "#" index column
            assert_eq!(table_builder.columns.len(), schema.fields.len() + 1);

            // The first column is the index column
            match &table_builder.columns[0] {
                TableColumnDefinition::Column(col) => assert_eq!(col.name, "index"),
                _ => panic!("expected index column"),
            }

            // Data columns start at index 1
            table_builder
                .columns
                .iter()
                .skip(1)
                .enumerate()
                .for_each(|(index, col)| {
                    let col_name = schema.fields[index].name.clone();

                    match col {
                        TableColumnDefinition::Group(group) => {
                            assert_eq!(group.text, col_name);
                        }
                        TableColumnDefinition::Column(column) => {
                            assert_eq!(column.text, col_name);
                        }
                    }
                });
        } else {
            assert_eq!(table_builder.columns.len(), 0);
        }

        if let Some(rows) = &response.rows {
            let fields = response
                .schema
                .as_ref()
                .map(|s| s.fields.as_slice())
                .unwrap_or(&[]);

            assert_eq!(table_builder.rows.len(), rows.len());

            rows.iter().enumerate().for_each(|(index, row)| {
                let expected_row_cells = row
                    .pointer("/f")
                    .and_then(|row| row.as_array())
                    .expect("expected row cells array");
                let actual_row = &table_builder.rows[index];

                assert!(matches!(
                    actual_row.cells[0],
                    TableValue::Index(value) if value == row_index + index
                ));

                // Walk through expected cells using the schema-aware matcher
                // that accounts for flattened non-repeated STRUCTs.
                let mut offset = 1; // skip the index cell
                for (cell_index, expected_cell) in expected_row_cells.iter().enumerate() {
                    if let Some(field) = fields.get(cell_index) {
                        offset += assert_cells_match(
                            &actual_row.cells,
                            offset,
                            expected_cell,
                            field,
                        );
                    }
                }
                assert_eq!(
                    offset,
                    actual_row.cells.len(),
                    "row {} cell count mismatch: expected {} actual {}",
                    index,
                    offset,
                    actual_row.cells.len()
                );
            });
        } else {
            assert_eq!(table_builder.rows.len(), 0);
        }
    }

    #[test]
    fn place_bq_table_rows_test_3() {
        let response = load_query_results(include_str!(
            "test_resources/complex_object_array_test3.json"
        ));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_4() {
        let response = load_query_results(include_str!(
            "test_resources/complex_object_array_test4.json"
        ));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_100_rows() {
        let response = load_query_results(include_str!("test_resources/100_rows.json"));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_all_types() {
        let response = load_query_results(include_str!("test_resources/all_types_test.json"));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_complex_object_array() {
        let response = load_query_results(include_str!(
            "test_resources/complex_object_array_test.json"
        ));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_complex_object_array_2() {
        let response = load_query_results(include_str!(
            "test_resources/complex_object_array_test2.json"
        ));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_no_rows() {
        let response = load_query_results(include_str!("test_resources/no_rows.json"));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_simple_array() {
        let response = load_query_results(include_str!("test_resources/simple_array.json"));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_struct_json() {
        let response = load_query_results(include_str!("test_resources/struct_json_test.json"));
        assert_table_builder_matches_response(&response, 1);
    }

    #[test]
    fn place_bq_table_rows_test_complex_nested() {
        let response =
            load_query_results(include_str!("test_resources/complex_nested_test.json"));
        assert_table_builder_matches_response(&response, 1);
    }
}
