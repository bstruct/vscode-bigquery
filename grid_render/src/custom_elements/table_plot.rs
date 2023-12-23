use serde_json::Value;
use web_sys::{HtmlElement, ShadowRoot};

#[derive(Debug, Clone)]
pub(crate) struct TableItem {
    pub is_null: bool,
    pub is_index: bool,
    pub value: Option<String>,
}

impl TableItem {
    pub fn new_main_index(index: usize) -> TableItem {
        TableItem {
            is_null: false,
            is_index: true,
            value: Some(format!("{}", index)),
        }
    }
    pub fn from_value(value: &Option<&Value>) -> TableItem {
        let text = match value {
            Some(v) => {
                if v.is_null() {
                    None
                } else {
                    if v.is_string() {
                        Some(String::from(v.as_str().unwrap()))
                    } else {
                        Some(String::from(v.to_string()))
                    }
                }
            }
            None => None,
        };

        TableItem {
            is_null: !text.is_some().clone(),
            is_index: false,
            value: text,
        }
    }

    pub(crate) fn from_string(domain: &String) -> TableItem {
        TableItem {
            is_null: false,
            is_index: false,
            value: Some(domain.clone()),
        }
    }
}

pub(crate) fn render_table(
    show_controls: bool,
    element: &HtmlElement,
    header: &Vec<String>,
    rows: &Vec<Vec<Option<TableItem>>>,
) {
    // console::log_1(&JsValue::from_str(&"0 - xxxxxx"));

    let shadow = match element.shadow_root() {
        Some(root) => {
            root.remove_child(&root.last_child().unwrap()).unwrap();
            root.remove_child(&root.last_child().unwrap()).unwrap();
            root.remove_child(&root.last_child().unwrap()).unwrap();

            root
        }
        None => shadow_init(element),
    };

    if show_controls {
        let number_of_rows_in_batch = 50;
        let number_of_rows_total = 500;

        render_control(&shadow, number_of_rows_in_batch, number_of_rows_total, 0);
    }

    render_html_table(&shadow, header, rows);
}

fn shadow_init(element: &HtmlElement) -> ShadowRoot {
    let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
    let shadow = element.attach_shadow(&shadow_init).unwrap();

    let shadow_style = crate::createElement("style");
    let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));
    shadow_style.set_inner_html(css_content);
    shadow.append_child(&shadow_style).unwrap();
    shadow
}

fn render_control(
    shadow: &ShadowRoot,
    number_of_rows_in_batch: usize,
    number_of_rows_total: usize,
    start_index: usize,
) {
    // let row_count = number_of_rows_in_batch;
    // let total_rows = crate::parse_to_usize(Some(query_response.total_rows.to_owned())).unwrap_or_default();

    // control"s background div
    let div = crate::createElement("div");
    div.set_id("controls-background");
    shadow.append_child(&div).unwrap();

    // control"s div
    let div = crate::createElement("div");
    div.set_id("controls");
    shadow.append_child(&div).unwrap();

    //span for page information
    let span_page_information = crate::createElement("span");
    span_page_information.set_id("paging");
    span_page_information.set_inner_html(&format!(
        "{} - {} of {}",
        start_index + 1,
        start_index + number_of_rows_in_batch,
        number_of_rows_total
    ));
    div.append_child(&span_page_information).unwrap();

    //first page
    let button = crate::createElement("button");
    button.set_inner_html("<< First page");
    // button.set_class_name("button");
    button.set_id("btn_first_page");
    div.append_child(&button).unwrap();

    // previous page
    let button = crate::createElement("button");
    button.set_inner_html("< Previous page");
    // button.set_class_name("button");
    button.set_id("btn_first_page");
    div.append_child(&button).unwrap();

    //next page
    let button = crate::createElement("button");
    button.set_inner_html("> Next page");
    // button.set_class_name("button");
    button.set_id("btn_next_page");
    div.append_child(&button).unwrap();

    // last page
    let button = crate::createElement("button");
    button.set_inner_html(">> Last page");
    // button.set_class_name("button");
    button.set_id("btn_last_page");
    div.append_child(&button).unwrap();
}

fn render_html_table(
    shadow: &ShadowRoot,
    header: &Vec<String>,
    rows: &Vec<Vec<Option<TableItem>>>,
) {
    //<table>
    let table = crate::createElement("table");
    shadow.append_child(&table).unwrap();

    //<thead>
    let thead = crate::createElement("thead");
    table.append_child(&thead).unwrap();
    let tr = crate::createElement("tr");
    thead.append_child(&tr).unwrap();
    append_header_columns(&tr, &header);

    //<tbody>
    let tbody = crate::createElement("tbody");
    table.append_child(&tbody).unwrap();

    //rows with data
    for row_index in 0..rows.len() {
        let row = &rows[row_index];

        let tr = crate::createElement("tr");
        tbody.append_child(&tr).unwrap();

        for col_index in 0..row.len() {
            let table_item = &row[col_index];
            tr.append_child(&create_cell_with_table_item(table_item))
                .unwrap();
        }
    }
}

fn append_header_columns(row: &web_sys::Element, column_headers: &Vec<String>) {
    for column_header_index in 0..column_headers.len() {
        let column_header = &column_headers[column_header_index];

        row.append_child(&create_cell_with_text(true, column_header))
            .unwrap();
    }
}

fn create_cell(column_header: bool) -> web_sys::HtmlElement {
    match column_header {
        true => crate::createElement("th"),
        false => crate::createElement("td"),
    }
}

fn create_cell_with_text(column_header: bool, inner_html: &str) -> web_sys::HtmlElement {
    let cell = create_cell(column_header);
    cell.set_inner_html(inner_html);
    cell
}

fn create_cell_with_table_item(table_item: &Option<TableItem>) -> web_sys::HtmlElement {
    let cell = create_cell(false);
    if table_item.is_some() {
        let table_item = &table_item.as_ref().unwrap().clone();
        if table_item.is_null {
            cell.set_inner_html("null");
        } else {
            cell.set_inner_html(&table_item.value.as_ref().unwrap());
        }
    }
    cell
}

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
}
