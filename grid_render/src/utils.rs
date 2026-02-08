use crate::custom_elements::base_element::BaseElement;
use web_sys::Element;
use website_component_table::{HtmlNode, HtmlNodeRender, TableBuilder};

pub(crate) fn base_element_append_sibbling_base_element(
    base_element: &BaseElement,
    render_result: &Vec<HtmlNode>,
) {
    let element = base_element.element();
    let parent_element = element.parent_element().unwrap();
    for item in render_result
        .iter()
        .filter_map(|n| n.to_element_node().ok())
    {
        parent_element.append_child(&item).unwrap();
    }
}

pub(crate) fn render_standalone(table_builder: &TableBuilder, parent_node: &Element) {
    let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));

    let parent = BaseElement::new_and_append(parent_node, "div", "st1").append_shadow();
    let parent_node = &parent.node();
    parent.append_child_style(css_content, "style1");

    let elements = &table_builder.render().expect("table render failed");
    for item in elements.iter().filter_map(|n| n.to_element_node().ok()) {
        parent_node.append_child(&item).expect("item not added");
    }
}
