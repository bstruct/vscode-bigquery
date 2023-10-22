use serde::Deserialize;

use crate::getElementById;

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    pub token: String,
    pub query: String,
}

pub fn handle(event: &web_sys::MessageEvent) {
    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "event {}: {:?}",
    //     event.type_(),
    //     event.data()
    // )));

    let data = event.data();

    let p = &serde_wasm_bindgen::from_value::<ExternalRequest>(data).unwrap();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "parsed: {}: {}",
        p.request_type,
        p.query
    )));

    let q1 = getElementById("q1");
    if q1.is_some(){
        q1.unwrap().set_inner_html(&p.query);
    }
}