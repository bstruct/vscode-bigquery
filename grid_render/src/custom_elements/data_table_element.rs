use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use serde_json::Value;

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
            .append_child("thead", "thead1")
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
                    .apply_fn(&set_text, &text);
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
            for row_index in 0..rows.len() {
                let tr =
                    BaseElement::new_and_append(&self.element(), "tr", &format!("tr{}", row_index));

                for col_index in 0..rows[row_index].len() {
                    let table_item = &rows[row_index][col_index];
                    BaseElement::new_and_append(&tr.element(), "td", &format!("td{}", col_index))
                        .apply_fn(&set_table_item, &table_item);
                }
            }
        }

        self.clone()
    }
}

fn set_text(base_element: &BaseElement, text: &String) {
    base_element.element().set_inner_html(text);
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
