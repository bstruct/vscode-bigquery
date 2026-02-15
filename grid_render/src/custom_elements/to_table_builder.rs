use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup,
    TableRow, TableStyle, TableValue,
};

use crate::bigquery::{base::TableFieldSchema, jobs::GetQueryResultsResponse};

impl GetQueryResultsResponse {
    pub(crate) fn to_table_builder(&self, row_index: usize) -> TableBuilder {
        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: get_columns(&self.schema),
            rows: get_rows(&self.rows, row_index),
        }
    }
}

impl crate::bigquery::tables::Table {
    pub(crate) fn to_table_builder(
        &self,
        rows: &Option<Vec<serde_json::Value>>,
        row_index: usize,
    ) -> TableBuilder {
        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: get_columns(&self.schema),
            rows: get_rows(rows, row_index),
        }
    }
}

fn get_columns(schema: &Option<crate::bigquery::base::TableSchema>) -> Vec<TableColumnDefinition> {
    if let Some(schema) = schema {
        schema
            .fields
            .iter()
            .map(|field| field.to_table_column_definition())
            .collect()
    } else {
        vec![]
    }
}

fn get_rows(rows: &Option<Vec<serde_json::Value>>, row_index: usize) -> Vec<TableRow> {
    if let Some(rows) = rows {
        rows.iter()
            .enumerate()
            .map(|(index, row)| json_value_to_row(row, row_index + index))
            .collect()
    } else {
        vec![]
    }
}

fn json_value_to_row(value: &serde_json::Value, row_index: usize) -> TableRow {
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
            .map(|cell| json_value_to_table_value(cell))
            .collect::<Vec<TableValue>>();
        cells.extend(row_cells);
        cells
    } else {
        vec![]
    };

    TableRow { cells }
}

fn json_value_to_table_value(value: &serde_json::Value) -> TableValue {
    let v = value.pointer("/v").unwrap_or_default();

    match v {
        serde_json::Value::Null => TableValue::Null,
        serde_json::Value::Bool(b) => TableValue::Boolean(b.clone()),
        serde_json::Value::Number(n) => TableValue::String(n.to_string()),
        serde_json::Value::String(s) => TableValue::String(s.clone()),
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
                            .map(|cell| json_value_to_table_value(cell))
                            .collect(),
                    })
                    .collect(),
                col_span: 1,
                start_col_index: 1,
            };
            TableValue::Array(inner_table)
        }
        serde_json::Value::Object(obj) => {
            // let object_values: Vec<(String, TableValue)> = obj.iter()
            //     .map(|(k, v)| (k.clone(), json_value_to_table_value(v)))
            //     .collect();
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
