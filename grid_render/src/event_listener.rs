use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, window};

pub fn add_message_listener_to_window() {
    let on_message_received_closure =
        Closure::wrap(Box::new(on_message_received) as Box<dyn Fn(&web_sys::MessageEvent)>);

    window()
        .unwrap()
        .add_event_listener_with_callback(
            "external_message",
            on_message_received_closure.as_ref().unchecked_ref(),
        )
        .unwrap();

    console::log_1(&wasm_bindgen::JsValue::from_str(&"event registered"));

    on_message_received_closure.forget();
}

fn on_message_received(event: &web_sys::MessageEvent) {

    let v = event.to_owned().data();

    console::log_1(&wasm_bindgen::JsValue::from_str(&format!("event: {:?}", v)));
}
