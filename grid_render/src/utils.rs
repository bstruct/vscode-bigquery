use crate::custom_elements::base_element::BaseElement;
use web_sys::Element;
use website_component_table::{HtmlNodeRender, TableBuilder};

pub(crate) fn render_standalone(table_builder: &TableBuilder, parent_node: &Element) {
    let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));

    let parent = BaseElement::new_and_append(parent_node, "div", "st1").append_shadow();
    let parent_node = &parent.node();
    parent.append_child_style(css_content, "style1");

    let elements = match table_builder.render() {
        Ok(els) => els,
        Err(e) => {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "render_standalone: table render failed: {:?}",
                e
            )));
            return;
        }
    };
    for item in elements.iter().filter_map(|n| n.to_element_node().ok()) {
        if let Err(e) = parent_node.append_child(&item) {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "render_standalone: failed to append child: {:?}",
                e
            )));
        }
    }
}
