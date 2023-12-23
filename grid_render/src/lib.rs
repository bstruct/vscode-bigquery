mod bigquery;
mod custom_elements;
mod message_handler;
mod external_request;

use custom_elements::CustomElement;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::Element;

// cfg_if! {
//     if #[cfg(feature = "wee_alloc")] {
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn getElementsByTagName(qualifiedName: &str) -> Vec<Element>;
    #[wasm_bindgen(js_namespace = document)]
    fn createElement(tagName: &str) -> web_sys::HtmlElement;
    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(elementId: &str) -> Option<Element>;
}

#[wasm_bindgen]
pub fn get_web_components_list() -> Vec<JsValue> {
    CustomElement::get_all()
        .iter()
        .map(|f| JsValue::from_str(&f.to_string()))
        .collect()
}

#[wasm_bindgen]
pub fn register_custom_element(custom_component_name: &JsValue, element: web_sys::HtmlElement) {
    let custom_component_string = custom_component_name
        .as_string()
        .expect("custom_component name not provided");

    let custom_component =
        CustomElement::from_str(&custom_component_string).expect("custom_component not found");

    custom_component
        .define_custom_component(&element)
        .expect("custom_component not configured");
}

#[wasm_bindgen]
pub async fn on_window_message_received(event: &web_sys::MessageEvent) {
    message_handler::handle(event).await;
}

fn parse_to_usize(number: Option<String>) -> Option<usize> {
    match number {
        Some(s) => {
            let u = s.parse::<usize>();
            if u.is_ok() {
                Some(u.unwrap())
            } else {
                None
            }
        }
        None => None,
    }
}

#[wasm_bindgen(start)]
fn main_js() {}
