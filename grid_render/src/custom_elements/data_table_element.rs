use serde_json::Value;
use web_sys::ShadowRoot;

// //number of high level rows that
// const PAGE_SIZE: usize = 50;

#[derive(Debug, Clone)]
pub(crate) struct DataTable;

#[derive(Debug, Clone)]
pub(crate) struct DataTableItem {
    pub is_null: bool,
    pub is_index: bool,
    pub value: Option<String>,
}

impl DataTable {
    pub(crate) fn render_table(
        shadow: &ShadowRoot,
        header: &Vec<String>,
        rows: &Vec<Vec<Option<DataTableItem>>>,
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
}

impl DataTableItem {
    pub fn new_main_index(index: usize) -> DataTableItem {
        DataTableItem {
            is_null: false,
            is_index: true,
            value: Some(format!("{}", index)),
        }
    }
   
    pub fn from_value(value: &Option<&Value>) -> DataTableItem {
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

        DataTableItem {
            is_null: !text.is_some().clone(),
            is_index: false,
            value: text,
        }
    }

    pub(crate) fn from_string(domain: &String) -> DataTableItem {
        DataTableItem {
            is_null: false,
            is_index: false,
            value: Some(domain.clone()),
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

fn create_cell(column_header: bool) -> web_sys::Element {
    match column_header {
        true => crate::createElement("th"),
        false => crate::createElement("td"),
    }
}

fn create_cell_with_text(column_header: bool, inner_html: &str) -> web_sys::Element {
    let cell = create_cell(column_header);
    cell.set_inner_html(inner_html);
    cell
}

fn create_cell_with_table_item(table_item: &Option<DataTableItem>) -> web_sys::Element {
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
