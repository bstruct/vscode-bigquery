use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::{closure::Closure, JsCast};
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
                    .apply_fn(&set_header_text, &text)
                    // .apply_fn(&set_resize_actions, &col_index)
                    ;
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

                for row_index in 0..rows.len() {
                    let tr = BaseElement::new_and_append(
                        &self.element(),
                        "tr",
                        &format!("tr{}", row_index),
                    );

                    for col_index in 0..rows[row_index].len() {
                        let table_item = &rows[row_index][col_index];
                        BaseElement::new_and_append(
                            &tr.element(),
                            "td",
                            &format!("td{}", col_index),
                        )
                        .append_child("div", &format!("d{}", col_index))
                        .apply_fn(&set_table_item, &table_item);
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

fn set_table_event_handlers(base_element: &BaseElement, _x: &Option<usize>) {
    let element = &base_element.element();

    if element.get_attribute("bee").is_none() {
        element.set_attribute("bee", "1").unwrap();

        let on_event_type_closure =
            Closure::wrap(Box::new(on_column_resize_event) as Box<dyn Fn(&web_sys::CustomEvent)>);

        element
            .add_event_listener_with_callback(
                "column_resize",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        on_event_type_closure.forget();
    }
}

fn on_column_resize_event(event: &web_sys::CustomEvent) {
    let element = event
        .current_target()
        .expect("target node not found")
        .dyn_into::<web_sys::Element>()
        .expect("node is not an element");

    let resize_information: ColumnResizeInformation =
        serde_wasm_bindgen::from_value(event.detail()).unwrap();

    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "target tag_name: {:?}",
    //     element.tag_name(),
    // )));

    //style element
    let child_element = element
        .first_child()
        .expect("element with no child element");

    assert_eq!(child_element.node_name(), "STYLE");

    child_element.set_text_content(Some(&format!(
        "tr td:nth-child({}) {{ max-width: {}px; }}",
        resize_information.column_index + 1,
        resize_information.column_width
    )));
}

fn set_header_text(base_element: &BaseElement, text: &String) {
    base_element.element().set_text_content(Some(text));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnResizeInformation {
    column_index: usize,
    column_width: usize,
}

fn set_resize_actions(base_element: &BaseElement, _col_index: &usize) {
    let element = &base_element.element();

    if element.get_attribute("bee").is_none() {
        element.set_attribute("bee", "1").unwrap();

        let on_event_type_closure =
            Closure::wrap(Box::new(on_mouse_event) as Box<dyn Fn(&web_sys::MouseEvent)>);

        element
            .add_event_listener_with_callback(
                "mousemove",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        on_event_type_closure.forget();
    }
}

fn on_mouse_event(mouse_event: &web_sys::MouseEvent) {
    if mouse_event.button() != 0 {
        return;
    }

    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "button: {:?}",
    //     mouse_event.button(),
    // )));

    let element = mouse_event
        .target()
        .expect("target node not found")
        .dyn_into::<web_sys::Element>()
        .expect("node is not an element");

    let child_element = 
        //th
        element.parent_element().expect("element with no parent")
        //tr
        .parent_element().expect("element with no parent")
        //thead
        .parent_element().expect("element with no parent")
        //table
        .parent_element().expect("element with no parent")
        //style
        .first_child().expect("element with no parent")
        ;


    child_element.set_text_content(Some(&format!(
            "tr td:nth-child({}) {{ max-width: {}px; }}",
            3,
            element.client_width()
    )));

    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "client_width: {:?}, parent_width: {:?}",
    //     element.client_width(),
    //     parent_element.client_width(),
    // )));

    // let mut custom_event_init = web_sys::CustomEventInit::new();
    // custom_event_init.bubbles(true);
    // custom_event_init.cancelable(true);
    // custom_event_init.composed(true);
    // let detail = serde_wasm_bindgen::to_value(&ColumnResizeInformation {
    //     column_index: 2,
    //     column_width: element.client_width() as usize,
    // })
    // .unwrap();
    // custom_event_init.detail(&detail);

    // let event = web_sys::CustomEvent::new_with_event_init_dict("column_resize", &custom_event_init)
    //     .unwrap();

    // parent_element.dispatch_event(&event).unwrap();
}

fn set_table_item(base_element: &BaseElement, table_item: &Option<DataTableItem>) {
    if let Some(table_item) = &table_item {
        base_element
            .element()
            .set_inner_html(&table_item.value.as_ref().unwrap_or(&"".to_string()));
    } else {
        base_element.element().set_inner_html("");
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
