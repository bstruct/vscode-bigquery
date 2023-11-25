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
    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "event {}: {:?}",
    //     event.type_(),
    //     event.data()
    // )));

    // let event_data: ExternalRequest = event.data().into();
    // let job_response = JSON::parse(event_data).expect(&"error 2");

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
            //     // launch query
            //     let bq_jobs = Jobs::new(&p.token);
            //     let project_id = &p.project_id;
            let job = &p.job;
            //     // let request = QueryRequest {
            //     //     query: String::from(&p.query),
            //     //     max_results: Some(50),
            //     // };

            //     // let response = bq_jobs.query(project_id, request).await;

            //     // let request = Job {
            //     //     // kind: (),
            //     //     // etag: (),
            //     //     // id: (),
            //     //     // self_link: (),
            //     //     // user_email: (),
            //     //     configuration: JobConfiguration {
            //     //         dry_run: false,
            //     //         query: JobConfigurationQuery { query: p.query.to_owned() },
            //     //         ..Default::default()
            //     //     },
            //     //     ..Default::default()
            //     //     // job_reference: (),
            //     //     // statistics: (),
            //     //     // status: (),
            //     //     // principal_subject: (),

            //     // };

            //     // let response = bq_jobs.insert(project_id, request).await;

            //     // if response.is_some() {
            q1.unwrap()
                .set_inner_html(&format!("job {:?}", serde_json::json!(&job)));
            //     // } else {
            //     //     q1.unwrap().set_inner_html("no response");
            //     // }
        } else {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
        }
    }
}
