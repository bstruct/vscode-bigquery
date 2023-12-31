use std::{borrow::Borrow, ops::Deref};

use super::{
    base_element_trait::BaseElementTrait, custom_element_definition::CustomElementDefinition,
};
use crate::{
    // bigquery::jobs::{GetQueryResultsRequest, Job, Jobs},
    custom_elements::{base_element::BaseElement, CustomElement},
    // parse_to_usize,
};
use wasm_bindgen::{prelude::Closure, JsCast};
// use wasm_bindgen_futures::spawn_local;
// use web_sys::Element;

const TAG_NAME: &'static str = "bq-table";

pub(crate) struct BigqueryTableCustomElementSettings {
    job_id: String,
    project_id: String,
    location: String,
    token: String,

    page_start_index: Option<usize>,
    page_size: usize,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,
    // element_id: String,
    // element: Box<Element>,
}

impl BigqueryTableCustomElementSettings {
    pub(crate) fn base_new(
        job_id: String,
        project_id: String,
        location: String,
        token: String,
    ) -> BigqueryTableCustomElementSettings {
        BigqueryTableCustomElementSettings {
            job_id,
            project_id,
            location,
            token,

            page_start_index: None,
            page_size: 50,
            rows_in_page: None,
            rows_total: None,
            // element_id: String,
            // element: Box<Element>,
        }
    }
}

pub(crate) struct BigqueryTableCustomElement {
    // job_id: String,
    // project_id: String,
    // location: String,
    // token: String,

    // page_start_index: Option<usize>,
    // page_size: usize,
    // rows_in_page: Option<usize>,
    // rows_total: Option<usize>,
    element_id: String,
    settings: Box<Option<BigqueryTableCustomElementSettings>>,
    // element: Box<Element>,
}

impl CustomElementDefinition for BigqueryTableCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::Element) {
        let on_event_type_closure =
            Closure::wrap(Box::new(BigqueryTableCustomElement::on_render_table)
                as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                "render_table",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl BaseElementTrait<BigqueryTableCustomElementSettings> for BigqueryTableCustomElement {
    fn new(id: &str, value: &Option<BigqueryTableCustomElementSettings>) -> Self {
        assert!(
            value.is_some(),
            "must pass initializing values to the bq-table element"
        );

        BigqueryTableCustomElement {
            element_id: id.to_string(),
            settings: Box::new(None),
        }
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        // let fn_parameter = self.settings.as_ref();

        BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            .apply_fn(&set_attributes, &Some(self))
            .append_shadow()
            
        // BaseElement::new_and_append("bq-table", "some_id", some_element)
        // .append_shadow()
        // .append_child_style("some css path")
        // .append_sibling_base_element(DataTableControls::new("some_id_1"))
        // .append_sibling_base_element(DataTable::new("some_id_2"))
        // ;

    }
}

fn set_attributes(base_element: &BaseElement, bq_table: &Option<&BigqueryTableCustomElement>) {
    let element = base_element.element();

    if let Some(bq_table) = bq_table {
        element.set_id(&bq_table.element_id);

        if let Some(settings) = bq_table.settings.as_ref() {
            element
                .set_attribute("jobId", settings.job_id.as_str())
                .unwrap();
            //     bq_table_custom_element
            //         .set_attribute("projectId", project_id)
            //         .unwrap();
            //     bq_table_custom_element
            //         .set_attribute("location", location)
            //         .unwrap();
            //     bq_table_custom_element
            //         .set_attribute("token", token)
            //         .unwrap();
            //     if start_index.is_some() {
            //         bq_table_custom_element
            //             .set_attribute("startIndex", &start_index.unwrap().to_string())
            //             .unwrap();
            //     }
        }
    } else {
        element.remove_attribute("jobId").unwrap();
    }
}

impl BigqueryTableCustomElement {
    // pub(crate) fn from_job(token: &String, job: &Job) -> BigqueryTableCustomElement {
    //     let job_reference = job.job_reference.as_ref().expect("job reference not found");

    //     let element = BigqueryTableCustomElement::new_element(
    //         &job_reference.job_id,
    //         &job_reference.project_id,
    //         &job_reference.location,
    //         token,
    //         &Some(0),
    //         &Some(50),
    //     );

    //     BigqueryTableCustomElement {
    //         job_id: job_reference.job_id.clone(),
    //         project_id: job_reference.project_id.clone(),
    //         location: job_reference.location.clone(),
    //         token: token.clone(),
    //         start_index: Some(0),
    //         max_results: Some(50),
    //         element: Box::new(element),
    //     }
    // }

    // pub(crate) fn from_element(element: &Element) -> BigqueryTableCustomElement {
    //     let job_id = element.get_attribute("jobId").unwrap();
    //     let project_id = element.get_attribute("projectId").unwrap();
    //     let location = element.get_attribute("location").unwrap();
    //     let token = element.get_attribute("token").unwrap();

    //     let start_index = parse_to_usize(element.get_attribute("startIndex"));
    //     let max_results = parse_to_usize(element.get_attribute("maxResults"));

    //     BigqueryTableCustomElement {
    //         job_id,
    //         project_id,
    //         location,
    //         token,
    //         start_index,
    //         max_results,
    //         element: Box::new(element.to_owned()),
    //     }
    // }

    // pub(crate) fn element(&self) -> &Box<Element> {
    //     &self.element
    // }

    // fn new_element(
    //     job_id: &String,
    //     project_id: &String,
    //     location: &String,
    //     token: &String,
    //     start_index: &Option<usize>,
    //     max_results: &Option<usize>,
    // ) -> Element {
    //     let bq_table_custom_element =
    //         crate::createElement(&super::CustomElement::BqTable.to_string());

    //     bq_table_custom_element
    //         .set_attribute("jobId", job_id)
    //         .unwrap();
    //     bq_table_custom_element
    //         .set_attribute("projectId", project_id)
    //         .unwrap();
    //     bq_table_custom_element
    //         .set_attribute("location", location)
    //         .unwrap();
    //     bq_table_custom_element
    //         .set_attribute("token", token)
    //         .unwrap();
    //     if start_index.is_some() {
    //         bq_table_custom_element
    //             .set_attribute("startIndex", &start_index.unwrap().to_string())
    //             .unwrap();
    //     }
    //     if max_results.is_some() {
    //         bq_table_custom_element
    //             .set_attribute("maxResults", &max_results.unwrap().to_string())
    //             .unwrap();
    //     }

    //     let id = "test-1";
    //     bq_table_custom_element.set_id(id);
    //     // observe(id);

    //     bq_table_custom_element
    // }

    // fn as_query_results_request(&self) -> GetQueryResultsRequest {
    //     GetQueryResultsRequest {
    //         project_id: self.project_id.clone(),
    //         job_id: self.job_id.clone(),
    //         start_index: Some(self.start_index.clone().unwrap_or(0).to_string()),
    //         page_token: None,
    //         max_results: self.max_results.clone(),
    //         timeout_ms: None,
    //         location: Some(self.location.clone()),
    //     }
    // }

    fn on_render_table(event: &web_sys::Event) {
        // let element = event
        //     .target()
        //     .unwrap()
        //     .dyn_into::<web_sys::Element>()
        //     .unwrap();

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     "on_render_table event on element: {:?}",
        //     element.id()
        // )));

        // let bq_table_element = BigqueryTableCustomElement::from_element(&element);
        // let jobs = Jobs::new(&bq_table_element.token);
        // let request = bq_table_element.as_query_results_request();

        // spawn_local(async move {
        //     let response = jobs.get_query_results(request).await;
        //     if let Some(response) = response {
        //         response.plot_table(&element);
        //     } else {
        //         element.set_inner_html(&format!("unexpected response: {:?}", response));
        //     }
        // });
    }

    // pub(crate) fn first_page(&self) {
    //     self.element.set_attribute("startIndex", "0").unwrap();
    // }

    // pub(crate) fn previous_page(&self) {
    //     let start_index = parse_to_usize(self.element.get_attribute("startIndex")).unwrap_or(0);
    //     let max_results = parse_to_usize(self.element.get_attribute("maxResults")).unwrap_or(50);

    //     self.element
    //         .set_attribute("startIndex", &format!("{0}", start_index - max_results))
    //         .unwrap();
    // }

    // pub(crate) fn next_page(&self) {
    //     let start_index = parse_to_usize(self.element.get_attribute("startIndex")).unwrap_or(0);
    //     let max_results = parse_to_usize(self.element.get_attribute("maxResults")).unwrap_or(50);

    //     self.element
    //         .set_attribute("startIndex", &format!("{0}", start_index + max_results))
    //         .unwrap();
    // }

    // pub(crate) fn last_page(&self) {
    //     let start_index = parse_to_usize(self.element.get_attribute("startIndex")).unwrap_or(0);
    //     let max_results = parse_to_usize(self.element.get_attribute("maxResults")).unwrap_or(50);

    //     self.element
    //         .set_attribute("startIndex", &format!("{0}", start_index + max_results))
    //         .unwrap();
    // }
}
