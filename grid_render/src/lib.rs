mod bigquery;
mod custom_elements;
mod external_request;
mod message_handler;
pub(crate) mod utils;

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
    fn createElement(tagName: &str) -> web_sys::Element;
    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(elementId: &str) -> Option<Element>;

    // //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
    // #[wasm_bindgen(js_namespace = document, js_name = "observeElement")]
    // fn observe_element(element: &Element);
    #[wasm_bindgen(js_namespace = document, js_name = "setState", catch)]
    fn set_state(state_json: &str) -> Result<(), JsValue>;
}

#[wasm_bindgen]
pub fn get_web_components_list() -> Vec<JsValue> {
    CustomElement::get_all()
        .iter()
        .map(|f| JsValue::from_str(&f.to_string()))
        .collect()
}

#[wasm_bindgen]
pub fn register_custom_element(custom_component_name: &JsValue, element: web_sys::Element) {
    let custom_component_string = match custom_component_name.as_string() {
        Some(s) => s,
        None => {
            web_sys::console::error_1(&JsValue::from_str(
                "register_custom_element: custom_component name not provided",
            ));
            return;
        }
    };

    let custom_component = match CustomElement::from_str(&custom_component_string) {
        Ok(c) => c,
        Err(_) => {
            web_sys::console::error_1(&JsValue::from_str(&format!(
                "register_custom_element: unknown component '{}'",
                custom_component_string
            )));
            return;
        }
    };

    if let Err(e) = custom_component.define_custom_component(&element) {
        web_sys::console::error_1(&JsValue::from_str(&format!(
            "register_custom_element: failed to configure component '{}': {:?}",
            custom_component_string, e
        )));
    }
}

#[wasm_bindgen]
pub async fn on_window_message_received(event: &web_sys::MessageEvent) {
    message_handler::handle(event).await;
}

fn parse_to_usize(number: Option<String>) -> Option<usize> {
    number?.parse::<usize>().ok()
}

#[wasm_bindgen(start)]
fn main_js() {}
