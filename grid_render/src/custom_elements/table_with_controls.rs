// use crate::web_events::dialog_user_entry_event::DialogUserEntryEvent;
// use crate::web_events::WebEvents::DialogUserEntry;

use super::custom_element_definition::CustomElementDefinition;
// use wasm_bindgen::{prelude::Closure, JsCast};

pub struct TableWithControls;

impl CustomElementDefinition for TableWithControls {
    fn define(_document: &web_sys::Document, _element: &web_sys::HtmlElement) {

        // element.set_inner_text(&"hi");

        // //form
        // let form = document
        //     .create_element("form")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlFormElement>()
        //     .unwrap();

        // let onsubmit_closure = Closure::wrap(
        //     Box::new(DialogInput::form_onsubmit) as Box<dyn Fn(web_sys::SubmitEvent) -> bool>
        // );
        // form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));
        // onsubmit_closure.forget();

        // element.append_child(&form).unwrap();

        // //input - text
        // let input_text = document
        //     .create_element("input")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlInputElement>()
        //     .unwrap();
        // input_text.set_type("text");
        // form.append_child(&input_text).unwrap();

        // //input - submit
        // let input_submit = document
        //     .create_element("input")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlInputElement>()
        //     .unwrap();
        // input_submit.set_type("submit");
        // form.append_child(&input_submit).unwrap();
    }
}

// impl DialogInput {
//     fn form_onsubmit(event: web_sys::SubmitEvent) -> bool {
//         let text_input_2 = event
//             .submitter()
//             .unwrap()
//             .parent_element()
//             .unwrap()
//             .query_selector(&"input[type=text]")
//             .expect("error 1")
//             .expect("error 2")
//             .dyn_into::<web_sys::HtmlInputElement>()
//             .expect("error 3");

//         let text = text_input_2.value();
//         text_input_2.set_value(&"");

//         web_sys::console::warn_1(&wasm_bindgen::JsValue::from(&text));

//         //trigger_event
//         // let document = &event.submitter().unwrap().owner_document().unwrap();

//         let dialog_entry_event = DialogUserEntryEvent {
//             text: &String::from(&text),
//         };
//         DialogUserEntry.trigger_event(&dialog_entry_event);
//         web_sys::console::warn_1(&wasm_bindgen::JsValue::from("event triggered"));

//         event.set_cancel_bubble(true);

//         false
//     }
// }
