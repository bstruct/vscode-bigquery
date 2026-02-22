use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup,
    TableRow, TableStyle, TableValue,
};

use crate::bigquery::{base::{TableFieldSchema, TableSchema}, jobs::GetQueryResultsResponse};

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

/// Approximate rendered width of a column header in characters.
fn header_char_len(col: &TableColumnDefinition) -> usize {
    match col {
        TableColumnDefinition::Column(c) => c.text.chars().count(),
        TableColumnDefinition::Group(g) => {
            // Sum child header widths plus small separator gap between them.
            g.columns.iter().map(header_char_len).sum::<usize>()
                + g.columns.len().saturating_sub(1) * 2
        }
    }
}

/// Set `width_px` on every `Column` leaf using the wider of header vs. content.
/// `columns[0]` is the index column and is left at its fixed 50 px.
fn patch_column_widths(columns: &mut Vec<TableColumnDefinition>, rows: &[TableRow]) {
    for (col_idx, col_def) in columns.iter_mut().enumerate() {
        if col_idx == 0 {
            continue; // index column stays fixed
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
                // RECORD fields: size sub-columns from header text only
                // (cell content is a nested inner table, not flat strings).
                patch_group_header_widths(&mut group.columns);
            }
        }
    }
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
            style: TableStyle::default(),
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
            style: TableStyle::default(),
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
        let row_cells = f
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let field_type = fields.get(i).map(|f| f.r#type.as_str()).unwrap_or("");
                let nested_fields = fields.get(i).and_then(|f| f.fields.as_deref()).unwrap_or(&[]);
                json_value_to_table_value(cell, field_type, nested_fields)
            })
            .collect::<Vec<TableValue>>();
        cells.extend(row_cells);
        cells
    } else {
        vec![]
    };

    TableRow { cells }
}

/// Format a BigQuery TIMESTAMP (seconds since Unix epoch, possibly in scientific notation)
/// as an ISO 8601 string using the JS Date API.
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

fn json_value_to_table_value(
    value: &serde_json::Value,
    field_type: &str,
    nested_fields: &[TableFieldSchema],
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
            let inner_table = InnerTableBuilder {
                style: TableStyle::default(),
                rows: arr
                    .iter()
                    .filter_map(|v| v.pointer("/v/f"))
                    .filter(|v| v.is_array())
                    .map(|v| TableRow {
                        cells: v
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                let nf_type = nested_fields.get(i).map(|f| f.r#type.as_str()).unwrap_or("");
                                let nf_nested = nested_fields.get(i).and_then(|f| f.fields.as_deref()).unwrap_or(&[]);
                                json_value_to_table_value(cell, nf_type, nf_nested)
                            })
                            .collect(),
                    })
                    .collect(),
                col_span: 1,
                start_col_index: 1,
            };
            TableValue::Array(inner_table)
        }
        serde_json::Value::Object(_obj) => {
            TableValue::String(value.to_string())
        }
    }
}

impl TableFieldSchema {
    pub(crate) fn to_table_column_definition(&self) -> TableColumnDefinition {
        if let Some(fields) = &self.fields {
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

#[cfg(test)]
mod tests {
    use crate::bigquery::jobs::GetQueryResultsResponse;
    use serde_json::Value;
    use website_component_table::{TableColumnDefinition, TableValue};

    fn load_query_results(contents: &str) -> GetQueryResultsResponse {
        serde_json::from_str::<GetQueryResultsResponse>(contents).unwrap()
    }

    fn assert_table_value_matches_json(actual: &TableValue, expected_cell: &Value) {
        let expected_value = expected_cell.pointer("/v").unwrap_or(&Value::Null);

        match expected_value {
            Value::Null => assert!(matches!(actual, TableValue::Null)),
            Value::Bool(expected) => match actual {
                TableValue::Boolean(actual) => assert_eq!(actual, expected),
                _ => panic!("expected boolean table value"),
            },
            Value::Number(expected) => match actual {
                TableValue::String(actual) => assert_eq!(actual, &expected.to_string()),
                _ => panic!("expected numeric string table value"),
            },
            Value::String(expected) => match actual {
                TableValue::String(actual) => assert_eq!(actual, expected),
                _ => panic!("expected string table value"),
            },
            Value::Array(expected_rows) => match actual {
                TableValue::Array(actual_table) => {
                    assert_eq!(actual_table.rows.len(), expected_rows.len());

                    expected_rows
                        .iter()
                        .enumerate()
                        .for_each(|(row_index, row_value)| {
                            let expected_cells = row_value
                                .pointer("/v/f")
                                .and_then(|v| v.as_array())
                                .expect("expected nested row array");
                            let actual_row = &actual_table.rows[row_index];

                            assert_eq!(actual_row.cells.len(), expected_cells.len());

                            expected_cells.iter().enumerate().for_each(
                                |(cell_index, cell_value)| {
                                    assert_table_value_matches_json(
                                        &actual_row.cells[cell_index],
                                        cell_value,
                                    );
                                },
                            );
                        });
                }
                _ => panic!("expected array table value"),
            },
            Value::Object(_) => match actual {
                TableValue::String(actual) => assert_eq!(actual, &expected_cell.to_string()),
                _ => panic!("expected object string table value"),
            },
        }
    }

    fn assert_table_builder_matches_response(response: &GetQueryResultsResponse, row_index: usize) {
        let table_builder = response.to_table_builder(row_index);

        if let Some(schema) = &response.schema {
            assert_eq!(table_builder.columns.len(), schema.fields.len());

            table_builder
                .columns
                .iter()
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
                assert_eq!(actual_row.cells.len(), expected_row_cells.len() + 1);

                expected_row_cells
                    .iter()
                    .enumerate()
                    .for_each(|(cell_index, expected_cell)| {
                        assert_table_value_matches_json(
                            &actual_row.cells[cell_index + 1],
                            expected_cell,
                        );
                    });
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
}
