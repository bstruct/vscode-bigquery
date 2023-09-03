use super::custom_element_definition::CustomElementDefinition;
use crate::bigquery::jobs::{GetQueryResultsRequest, Jobs, TableFieldSchema};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlElement};

pub struct QueryResultsWithControls;

impl CustomElementDefinition for QueryResultsWithControls {
    fn define(_document: &web_sys::Document, element: &web_sys::HtmlElement) {
        // element.add_event_listener_with_callback("type_", listener)

        let on_event_type_closure = Closure::wrap(Box::new(
            QueryResultsWithControls::on_render_table,
        ) as Box<dyn Fn(&web_sys::Event)>);
        // form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));

        element
            .add_event_listener_with_callback(
                "render_table",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl QueryResultsWithControls {
    pub fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        //clear out the content
        element.set_inner_html("");

        let job_id = element.get_attribute("jobId").unwrap();
        let project_id = element.get_attribute("projectId").unwrap();
        let location = element.get_attribute("location").unwrap();
        let token = element.get_attribute("token").unwrap();

        let jobs = Jobs::new(&token);
        let request = GetQueryResultsRequest {
            project_id: project_id,
            job_id: job_id,
            start_index: None,
            page_token: None,
            max_results: None,
            timeout_ms: None,
            location: Some(location),
        };

        spawn_local(async move {
            let response = jobs.get_query_results(request).await;
            if response.is_some() {
                render_table(&element, &response.unwrap());
            }

            // element.set_inner_text(&format!("xxx: {:?}", response));
        });
    }
}

fn render_table(
    element: &HtmlElement,
    query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
) {
    // https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/src/data-grid/README.md
    //<vscode-data-grid>
    let grid = crate::createElement("vscode-data-grid");
    element.append_child(&grid).unwrap();

    //<vscode-data-grid-row>
    let row = crate::createElement("vscode-data-grid-row");
    grid.append_child(&row).unwrap();

    //<vscode-data-grid-cell>

    // "Row" header that contains the row index
    let mut column_index = 1;
    row.append_child(&create_cell_with_text(true, column_index, &"Row"))
        .unwrap();
    column_index = column_index + 1;

    //top level column names
    let fields = &query_response.schema.to_owned().unwrap().fields.to_vec();
    for column in fields {
        row.append_child(&create_cell_with_text(true, column_index, &column.name))
            .unwrap();
        column_index = column_index + 1;
    }

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

        for (field_schema_index, field_schema) in fields.iter().enumerate() {
            console::log_1(&JsValue::from_str(&field_schema.r#type));

            column_index = column_index + 1;
            // let value = &"xxx"; //query_response_row.get(0).unwrap().as_str().unwrap();
            let element = get_value_element(query_response_row, field_schema_index, field_schema);
            row.append_child(&create_cell_with_element(false, column_index, &element))
                .unwrap();
        }

        row_index = row_index + 1;
    }
}

fn get_value_element(
    query_response_row: &serde_json::Value,
    field_schema_index: usize,
    field_schema: &TableFieldSchema,
) -> web_sys::Element {
    match field_schema.r#type.as_str() {
        "DATETIME" | "DATE" | "TIME" | "TIMESTAMP" | "GEOGRAPHY" |
        "NUMERIC" | "FLOAT" | "INTEGER" | 
        "STRING" | "INTERVAL" | "JSON"
        | "BYTES"| "BOOLEAN"
        => {
            let default_element = crate::createElement("span");

            let value = query_response_row.pointer(&format!("/f/{}/v", field_schema_index));
            if value.is_none() || value.unwrap().is_null() {
                default_element.set_attribute("class", "nullValue").unwrap();
                default_element.set_inner_html(&"null");
            } else {
                default_element.set_text_content(value.unwrap().as_str());
            }
            default_element
        }
        _ => {
            let default_element = crate::createElement("span");
            default_element.set_text_content(Some("value"));
            default_element
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bigquery::jobs::TableFieldSchema;
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
            r#type: "JSONx".to_string(),
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

        let _element = super::get_value_element(&row, field_schema_index, &schema);

        // assert_eq!(element.text_content(), Some(row[1].as_str().unwrap().to_string()));
        // assert_eq!(element.text_content(), Some(String::from("value")));
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

fn create_cell_with_text(column_header: bool, grid_column: u8, inner_html: &str) -> web_sys::Element {
    let cell = create_cell(column_header, grid_column);
    cell.set_inner_html(inner_html);
    cell
}

fn create_cell_with_element(column_header: bool, grid_column: u8, element: &web_sys::Element) -> web_sys::Element {
    let cell = create_cell(column_header, grid_column);
    cell.append_child(element).unwrap();
    cell
}
