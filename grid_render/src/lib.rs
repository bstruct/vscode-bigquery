mod custom_elements;

use custom_elements::CustomElement;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::{Element, console};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = document)]
    fn getElementsByTagName(qualifiedName: &str) -> Vec<Element>;
    #[wasm_bindgen(js_namespace = document)]
    fn createElement(tagName: &str) -> Element;
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

#[wasm_bindgen(start)]
fn main() {

    console::info_1(&JsValue::from("I'm in"));

    // let window = web_sys::window().expect("no window exists");
    // let document = window.document().expect("window should have a document");

    //to be explored later
    // let observer = web_sys::MutationObserver::new()
    //     .expect("failed to create MutationObserver")
    // ;
    // observer.observe(&document.first_child().unwrap()).unwrap();
    // observer.take_records();
    // observer.disconnect();

    // //sidebar
    // let sidebar = document
    //     .get_element_by_id(&"sidebar")
    //     .expect("element id 'sidebar' was not found");

    // let p1 = document
    //     .create_element("p")
    //     .expect("error when creating element");
    // p1.set_text_content(Some(WELCOME_MESSAGE));
    // sidebar.append_child(&p1).unwrap();

    // let dialog_list = document
    //     .create_element(&custom_elements::CustomElement::DialogList.to_string())
    //     .unwrap();
    // sidebar.append_child(&dialog_list).unwrap();

    // let dialog_input = document
    //     .create_element(&custom_elements::CustomElement::DialogInput.to_string())
    //     .unwrap();
    // sidebar.append_child(&dialog_input).unwrap();

    // //main_content
    // let main_content = document
    //     .get_element_by_id(&"main_content")
    //     .expect("element id 'main_content' was not found");

    // main_content.set_inner_html(&"<div>
    //         <article>
    //             <p>This is an article of content that sits inline with a sidebar. Resize the browser to see how when
    //                 there's no enough room, the sidebar will stack on to a new line.</p>
    //         </article>
    //     </div>
    //     <div>
    //         <navigation-item></navigation-item>
    //     </div>");
}
