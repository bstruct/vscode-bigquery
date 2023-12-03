use serde_json::Value;
use web_sys::{HtmlElement, ShadowRoot};

#[derive(Debug, Clone)]
pub(crate) struct TableItem {
    pub is_none: bool,
    pub is_index: bool,
    pub value: Option<String>,
}

impl TableItem {
    pub fn new(is_none: bool, is_main_index: bool, value: Option<String>) -> TableItem {
        TableItem {
            is_none,
            is_index: is_main_index,
            value,
        }
    }
    pub fn from_value(value: &Option<&Value>) -> TableItem {
        let text = match value {
            Some(v) => Some(String::from(v.to_string().clone())),
            None => None,
        };

        TableItem {
            is_none: value.is_some().clone(),
            is_index: false,
            value: text,
        }
    }
}

pub(crate) fn render_table(
    element: &HtmlElement,
    header: &Vec<String>,
    rows: &Vec<Vec<Option<TableItem>>>,
) {
    let shadow = &shadow_init(element);

    // render_control(shadow, query_response, start_index);

    // render_table(shadow, query_response);
}

// fn render_table_v2(
//     element: &HtmlElement,
//     query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
//     start_index: usize,
// ) {
//     if query_response.schema.is_some() {
//         let shadow = &shadow_init(element);

//         render_control(shadow, query_response, start_index);

//         render_table(shadow, query_response);
//     }
// }

fn shadow_init(element: &HtmlElement) -> ShadowRoot {
    let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
    let shadow = element.attach_shadow(&shadow_init).unwrap();

    let shadow_style = crate::createElement("style");
    let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));
    shadow_style.set_inner_html(css_content);
    shadow.append_child(&shadow_style).unwrap();
    shadow
}

// fn render_control(
//     shadow: &ShadowRoot,
//     query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
//     start_index: usize,
// ) {
//     let row_count = query_response.rows.len();
//     let total_rows = parse_to_usize(Some(query_response.total_rows.to_owned())).unwrap_or_default();

//     // control"s background div
//     let div = crate::createElement("div");
//     div.set_id("controls-background");
//     shadow.append_child(&div).unwrap();

//     // control"s div
//     let div = crate::createElement("div");
//     div.set_id("controls");
//     shadow.append_child(&div).unwrap();

//     //span for page information
//     let span_page_information = crate::createElement("span");
//     span_page_information.set_id("paging");
//     span_page_information.set_inner_html(&format!(
//         "{} - {} of {}",
//         start_index + 1,
//         start_index + row_count,
//         total_rows
//     ));
//     div.append_child(&span_page_information).unwrap();

//     //first page
//     let button = crate::createElement("button");
//     button.set_inner_html("<< First page");
//     // button.set_class_name("button");
//     button.set_id("btn_first_page");
//     div.append_child(&button).unwrap();

//     // previous page
//     let button = crate::createElement("button");
//     button.set_inner_html("< Previous page");
//     // button.set_class_name("button");
//     button.set_id("btn_first_page");
//     div.append_child(&button).unwrap();

//     //next page
//     let button = crate::createElement("button");
//     button.set_inner_html("> Next page");
//     // button.set_class_name("button");
//     button.set_id("btn_next_page");
//     div.append_child(&button).unwrap();

//     // last page
//     let button = crate::createElement("button");
//     button.set_inner_html(">> Last page");
//     // button.set_class_name("button");
//     button.set_id("btn_last_page");
//     div.append_child(&button).unwrap();
// }

// fn render_table(
//     shadow: &ShadowRoot,
//     query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
// ) {
//     let fields_schema: &Vec<TableFieldSchema> =
//         &query_response.schema.to_owned().unwrap().fields.to_vec();

//     //<table>
//     let table = crate::createElement("table");
//     shadow.append_child(&table).unwrap();

//     //<thead>
//     let thead = crate::createElement("thead");
//     table.append_child(&thead).unwrap();
//     let tr = crate::createElement("tr");
//     thead.append_child(&tr).unwrap();
//     append_header_columns(&tr, &fields_schema, 1, &None);

//     //<tbody>
//     let tbody = crate::createElement("tbody");
//     table.append_child(&tbody).unwrap();

//     //rows with data
//     let mut row_index = 1;
//     for query_response_row in &query_response.rows {
//         let number_of_rows = calculate_number_of_needed_trs(query_response_row);
//         let mut column_index = 0;

//         for inner_row_index in 0..number_of_rows {
//             let row = crate::createElement("tr");
//             tbody.append_child(&row).unwrap();

//             //main row index
//             if inner_row_index == 0 && column_index == 0 {
//                 row.append_child(&create_cell_with_text(false, &(row_index).to_string()))
//                     .unwrap();
//             }

//             for (field_schema_index, field_schema) in fields_schema.iter().enumerate() {
//                 // console::log_1(&JsValue::from_str(&field_schema.r#type));

//                 column_index = column_index + 1;
//                 // let value = &"xxx"; //query_response_row.get(0).unwrap().as_str().unwrap();
//                 let element =
//                     get_value_element(query_response_row, field_schema_index, field_schema);
//                 row.append_child(&create_cell_with_element(false, column_index, &element))
//                     .unwrap();
//             }
//         }

//         row_index = row_index + 1;
//     }
// }

// fn calculate_number_of_needed_trs(query_response_row: &serde_json::Value) -> u64 {
//     let mut number_of_rows: u64 = 1;

//     let f = query_response_row.pointer("/f");

//     if f.is_some() && f.unwrap().is_array() {
//         let len = f.unwrap().as_array().unwrap().len();
//         for i in 0..len {
//             let value = query_response_row.pointer(&format!("/f/{}/v", i));
//             if value.is_some() && value.unwrap().is_array() {
//                 let count = value.unwrap().as_array().unwrap().len() as u64;
//                 if count > number_of_rows {
//                     number_of_rows = count;
//                 }
//             }
//         }
//     }

//     number_of_rows
// }

// fn calculate_number_of_table_rows_inner_array(query_response_row: &Vec<serde_json::Value>) -> u64 {
//     let mut number_of_rows: u64 = 1;
//     if query_response_row[0].is_array() {
//         let f = query_response_row[0].as_array().unwrap();
//         for item in f {
//             let size = calculate_number_of_table_rows_inner_array(item.as_array().unwrap());
//             if size > 0 {
//                 number_of_rows = max(number_of_rows as f64, size as f64) as u64;
//             }
//         }
//     }

//     number_of_rows
// }

// fn append_header_columns(
//     row: &web_sys::Element,
//     fields_schema: &Vec<TableFieldSchema>,
//     column_index: u8,
//     column_prefix: &Option<String>,
// ) -> u8 {
//     let mut local_column_index: u8 = column_index;

//     if column_index == 1 && column_prefix.is_none() {
//         // "Row" header that contains the row index
//         row.append_child(&create_cell_with_text(true, 1, &"#"))
//             .unwrap();
//         local_column_index = local_column_index + 1;
//     }

//     for field_schema in fields_schema.to_owned() {
//         if field_schema.mode.is_some() && field_schema.mode.unwrap() == "REPEATED" {
//             let column_hashtag = match column_prefix.is_some() {
//                 true => format!("{}{}.#", column_prefix.clone().unwrap(), field_schema.name),
//                 false => format!("{}.#", field_schema.name),
//             };

//             row.append_child(&create_cell_with_text(
//                 true,
//                 local_column_index,
//                 &column_hashtag,
//             ))
//             .unwrap();
//         }

//         if field_schema.r#type == "RECORD" && field_schema.fields.is_some() {
//             let local_column_prefix = match column_prefix.is_some() {
//                 true => format!("{}{}.", column_prefix.clone().unwrap(), field_schema.name),
//                 false => format!("{}.", field_schema.name),
//             };

//             local_column_index = append_header_columns(
//                 row,
//                 &field_schema.fields.to_owned().unwrap(),
//                 local_column_index,
//                 &Some(local_column_prefix),
//             );
//         } else {
//             let column_name = match column_prefix.is_some() {
//                 true => format!("{}{}", column_prefix.clone().unwrap(), field_schema.name),
//                 false => field_schema.name,
//             };

//             row.append_child(&create_cell_with_text(
//                 true,
//                 local_column_index,
//                 &column_name,
//             ))
//             .unwrap();
//             local_column_index = local_column_index + 1;
//         }
//     }

//     local_column_index
// }

// fn get_value_element(
//     query_response_row: &serde_json::Value,
//     field_schema_index: usize,
//     field_schema: &TableFieldSchema,
// ) -> web_sys::HtmlElement {
//     if field_schema.mode.is_some() && field_schema.mode.to_owned().unwrap().eq("REPEATED") {
//         let default_element = crate::createElement("span");
//         default_element
//     } else {
//         // match field_schema.r#type.as_str() {
//         // "DATETIME" | "DATE" | "TIME" | "TIMESTAMP" | "GEOGRAPHY" |
//         // "NUMERIC" | "FLOAT" | "INTEGER" |
//         // "STRING" | "INTERVAL" | "JSON" |
//         // "BYTES"| "BOOLEAN"
//         //     => {
//         let element = crate::createElement("span");

//         let value = query_response_row.pointer(&format!("/f/{}/v", field_schema_index));
//         if value.is_none() || value.unwrap().is_null() {
//             element.set_attribute("class", "nullValue").unwrap();
//             element.set_inner_html(&"null");
//         } else {
//             element.set_text_content(value.unwrap().as_str());
//         }
//         element
//     }
// }

// fn create_cell(column_header: bool) -> web_sys::HtmlElement {
//     match column_header {
//         true => crate::createElement("th"),
//         false => crate::createElement("td"),
//     }
// }

// fn create_cell_with_text(column_header: bool, inner_html: &str) -> web_sys::HtmlElement {
//     let cell = create_cell(column_header);
//     cell.set_inner_html(inner_html);
//     cell
// }

// fn create_cell_with_element(
//     column_header: bool,
//     grid_column: u8,
//     element: &web_sys::Element,
// ) -> web_sys::HtmlElement {
//     let cell = create_cell(column_header, grid_column);
//     cell.append_child(element).unwrap();
//     cell
// }

#[cfg(test)]
mod tests {
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

    // #[wasm_bindgen_test]
    // fn get_value_element_record_and_json() {
    //     let field_schema_index: usize = 1;
    //     let schema = TableFieldSchema {
    //         r#type: "JSON".to_string(),
    //         name: "data".to_string(),
    //         mode: None,
    //         fields: None,
    //         description: None,
    //         max_length: None,
    //         precision: None,
    //         scale: None,
    //         collation: None,
    //         default_value_expression: None,
    //     };

    //     let json_row = r#"
    //     {"f":[{"v":{"f":[{"v":"1"},{"v":"dsdfdsd"}]}},{"v":"{\"Additional_info\":\"\",\"Colour_PDP\":\"\",\"Combi_number\":\"000000867878433060\",\"Lining\":\"\",\"Not_searchable\":false,\"Pim_Value\":\"\",\"Sleeve_Length\":\"\",\"Width_accessoires\":\"\",\"pimExportDate\":\"2022-02-23\",\"row_number\":1}"}]}
    //     "#;

    //     let row: serde_json::Value = serde_json::from_str(json_row).unwrap();
    //     let e1 = &row.pointer("/f/1/v");

    //     let element = table_plot::get_value_element(&row, field_schema_index, &schema);

    //     assert_eq!(
    //         element.text_content().unwrap(),
    //         e1.unwrap().as_str().unwrap()
    //     );
    //     // assert_eq!(element.text_content(), Some(String::from("value")));
    // }

    // #[wasm_bindgen_test]
    // fn append_header_columns_simple() {
    //     let row = crate::createElement("vscode-data-grid-row");

    //     let table_schema = TableSchema {
    //         fields: vec![
    //             TableFieldSchema {
    //                 name: String::from("name1"),
    //                 r#type: String::from("STRING"),
    //                 mode: None,
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //             TableFieldSchema {
    //                 name: String::from("name2"),
    //                 r#type: String::from("JSON"),
    //                 mode: None,
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //         ],
    //     };

    //     table_plot::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

    //     assert!(&row.has_child_nodes());
    //     assert_eq!(&row.child_element_count(), &3);
    //     let child_element = &row.first_child().unwrap();
    //     assert_eq!(child_element.text_content().unwrap(), "#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name1");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2");
    // }

    // #[wasm_bindgen_test]
    // fn append_header_columns_record() {
    //     let row = crate::createElement("vscode-data-grid-row");

    //     let table_schema = TableSchema {
    //         fields: vec![
    //             TableFieldSchema {
    //                 name: String::from("name1"),
    //                 r#type: String::from("STRING"),
    //                 mode: None,
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //             TableFieldSchema {
    //                 name: String::from("name2"),
    //                 r#type: String::from("RECORD"),
    //                 mode: None,
    //                 fields: Some(vec![
    //                     TableFieldSchema {
    //                         name: String::from("a"),
    //                         r#type: String::from("STRING"),
    //                         mode: None,
    //                         fields: None,
    //                         description: None,
    //                         max_length: None,
    //                         precision: None,
    //                         scale: None,
    //                         collation: None,
    //                         default_value_expression: None,
    //                     },
    //                     TableFieldSchema {
    //                         name: String::from("b"),
    //                         r#type: String::from("STRING"),
    //                         mode: None,
    //                         fields: None,
    //                         description: None,
    //                         max_length: None,
    //                         precision: None,
    //                         scale: None,
    //                         collation: None,
    //                         default_value_expression: None,
    //                     },
    //                 ]),
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //         ],
    //     };

    //     table_plot::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

    //     assert!(&row.has_child_nodes());
    //     assert_eq!(&row.child_element_count(), &4);
    //     let child_element = &row.first_child().unwrap();
    //     assert_eq!(child_element.text_content().unwrap(), "#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name1");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.a");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.b");
    //     assert!(child_element.next_sibling().is_none());
    // }

    // #[wasm_bindgen_test]
    // fn append_header_columns_int_array() {
    //     let row = crate::createElement("vscode-data-grid-row");

    //     let table_schema = TableSchema {
    //         fields: vec![
    //             TableFieldSchema {
    //                 name: String::from("name1"),
    //                 r#type: String::from("STRING"),
    //                 mode: None,
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //             TableFieldSchema {
    //                 name: String::from("name2"),
    //                 r#type: String::from("INT64"),
    //                 mode: Some(String::from("REPEATED")),
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //         ],
    //     };

    //     table_plot::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

    //     assert!(&row.has_child_nodes());
    //     assert_eq!(&row.child_element_count(), &4);
    //     let child_element = &row.first_child().unwrap();
    //     assert_eq!(child_element.text_content().unwrap(), "#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name1");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2");
    // }

    // #[wasm_bindgen_test]
    // fn append_header_columns_record_array() {
    //     let row = crate::createElement("vscode-data-grid-row");

    //     let table_schema = TableSchema {
    //         fields: vec![
    //             TableFieldSchema {
    //                 name: String::from("name1"),
    //                 r#type: String::from("STRING"),
    //                 mode: None,
    //                 fields: None,
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //             TableFieldSchema {
    //                 name: String::from("name2"),
    //                 r#type: String::from("RECORD"),
    //                 mode: Some(String::from("REPEATED")),
    //                 fields: Some(vec![
    //                     TableFieldSchema {
    //                         name: String::from("a"),
    //                         r#type: String::from("STRING"),
    //                         mode: None,
    //                         fields: None,
    //                         description: None,
    //                         max_length: None,
    //                         precision: None,
    //                         scale: None,
    //                         collation: None,
    //                         default_value_expression: None,
    //                     },
    //                     TableFieldSchema {
    //                         name: String::from("b"),
    //                         r#type: String::from("STRING"),
    //                         mode: None,
    //                         fields: None,
    //                         description: None,
    //                         max_length: None,
    //                         precision: None,
    //                         scale: None,
    //                         collation: None,
    //                         default_value_expression: None,
    //                     },
    //                 ]),
    //                 description: None,
    //                 max_length: None,
    //                 precision: None,
    //                 scale: None,
    //                 collation: None,
    //                 default_value_expression: None,
    //             },
    //         ],
    //     };

    //     table_plot::append_header_columns(&row, &table_schema.fields.to_owned(), 1, &None);

    //     assert!(&row.has_child_nodes());
    //     assert_eq!(&row.child_element_count(), &5);
    //     let child_element = &row.first_child().unwrap();
    //     assert_eq!(child_element.text_content().unwrap(), "#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name1");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.#");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.a");
    //     let child_element = child_element.next_sibling().unwrap();
    //     assert_eq!(&child_element.text_content().unwrap(), &"name2.b");
    // }

    // #[wasm_bindgen_test]
    // fn complex_object_array_test_1() {
    //     let complex_object_array_test = include_str!("complex_object_array_test.json");
    //     let complex_object_array_test = js_sys::JSON::parse(complex_object_array_test).unwrap();
    //     let complex_object_array_test = &serde_wasm_bindgen::from_value::<
    //         crate::bigquery::jobs::GetQueryResultsResponse,
    //     >(complex_object_array_test)
    //     .unwrap();

    //     let element = &createElement("div");
    //     complex_object_array_test.plot_table(element);

    //     let shadow = element.shadow_root().unwrap();
    //     let child = shadow.first_element_child().unwrap();
    //     assert_eq!(child.tag_name(), "STYLE");

    //     let child = child.next_element_sibling().unwrap();
    //     assert_eq!(child.tag_name(), "DIV");
    //     assert_eq!(child.id(), "controls-background");

    //     let child = child.next_element_sibling().unwrap();
    //     assert_eq!(child.tag_name(), "DIV");
    //     assert_eq!(child.id(), "controls");

    //     let table = child.next_element_sibling().unwrap();
    //     assert_eq!(table.tag_name(), "TABLE");

    //     let mut thead_td = table
    //         .first_element_child()
    //         .unwrap()
    //         .first_element_child()
    //         .unwrap()
    //         .first_element_child()
    //         .unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "#");

    //     let field_names = [
    //         "Pim_Value",
    //         "AttributeValueCategory",
    //         "Colour_PDP",
    //         "Width_accessoires",
    //         "Height_accessoires",
    //         "Lining",
    //         "Shop_by_Sport",
    //         "Not_searchable",
    //         "Exclusive_Access",
    //         "Promo_Activity",
    //         "Additional_info",
    //         "Lifestyle",
    //         "Ranking",
    //         "Product_GBPC",
    //         "TradebyteActive_Combi",
    //         "MainColorPDP",
    //         "Sleeve_Length",
    //         "Padding",
    //         "Soldout",
    //         "Neck_Line",
    //         "Key_Looks",
    //         "pimExportDate",
    //         "ProductGroupCategory",
    //         "Heel_Height",
    //         "USP_flag",
    //         "Material_2",
    //         "Combi_number",
    //         "Flavour_Copy",
    //         "Delete_Flag",
    //         "New_Arrivals",
    //         "Combi_Reference",
    //         "RISE",
    //         "CTP_date",
    //         "STYLE",
    //         "Proper_style_name",
    //         "StyleLength",
    //         "Actual_Online_Date",
    //         "Structure_assignments",
    //         "Functionality",
    //         "Promo_Flag",
    //         "Fit_for_bottoms",
    //         "Sustainable",
    //         "Material_3",
    //         "Fit_for_tops",
    //         "Material",
    //         "Program",
    //         "ImageCount",
    //         "Collection",
    //         "Occasion",
    //         "Shop_by_Activity",
    //         "Brand",
    //         "Length_accessoires",
    //         "Backfill_AboutYou",
    //         "DETAIL",
    //         "row_number",
    //     ];

    //     for i in 0..28 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.value");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.level");

    //     for i in 29..37 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Structure_assignments.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Structure_assignments.assignment");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(
    //         thead_td.inner_html(),
    //         "Structure_assignments.structure_system"
    //     );

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Functionality");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.value");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.country");

    //     for i in 40..55 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     // values

    //     let tbody = table
    //         .first_element_child()
    //         .unwrap()
    //         .next_element_sibling()
    //         .unwrap();
    //     assert_eq!(tbody.tag_name(), "TBODY");

    //     // assert_eq!(thead_td.inner_html(), "Row");
    // }
}
