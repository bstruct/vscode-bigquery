
fn render_table_v1(
    element: &HtmlElement,
    query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
) {
    if query_response.schema.is_some() {
        let fields_schema: &Vec<TableFieldSchema> =
            &query_response.schema.to_owned().unwrap().fields.to_vec();

        // https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/src/data-grid/README.md
        //<vscode-data-grid>
        let grid = crate::createElement("vscode-data-grid");
        element.append_child(&grid).unwrap();

        //<vscode-data-grid-row>

        let row: web_sys::Element = crate::createElement("vscode-data-grid-row");
        grid.append_child(&row).unwrap();
        append_header_columns(&row, &fields_schema, 1, &None);

        //<vscode-data-grid-cell>

        //rows with data
        let mut row_index = 1;
        for query_response_row in &query_response.rows {
            let row = crate::createElement("vscode-data-grid-row");
            grid.append_child(&row).unwrap();

            let mut column_index = 1;
            row.append_child(&create_cell_with_text(
                false,
                column_index,
                &(row_index).to_string(),
            ))
            .unwrap();

            console::log_1(&JsValue::from_str(&query_response_row.to_string()));

            for (field_schema_index, field_schema) in fields_schema.iter().enumerate() {
                console::log_1(&JsValue::from_str(&field_schema.r#type));

                column_index = column_index + 1;
                // let value = &"xxx"; //query_response_row.get(0).unwrap().as_str().unwrap();
                let element =
                    get_value_element(query_response_row, field_schema_index, field_schema);
                row.append_child(&create_cell_with_element(false, column_index, &element))
                    .unwrap();
            }

            row_index = row_index + 1;
        }
    }
}

fn append_header_columns(
    row: &web_sys::Element,
    fields_schema: &Vec<TableFieldSchema>,
    column_index: u8,
    column_prefix: &Option<String>,
) -> u8 {
    let mut local_column_index: u8 = column_index;

    if column_index == 1 && column_prefix.is_none() {
        // "Row" header that contains the row index
        row.append_child(&create_cell_with_text(true, 1, &"Row"))
            .unwrap();
        local_column_index = local_column_index + 1;
    }

    for field_schema in fields_schema.to_owned() {
        if field_schema.mode.is_some() && field_schema.mode.unwrap() == "REPEATED" {
            let column_hashtag = match column_prefix.is_some() {
                true => format!("{}{}.#", column_prefix.clone().unwrap(), field_schema.name),
                false => format!("{}.#", field_schema.name),
            };

            row.append_child(&create_cell_with_text(
                true,
                local_column_index,
                &column_hashtag,
            ))
            .unwrap();
        }

        if field_schema.r#type == "RECORD" && field_schema.fields.is_some() {
            let local_column_prefix = match column_prefix.is_some() {
                true => format!("{}{}.", column_prefix.clone().unwrap(), field_schema.name),
                false => format!("{}.", field_schema.name),
            };

            local_column_index = append_header_columns(
                row,
                &field_schema.fields.to_owned().unwrap(),
                local_column_index,
                &Some(local_column_prefix),
            );
        } else {
            let column_name = match column_prefix.is_some() {
                true => format!("{}{}", column_prefix.clone().unwrap(), field_schema.name),
                false => field_schema.name,
            };

            row.append_child(&create_cell_with_text(
                true,
                local_column_index,
                &column_name,
            ))
            .unwrap();
            local_column_index = local_column_index + 1;
        }
    }

    local_column_index
}

fn get_value_element(
    query_response_row: &serde_json::Value,
    field_schema_index: usize,
    field_schema: &TableFieldSchema,
) -> web_sys::Element {
    if field_schema.mode.is_some() && field_schema.mode.to_owned().unwrap().eq("REPEATED") {
        let default_element = crate::createElement("span");
        default_element
    } else {
        // match field_schema.r#type.as_str() {
        // "DATETIME" | "DATE" | "TIME" | "TIMESTAMP" | "GEOGRAPHY" |
        // "NUMERIC" | "FLOAT" | "INTEGER" |
        // "STRING" | "INTERVAL" | "JSON" |
        // "BYTES"| "BOOLEAN"
        //     => {
        let element = crate::createElement("span");

        let value = query_response_row.pointer(&format!("/f/{}/v", field_schema_index));
        if value.is_none() || value.unwrap().is_null() {
            element.set_attribute("class", "nullValue").unwrap();
            element.set_inner_html(&"null");
        } else {
            element.set_text_content(value.unwrap().as_str());
        }
        element
    }
}

fn create_cell(column_header: bool, grid_column: u8) -> web_sys::Element {
    let cell = crate::createElement("vscode-data-grid-cell");
    if column_header {
        cell.set_attribute("cell-type", "columnheader").unwrap();
    }
    cell.set_attribute("style", "background-color: var(--list-hover-background);")
        .unwrap();
    cell.set_attribute("grid-column", &grid_column.to_string())
        .unwrap();
    cell
}

fn create_cell_with_text(
    column_header: bool,
    grid_column: u8,
    inner_html: &str,
) -> web_sys::Element {
    let cell = create_cell(column_header, grid_column);
    cell.set_inner_html(inner_html);
    cell
}

fn create_cell_with_element(
    column_header: bool,
    grid_column: u8,
    element: &web_sys::Element,
) -> web_sys::Element {
    let cell = create_cell(column_header, grid_column);
    cell.append_child(element).unwrap();
    cell
}

#[cfg(test)]
mod tests {
    use crate::bigquery::jobs::{TableFieldSchema, TableSchema};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    //* try out some stuff */
    #[test]
    pub fn try_out_serde_json() {
        let json_row = r#"
            {
                "f":[
                    {"v":
                        {"f":[{"v":"1"},{"v":"dsdfdsd"}]}
                    },
                    {"v":
                        "{\"Additional_info\":\"\",\"Colour_PDP\":\"\",\"Combi_number\":\"000000867878433060\",\"Lining\":\"\",\"Not_searchable\":false,\"Pim_Value\":\"\",\"Sleeve_Length\":\"\",\"Width_accessoires\":\"\",\"pimExportDate\":\"2022-02-23\",\"row_number\":1}"
                    }
                ]
            }
        "#;

        let row: serde_json::Value = serde_json::from_str(json_row).unwrap();
        let e1 = &row.pointer("/f/1/v");

        assert!(e1.is_some());
        // println!("value: {:?}", &e1.unwrap().to_string());
        assert_eq!(
            e1.unwrap().to_string(),
            r#""{\"Additional_info\":\"\",\"Colour_PDP\":\"\",\"Combi_number\":\"000000867878433060\",\"Lining\":\"\",\"Not_searchable\":false,\"Pim_Value\":\"\",\"Sleeve_Length\":\"\",\"Width_accessoires\":\"\",\"pimExportDate\":\"2022-02-23\",\"row_number\":1}""#
        );
    }

    #[wasm_bindgen_test]
    fn get_value_element_record_and_json() {
        let field_schema_index: usize = 1;
        let schema = TableFieldSchema {
            r#type: "JSON".to_string(),
            name: "data".to_string(),
            mode: None,
            fields: None,
            description: None,
            max_length: None,
            precision: None,
            scale: None,
            collation: None,
            default_value_expression: None,
        };

        let json_row = r#"
        {"f":[{"v":{"f":[{"v":"1"},{"v":"dsdfdsd"}]}},{"v":"{\"Additional_info\":\"\",\"Colour_PDP\":\"\",\"Combi_number\":\"000000867878433060\",\"Lining\":\"\",\"Not_searchable\":false,\"Pim_Value\":\"\",\"Sleeve_Length\":\"\",\"Width_accessoires\":\"\",\"pimExportDate\":\"2022-02-23\",\"row_number\":1}"}]}
        "#;

        let row: serde_json::Value = serde_json::from_str(json_row).unwrap();
        let e1 = &row.pointer("/f/1/v");

        let element = super::get_value_element(&row, field_schema_index, &schema);

        assert_eq!(
            element.text_content().unwrap(),
            e1.unwrap().as_str().unwrap()
        );
        // assert_eq!(element.text_content(), Some(String::from("value")));
    }

    #[wasm_bindgen_test]
    fn append_header_columns_simple() {
        let row = crate::createElement("vscode-data-grid-row");

        let table_schema = TableSchema {
            fields: vec![
                TableFieldSchema {
                    name: String::from("name1"),
                    r#type: String::from("STRING"),
                    mode: None,
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
                TableFieldSchema {
                    name: String::from("name2"),
                    r#type: String::from("JSON"),
                    mode: None,
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
            ],
        };

        super::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

        assert!(&row.has_child_nodes());
        assert_eq!(&row.child_element_count(), &3);
        let child_element = &row.first_child().unwrap();
        assert_eq!(child_element.text_content().unwrap(), "Row");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name1");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2");
    }

    #[wasm_bindgen_test]
    fn append_header_columns_record() {
        let row = crate::createElement("vscode-data-grid-row");

        let table_schema = TableSchema {
            fields: vec![
                TableFieldSchema {
                    name: String::from("name1"),
                    r#type: String::from("STRING"),
                    mode: None,
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
                TableFieldSchema {
                    name: String::from("name2"),
                    r#type: String::from("RECORD"),
                    mode: None,
                    fields: Some(vec![
                        TableFieldSchema {
                            name: String::from("a"),
                            r#type: String::from("STRING"),
                            mode: None,
                            fields: None,
                            description: None,
                            max_length: None,
                            precision: None,
                            scale: None,
                            collation: None,
                            default_value_expression: None,
                        },
                        TableFieldSchema {
                            name: String::from("b"),
                            r#type: String::from("STRING"),
                            mode: None,
                            fields: None,
                            description: None,
                            max_length: None,
                            precision: None,
                            scale: None,
                            collation: None,
                            default_value_expression: None,
                        },
                    ]),
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
            ],
        };

        super::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

        assert!(&row.has_child_nodes());
        assert_eq!(&row.child_element_count(), &4);
        let child_element = &row.first_child().unwrap();
        assert_eq!(child_element.text_content().unwrap(), "Row");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name1");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.a");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.b");
        assert!(child_element.next_sibling().is_none());
    }

    #[wasm_bindgen_test]
    fn append_header_columns_int_array() {
        let row = crate::createElement("vscode-data-grid-row");

        let table_schema = TableSchema {
            fields: vec![
                TableFieldSchema {
                    name: String::from("name1"),
                    r#type: String::from("STRING"),
                    mode: None,
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
                TableFieldSchema {
                    name: String::from("name2"),
                    r#type: String::from("INT64"),
                    mode: Some(String::from("REPEATED")),
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
            ],
        };

        super::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

        assert!(&row.has_child_nodes());
        assert_eq!(&row.child_element_count(), &4);
        let child_element = &row.first_child().unwrap();
        assert_eq!(child_element.text_content().unwrap(), "Row");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name1");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.#");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2");
    }

    #[wasm_bindgen_test]
    fn append_header_columns_record_array() {
        let row = crate::createElement("vscode-data-grid-row");

        let table_schema = TableSchema {
            fields: vec![
                TableFieldSchema {
                    name: String::from("name1"),
                    r#type: String::from("STRING"),
                    mode: None,
                    fields: None,
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
                TableFieldSchema {
                    name: String::from("name2"),
                    r#type: String::from("RECORD"),
                    mode: Some(String::from("REPEATED")),
                    fields: Some(vec![
                        TableFieldSchema {
                            name: String::from("a"),
                            r#type: String::from("STRING"),
                            mode: None,
                            fields: None,
                            description: None,
                            max_length: None,
                            precision: None,
                            scale: None,
                            collation: None,
                            default_value_expression: None,
                        },
                        TableFieldSchema {
                            name: String::from("b"),
                            r#type: String::from("STRING"),
                            mode: None,
                            fields: None,
                            description: None,
                            max_length: None,
                            precision: None,
                            scale: None,
                            collation: None,
                            default_value_expression: None,
                        },
                    ]),
                    description: None,
                    max_length: None,
                    precision: None,
                    scale: None,
                    collation: None,
                    default_value_expression: None,
                },
            ],
        };

        super::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

        assert!(&row.has_child_nodes());
        assert_eq!(&row.child_element_count(), &5);
        let child_element = &row.first_child().unwrap();
        assert_eq!(child_element.text_content().unwrap(), "Row");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name1");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.#");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.a");
        let child_element = child_element.next_sibling().unwrap();
        assert_eq!(&child_element.text_content().unwrap(), &"name2.b");
    }
}
