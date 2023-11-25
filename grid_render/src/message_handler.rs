use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::{bigquery::{jobs::{Job, self, GetQueryResultsRequest}, self}, getElementById};

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    #[serde(alias = "projectId")]
    pub project_id: String,
    pub token: String,
    pub query: String,
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
            "parsed: {}: {}",
            external_request.request_type, external_request.query
        )));

        let q1 = getElementById("q1");
        if q1.is_some() {
            let q1 = &q1.unwrap();

            q1.set_inner_html(&"Loading...");

            let job = &external_request.job;
            if job.is_some() {
                let job = job.as_ref().unwrap();

                q1.set_inner_html(&format!(
                    "Loading job {}",
                    job.id.clone().unwrap_or_default()
                ));

                let job_reference = &job.job_reference.as_ref().unwrap();

                let bq_jobs = bigquery::jobs::Jobs::new(&external_request.token);
                let request = GetQueryResultsRequest{
                    project_id: String::from(job_reference.project_id.clone()),
                    job_id: String::from(job_reference.job_id.clone()),
                    start_index: Some(String::from("0")),
                    page_token: None,
                    max_results: Some(50),
                    timeout_ms: None,
                    location: Some(String::from(job_reference.location.clone())),
                };

                let results = bq_jobs.get_query_results(request).await;
                if results.is_some(){

                    let results = results.unwrap();

                    q1.set_inner_html(&format!(
                        "total_rows {}",
                        results.total_rows
                    ));
                    
                }

            } else {
                q1.set_inner_html(&"Unexpected error occured.");
            }
    
        } else {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
        }
    }
}
