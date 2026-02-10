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
            columns: self.get_columns(),
            rows: self.get_rows_new(row_index),
        }
    }

    fn get_columns(&self) -> Vec<TableColumnDefinition> {
        if let Some(schema) = &self.schema {
            schema
                .fields
                .iter()
                .map(|field| field.to_table_column_definition())
                .collect()
        } else {
            vec![]
        }
    }

    fn get_rows_new(&self, row_index: usize) -> Vec<TableRow> {
        if let Some(rows) = &self.rows {
            rows.iter()
                .enumerate()
                .map(|(index, row)| json_value_to_row(row, row_index + index))
                .collect()
        } else {
            vec![]
        }
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
    use website_component_table::{TableColumnDefinition, TableValue};

    #[test]
    fn place_bq_table_rows_test_3() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test3.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let table_builder = complex_object_array_test.to_table_builder(1);

        assert_eq!(table_builder.rows.len(), 1);
        assert_eq!(table_builder.columns.len(), 28);

        table_builder
            .columns
            .iter()
            .enumerate()
            .for_each(|(index, col)| {
                let col_name = if let Some(schema) = &complex_object_array_test.schema {
                    schema.fields.get(index).unwrap().name.clone()
                } else {
                    "".to_string()
                };

                match col {
                    TableColumnDefinition::Group(group) => {
                        assert_eq!(group.text, col_name);
                    }
                    TableColumnDefinition::Column(column) => {
                        assert_eq!(column.text, col_name);
                    }
                }
            });

        let row1 = &table_builder.rows[0];

        assert!(match row1.cells[0].clone() {
            TableValue::Index(index) => {
                index == 1
            }
            _ => false,
        });

        assert!(match row1.cells[1].clone() {
            TableValue::String(s) => {
                s == "1.71965592E9"
            }
            _ => false,
        });

        assert!(match row1.cells[2].clone() {
            TableValue::String(s) => {
                s == "CB"
            }
            _ => false,
        });

        assert!(match row1.cells[3].clone() {
            TableValue::String(s) => {
                s == "DQ2MH67779LAK891"
            }
            _ => false,
        });

        assert!(match row1.cells[4].clone() {
            TableValue::String(s) => {
                s == "4484557887039"
            }
            _ => false,
        });

        assert!(match row1.cells[5].clone() {
            TableValue::String(s) => {
                s == "2003-12-02"
            }
            _ => false,
        });

        assert!(match row1.cells[6].clone() {
            TableValue::Null => {
                true
            }
            _ => false,
        });

        assert!(match row1.cells[7].clone() {
            TableValue::String(s) => {
                s == "5"
            }
            _ => false,
        });

        assert!(match row1.cells[8].clone() {
            TableValue::String(s) => {
                s == "ASIJPI"
            }
            _ => false,
        });

        assert!(match row1.cells[9].clone() {
            TableValue::String(s) => {
                s == "GXBHH"
            }
            _ => false,
        });

        assert!(match row1.cells[10].clone() {
            TableValue::Array(arr) => {
                assert_eq!(arr.rows.len(), 1);
                let row1 = &arr.rows[0];
                assert_eq!(row1.cells.len(), 4);
                assert!(match &row1.cells[0] {
                    TableValue::String(s) => {
                        s == "ggiz"
                    }
                    _ => false,
                });
                assert!(match &row1.cells[1] {
                    TableValue::String(s) => {
                        s == "11"
                    }
                    _ => false,
                });
                assert!(match &row1.cells[2] {
                    TableValue::String(s) => {
                        s == "tc"
                    }
                    _ => false,
                });
                assert!(match &row1.cells[3] {
                    TableValue::Null => {
                        true
                    }
                    _ => false,
                });
            
                true
            }
            _ => false,
        });







    }

    // #[wasm_bindgen_test]
    #[test]
    fn place_bq_table_rows_test_4() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test4.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let table_builder = complex_object_array_test.to_table_builder(1);

        assert_eq!(table_builder.rows.len(), 50);
        assert_eq!(table_builder.columns.len(), 28);

        // // check if for every column that contains "#", the row has the number "1" for that same index.
        // let mut index = 0;
        // for h in header {
        //     if h.contains("#") {
        //         assert!(match rows[0].cells[index] {
        //             TableValue::Index(v) => v == 1,
        //             _ => false,
        //         });
        //     }
        //     index += 1;
        // }
    }
}
