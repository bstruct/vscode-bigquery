use super::custom_element_definition::CustomElementDefinition;
use crate::bigquery::jobs::{GetQueryResultsRequest, Jobs};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;

pub struct TableWithControls;

impl CustomElementDefinition for TableWithControls {
    fn define(_document: &web_sys::Document, element: &web_sys::HtmlElement) {
        // element.add_event_listener_with_callback("type_", listener)

        let on_event_type_closure = Closure::wrap(
            Box::new(TableWithControls::on_render_table) as Box<dyn Fn(&web_sys::Event)>
        );
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

impl TableWithControls {
    pub fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        //clear out the content
        element.set_inner_html("");

        let job_id = element.get_attribute("jobId").unwrap();
        let project_id = element.get_attribute("projectId").unwrap();
        let location = element.get_attribute("location").unwrap();
        let token = element.get_attribute("token").unwrap();

        let start_index = element.get_attribute("startIndex");
        let max_results = match element.get_attribute("maxResults") {
            Some(s) => {
                let u = s.parse::<u8>();
                if u.is_ok(){
                    Some(u.unwrap())
                }else{
                    None
                }
            },
            None => None,
        };
        // let xxx = element.get_attribute("openInTabVisible").unwrap();

        let jobs = Jobs::new(&token);
        let request = GetQueryResultsRequest {
            project_id: project_id,
            job_id: job_id,
            start_index: start_index,
            page_token: None,
            max_results: max_results,
            timeout_ms: None,
            location: Some(location),
        };

        spawn_local(async move {
            let response = jobs.get_query_results(request).await;
            if response.is_some() {
                crate::custom_elements::table_v2::render_table_v2(&element, &response.unwrap());
            }

            // element.set_inner_text(&format!("xxx: {:?}", response));
        });
    }
}
