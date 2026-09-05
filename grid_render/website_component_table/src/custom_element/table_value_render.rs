use website_base::{
    base_result::BaseResult,
    document::{HtmlNode, create_element_with_children, create_element_with_text},
    struct_node::HtmlNodeRender,
    wasm_bindgen::JsCast,
};

impl HtmlNodeRender for crate::TableValue {
    fn render(&self) -> BaseResult<Vec<HtmlNode>> {
        let td = match self {
            crate::TableValue::Null => {
                create_element_with_text("td", Some("null"))?.set_attribute("class", "null")?
            }
            crate::TableValue::Float(f) => create_element_with_text("td", Some(&f.to_string()))?
                .set_attribute("class", "number")?,
            crate::TableValue::Int(i) => create_element_with_text("td", Some(&i.to_string()))?
                .set_attribute("class", "number")?,
            crate::TableValue::Boolean(b) => create_element_with_text("td", Some(&b.to_string()))?
                .set_attribute("class", "boolean")?,
            crate::TableValue::Index(i) => create_element_with_text("td", Some(&i.to_string()))?
                .set_attribute("class", "index")?,
            crate::TableValue::String(s) => get_string_cell(s)?,
            crate::TableValue::Array(array) => get_array_cell(array)?,
        };

        Ok(vec![td])
    }
}

fn get_string_cell(value: &str) -> BaseResult<HtmlNode> {
    create_element_with_text("td", Some(value))?
        .set_attribute("class", "text")?
        .set_mouse_event_listener("mouseleave", td_text_mouse_leave)
}

fn td_text_mouse_leave(event: &website_base::web_sys::MouseEvent) {
    if let Some(target) = event.target()
        && let Some(element) = target.dyn_ref::<website_base::web_sys::Element>() {
            element.scroll_to_with_x_and_y(0.0, 0.0);
        }

    event.stop_propagation();
}

fn get_array_cell(value: &crate::InnerTableBuilder) -> BaseResult<HtmlNode> {
    let rows_len = value.rows.len();

    let content = if rows_len == 1 {
        "(1 row)".to_string()
    } else {
        format!("({} rows)", rows_len)
    };

    let mut children = Vec::with_capacity(1 + rows_len);
    children.push(
        create_element_with_text("div", Some(&content))?.set_attribute("class", "ias")?, // Inner Array Summary
    );
    children.extend(value.render()?);

    create_element_with_children("td", &vec![create_element_with_children("div", &children)?])?
        .set_attribute("class", "array")?
        .set_attribute("colspan", &value.col_span.to_string())
}
