use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::{
    bigquery::{
        self,
        jobs::{GetQueryResultsRequest, Job},
    },
    createElement, getElementById,
};

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    #[serde(alias = "projectId")]
    pub project_id: Option<String>,
    pub token: Option<String>,
    pub query: Option<String>,
    pub job: Option<Job>,
}

pub async fn handle(event: &web_sys::MessageEvent) {
    let parse_event = &serde_wasm_bindgen::from_value::<ExternalRequest>(event.data());

    if parse_event.is_err() {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "error parsing json: {:?}",
            parse_event.as_ref().unwrap_err()
        )));
    } else {
        let external_request = parse_event.as_ref().unwrap();
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "request_type: {}",
            external_request.request_type
        )));

        let q1 = getElementById("q1");
        if q1.is_some() {
            let q1 = &q1.unwrap();

            match external_request.request_type.as_str() {
                "clear" => {
                    q1.set_inner_html(&"Loading...");
                }
                "execute_query" => {
                    execute_query(q1, external_request).await;
                }
                _ => {}
            }
        } else {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
        }
    }
}

async fn execute_query(q1: &web_sys::Element, external_request: &ExternalRequest) {
    q1.set_inner_html(&"Loading...");

    let job = &external_request.job;
    if job.is_some() {
        let job = job.as_ref().unwrap();
        let job_reference = job.job_reference.as_ref().unwrap();

        q1.set_inner_html(&format!("Loading job {}", job_reference.job_id));

        let bq_jobs = bigquery::jobs::Jobs::new(&external_request.token.as_ref().unwrap());
        let request = GetQueryResultsRequest {
            project_id: String::from(job_reference.project_id.clone()),
            job_id: String::from(job_reference.job_id.clone()),
            start_index: Some(String::from("0")),
            page_token: None,
            max_results: Some(50),
            timeout_ms: None,
            location: Some(String::from(job_reference.location.clone())),
        };

        let results = bq_jobs.get_query_results(request).await;
        if results.is_some() {
            let query_result_response = results.unwrap();

            let div_for_table = &createElement("div");
            q1.set_inner_html(&"");
            q1.append_child(div_for_table).unwrap();

            query_result_response.plot_table(div_for_table);
        }
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}
