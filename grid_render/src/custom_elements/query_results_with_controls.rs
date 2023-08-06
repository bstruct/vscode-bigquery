// use web_sys::Node;

// use crate::{createElement, web_events::dialog_user_entry_event::DialogUserEntryEvent};

use super::custom_element_definition::CustomElementDefinition;
// use wasm_bindgen::{prelude::Closure, JsCast};

pub struct QueryResultsWithControls;

impl CustomElementDefinition for QueryResultsWithControls {
    fn define(_document: &web_sys::Document, _element: &web_sys::HtmlElement) {
        // element.on

        // let on_event_type_closure =
        //     Closure::wrap(Box::new(DialogList::on_event_type) as Box<dyn Fn(web_sys::Event)>);
        // // form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));

        // element
        //     .add_event_listener_with_callback(
        //         "interface",
        //         on_event_type_closure.as_ref().unchecked_ref(),
        //     )
        //     .unwrap();

        // on_event_type_closure.forget();
        // element.add_event_listener_with_callback_and_bool(type_, listener, options);
        // element.add_event_listener_with_callback_and_bool_and_wants_untrusted(type_, listener, options, wants_untrusted);

        //input - text
        //let input_text = document
        //    .create_element("input")
        //    .unwrap()
        //    .dyn_into::<web_sys::HtmlInputElement>()
        //    .unwrap();
        //input_text.set_type("text");
        //form.append_child(&input_text).unwrap();
    }
}

// impl DialogList {
//     pub fn on_event_type(element: &web_sys::Element, event: &DialogUserEntryEvent) {
//         let p = createElement("p");
//         p.set_text_content(Some(event.text));
//         element.append_child(&Node::from(p)).unwrap();
//     }
// }