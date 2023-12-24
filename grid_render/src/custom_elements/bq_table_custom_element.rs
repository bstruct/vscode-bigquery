use super::custom_element_definition::CustomElementDefinition;
use crate::{
    bigquery::jobs::{GetQueryResultsRequest, Job, Jobs},
    parse_to_usize, observe,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;

pub struct BigqueryTableCustomElement;

impl CustomElementDefinition for BigqueryTableCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::HtmlElement) {
        // element.add_event_listener_with_callback("type_", listener)

        let on_event_type_closure =
            Closure::wrap(Box::new(BigqueryTableCustomElement::on_render_table)
                as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                "visible",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl BigqueryTableCustomElement {
    pub(crate) fn from_job(token: &String, job: &Job) -> web_sys::HtmlElement {
        let job_refeerence = job.job_reference.as_ref().expect("job reference not found");

        BigqueryTableCustomElement::new(
            &job_refeerence.job_id,
            &job_refeerence.project_id,
            &job_refeerence.location,
            token,
            &Some(0),
            &Some(50),
        )
    }

    fn new(
        job_id: &String,
        project_id: &String,
        location: &String,
        token: &String,
        start_index: &Option<usize>,
        max_results: &Option<usize>,
    ) -> web_sys::HtmlElement {
        let bq_table_custom_element =
            crate::createElement(&super::CustomElement::BqTable.to_string());

        bq_table_custom_element.set_attribute("jobId", job_id).unwrap();
        bq_table_custom_element.set_attribute("projectId", project_id).unwrap();
        bq_table_custom_element.set_attribute("location", location).unwrap();
        bq_table_custom_element.set_attribute("token", token).unwrap();
        if start_index.is_some() {
            bq_table_custom_element.set_attribute("startIndex", &start_index.unwrap().to_string()).unwrap();
        }
        if max_results.is_some() {
            bq_table_custom_element.set_attribute("maxResults", &max_results.unwrap().to_string()).unwrap();
        }

        let id = "test-1";
        bq_table_custom_element.set_id(id);
        //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
        // observe(id);

        bq_table_custom_element
    }

    fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        //clear out the content
        element.set_inner_html("hi");

        // let job_id = element.get_attribute("jobId").unwrap();
        // let project_id = element.get_attribute("projectId").unwrap();
        // let location = element.get_attribute("location").unwrap();
        // let token = element.get_attribute("token").unwrap();

        // let start_index = element.get_attribute("startIndex");
        // let max_results = parse_to_usize(element.get_attribute("maxResults"));
        // // let xxx = element.get_attribute("openInTabVisible").unwrap();

        // let jobs = Jobs::new(&token);
        // let request = GetQueryResultsRequest {
        //     project_id: project_id,
        //     job_id: job_id,
        //     start_index: start_index.to_owned(),
        //     page_token: None,
        //     max_results: max_results,
        //     timeout_ms: None,
        //     location: Some(location),
        // };

        // element.set_inner_text(&format!("xxx: {:?}", request));

        // spawn_local(async move {
        //     let response = jobs.get_query_results(request).await;
        //     // if response.is_some() {
        //     //     let start_index = parse_to_usize(start_index).unwrap_or_default();

        //     //     crate::custom_elements::table_v2::render_table_v2(&element, &response.unwrap(), start_index);
        //     // }

        //     element.set_inner_text(&format!("xxx: {:?}", response));
        // });
    }
}
