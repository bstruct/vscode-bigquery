use serde::Deserialize;

use crate::{getElementById, bigquery::jobs::{Jobs, QueryRequest}};

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    pub token: String,
    pub query: String,
}

pub async fn handle(event: &web_sys::MessageEvent) {
    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "event {}: {:?}",
    //     event.type_(),
    //     event.data()
    // )));

    let data = event.data();

    let p = &serde_wasm_bindgen::from_value::<ExternalRequest>(data).unwrap();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "parsed: {}: {}",
        p.request_type,
        p.query
    )));

    let q1 = getElementById("q1");
    if q1.is_some(){

        // launch query
        let bq_jobs = Jobs::new(&p.token);
        let project_id = "";
        // let request:QueryRequest = {
        //     query: "()", 
        //     max_results: 10,
        //     ..Default()
        // };

        // await bq_jobs.query(project_id, request);


        q1.unwrap().set_inner_html(&p.query);
    }
}