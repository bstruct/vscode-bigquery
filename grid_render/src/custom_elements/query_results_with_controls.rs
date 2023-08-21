// use web_sys::Node;

// use crate::{createElement, web_events::dialog_user_entry_event::DialogUserEntryEvent};

use crate::bigquery::jobs::{GetQueryResultsRequest, Jobs};

use super::custom_element_definition::CustomElementDefinition;
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct QueryResultsWithControls;

impl CustomElementDefinition for QueryResultsWithControls {
    fn define(_document: &web_sys::Document, element: &web_sys::HtmlElement) {
        // element.add_event_listener_with_callback("type_", listener)

        let on_event_type_closure = Closure::wrap(Box::new(
            QueryResultsWithControls::on_render_table,
        ) as Box<dyn Fn(&web_sys::Event)>);
        // form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));

        element
            .add_event_listener_with_callback(
                "render_table",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl QueryResultsWithControls {
    pub fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        let job_id = element.get_attribute("jobId").unwrap();
        let project_id = element.get_attribute("projectId").unwrap();
        let location = element.get_attribute("location").unwrap();

        let jobs = Jobs::new(&"token");
        let request = GetQueryResultsRequest {
            project_id: project_id,
            job_id: job_id,
            start_index: None,
            page_token: None,
            max_results: None,
            timeout_ms: None,
            location: Some(location),
        };

        let _response = jobs.get_query_results(request);

        // let r = crate::getQueryResults(&job_id, &project_id, &location);

        // let max_results = element.get_attribute("xxx").unwrap_or(String::from("1"));

        // let max_results_number = max_results.parse::<u8>().unwrap() + 1;

        // element
        //     .set_attribute("xxx", &max_results_number.to_string())
        //     .unwrap();

        // element.set_inner_text(&format!("xxx: {:?}", r.as_string()));
    }
}
