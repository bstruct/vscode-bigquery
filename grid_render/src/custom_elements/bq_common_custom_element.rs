use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::parse_to_usize;

pub(crate) fn get_attribute(element: &Element, attribute_name: &str) -> String {
    let att = element.get_attribute(attribute_name);
    assert!(att.is_some(), "attribute not found: {}", attribute_name);
    att.unwrap()
}

pub(crate) fn get_opt_attribute(element: &Element, attribute_name: &str) -> Option<String> {
    element.get_attribute(attribute_name)
}

pub(crate) fn set_attribute(element: &web_sys::Element, attribute_name: &str, value: &str) {
    element.set_attribute(attribute_name, value).unwrap();
}

pub(crate) fn remove_attribute(element: &web_sys::Element, attribute_name: &str) {
    element.remove_attribute(attribute_name).unwrap();
}

pub(crate) fn set_optional_attribute(
    element: &Element,
    attribute_name: &str,
    value: &Option<usize>,
) {
    match value {
        Some(v) => element.set_attribute(attribute_name, &v.to_string()).unwrap(),
        None => element.remove_attribute(attribute_name).unwrap(),
    }
}

pub(crate) fn get_opt_num_attribute(element: &Element, attribute_name: &str) -> Option<usize> {
    parse_to_usize(element.get_attribute(attribute_name))
}

pub(crate) fn get_num_attribute(element: &Element, attribute_name: &str) -> usize {
    match parse_to_usize(element.get_attribute(attribute_name)) {
        Some(num) => num,
        None => panic!("attribute not found: {attribute_name}"),
    }
}

/// Shared handler for pagination button events (first / previous / next / last page).
///
/// `page_op` receives the custom element's DOM `Element`, applies the page change,
/// and returns `true` if the index actually changed (so re-render is triggered).
pub(crate) fn handle_page_nav_event(
    event: &web_sys::Event,
    tag_name: &str,
    page_op: impl Fn(&Element) -> bool,
    render_event_name: &str,
) {
    let element = match event
        .current_target()
        .and_then(|t| t.dyn_into::<Element>().ok())
    {
        Some(e) => e,
        None => {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                "handle_page_nav_event: current_target is not an element",
            ));
            return;
        }
    };

    if element.tag_name() != tag_name.to_uppercase() {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "handle_page_nav_event: unexpected tag '{}', expected '{}'",
            element.tag_name(),
            tag_name.to_uppercase(),
        )));
        return;
    }

    let changed = page_op(&element);
    if changed {
        if let Ok(event) = web_sys::Event::new(render_event_name) {
            let _ = element.dispatch_event(&event);
        }
    }
}
