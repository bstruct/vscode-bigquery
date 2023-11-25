use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::{bigquery::jobs::Job, getElementById};

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
        let p = parse_event.as_ref().unwrap();
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "parsed: {}: {}",
            p.request_type, p.query
        )));

        let q1 = getElementById("q1");
        if q1.is_some() {
            let q1 = &q1.unwrap();

            q1.set_inner_html(&"Loading...");
            // launch query
            // let bq_jobs = Jobs::new(&p.token);
            // let project_id = &p.project_id;
            let job = &p.job;
            if job.is_some() {
                let job = job.as_ref().unwrap();

                q1.set_inner_html(&format!(
                    "Loading job {}",
                    job.id.clone().unwrap_or_default()
                ));
            } else {
                q1.set_inner_html(&"Unexpected error occured.");
            }
            // let request = QueryRequest {
            //     query: String::from(&p.query),
            //     max_results: Some(50),
            // };
        } else {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
        }
    }
}
