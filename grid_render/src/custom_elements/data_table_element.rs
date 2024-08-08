use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use serde_json::Value;
use web_sys::Element;

#[derive(Debug)]
pub(crate) struct DataTable {
    element_id: String,
    header: Option<Vec<String>>,
    rows: Option<Vec<Vec<Option<DataTableItem>>>>,
}

impl DataTable {
    pub(crate) fn new(
        element_id: &str,
        header: &Option<Vec<String>>,
        rows: &Option<Vec<Vec<Option<DataTableItem>>>>,
    ) -> DataTable {
        DataTable {
            element_id: element_id.to_string(),
            header: header.to_owned(),
            rows: rows.to_owned(),
        }
    }

    pub(crate) fn render_standalone(&self, parent_node: &web_sys::Node) -> BaseElement {
        let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));

        BaseElement::new_and_append(parent_node, "div", "st1")
            .append_shadow()
            .append_child_style(css_content, "style1")
            .append_sibling_base_element(self)
    }
}

impl BaseElementTrait for DataTable {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        if self.rows.is_some() {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                "====== render table ========, row_count: {}",
                &self.rows.as_ref().unwrap().len()
            )));
        } else {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                "====== render table ========, rows: None"
            )));
        }

        BaseElement::new_and_append(parent_node, "table", &self.get_element_id())
            // .apply_fn(&set_table_event_handlers, &None)
            .append_child("style", "tablestyle")
            .append_sibling("thead", "thead1")
            .resolve_header(&self.header)
            .append_sibling("tbody", "tbody1")
            .resolve_rows(&self.rows)
    }
}

impl BaseElement {
    fn resolve_header(&self, header: &Option<Vec<String>>) -> BaseElement {
        if let Some(header) = header {
            let tr = BaseElement::new_and_append(&self.element(), "tr", "tr1");

            for col_index in 0..header.len() {
                let text = &header[col_index];
                BaseElement::new_and_append(&tr.element(), "th", &format!("th{}", col_index))
                    .append_child("div", &format!("d{}", col_index))
                    .apply_fn(&set_header_text, &text);
            }
        } else {
            while let Some(child) = self.first_child() {
                self.remove_child(&child);
            }
        }

        self.clone()
    }

    fn resolve_rows(&self, rows: &Option<Vec<Vec<Option<DataTableItem>>>>) -> BaseElement {
        if let Some(rows) = rows {
            if rows.len() == 0 {
                while let Some(child) = self.first_child() {
                    self.remove_child(&child);
                }
            } else {
                let mut last_pointer_tr: Option<Element> = None;

                let element = &self.element();

                for row_index in 0..rows.len() {
                    let tr =
                        BaseElement::new_and_append(element, "tr", &format!("tr{}", row_index));

                    for col_index in 0..rows[row_index].len() {
                        let table_item = &rows[row_index][col_index];
                        BaseElement::new_and_append(
                            &tr.element(),
                            "td",
                            &format!("td{}", col_index),
                        )
                        .append_child("div", &format!("d{}", col_index))
                        .apply_fn(&set_table_item, &table_item)
                        .apply_fn(&check_row_div_width, &None);
                    }

                    last_pointer_tr = Some(tr.element());
                }

                if last_pointer_tr.is_some() {
                    let last_pointer_tr =
                        BaseElement::from_element(&last_pointer_tr.as_ref().unwrap());
                    while let Some(next_sibling) = last_pointer_tr.next_sibling() {
                        self.remove_child(&next_sibling);
                    }
                }
            }
        }

        self.clone()
    }
}

fn check_row_div_width(base_element: &BaseElement, _x: &Option<usize>) {
    let element = &base_element.element();
    // let parent_element = element.parent_element().unwrap();

    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "width: {:?}, parent_width: {:?}",
    //     element.client_width(),
    //     parent_element.client_width(),
    // )));

    if element.client_width() >= 600 {
        element.set_class_name("resize");
    }
}

fn set_header_text(base_element: &BaseElement, text: &String) {
    base_element.element().set_text_content(Some(text));
}

fn set_table_item(base_element: &BaseElement, table_item: &Option<DataTableItem>) {
    let element = &base_element.element();
    let td = element.parent_element().unwrap();
    if let Some(table_item) = &table_item {
        if table_item.is_index {
            td.set_class_name("index");
        } else {
            td.set_class_name("v");
        }
        if table_item.is_null {
            element.set_text_content(Some(&"null".to_string()));
            element.set_class_name("nullValue");
        } else {
            base_element
                .element()
                .set_text_content(Some(&table_item.value.as_ref().unwrap_or(&"".to_string())));
        }
    } else {
        // let e = base_element.element();
        td.set_class_name("");
        element.set_inner_html("");
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DataTableItem {
    pub is_null: bool,
    pub is_index: bool,
    pub value: Option<String>,
}

impl DataTableItem {
    pub fn new_main_index(index: usize) -> DataTableItem {
        DataTableItem {
            is_null: false,
            is_index: true,
            value: Some(format!("{}", index)),
        }
    }

    pub fn from_value(value: &Option<Value>) -> DataTableItem {
        let text = match value {
            Some(v) => {
                if v.is_null() {
                    None
                } else {
                    Some(String::from(v.as_str().unwrap_or_default()))
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

    pub(crate) fn from_string(v: &String) -> DataTableItem {
        DataTableItem {
            is_null: false,
            is_index: false,
            value: Some(v.clone()),
        }
    }
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
