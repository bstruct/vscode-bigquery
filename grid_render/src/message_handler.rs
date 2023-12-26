use wasm_bindgen::JsValue;

use crate::{
    createElement,
    custom_elements::bq_table_custom_element::BigqueryTableCustomElement,
    external_request::{ExternalRequest, ExternalRequestError},
    getElementById,
    observe,
};

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
                "error" => {
                    show_error(q1, external_request).await;
                }
                _ => {}
            }
        } else {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
        }
    }
}

async fn execute_query(q1: &web_sys::Element, external_request: &ExternalRequest) {
    //clear the div
    q1.set_inner_html(&"");

    let job = &external_request.job;
    if job.is_some() {
        let job = job.as_ref().unwrap();

        if job.is_query_script() {
            let job_reference = job.job_reference.as_ref().unwrap();
            q1.set_inner_html(&format!("multiple results {}", job_reference.job_id));
        } else {
            let token = external_request.token.as_ref().expect("token not found");
            let bq_table_custom_element =
                &BigqueryTableCustomElement::new_html_element_from_job(token, job);
            q1.append_child(&bq_table_custom_element.html_element())
                .unwrap();

            //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
            observe(&bq_table_custom_element.html_element().id());

            // q1.set_inner_html(&format!("Loading job {}", job_reference.job_id));

            // let bq_jobs = bigquery::jobs::Jobs::new(&external_request.token.as_ref().unwrap());
            // let request = GetQueryResultsRequest {
            //     project_id: String::from(job_reference.project_id.clone()),
            //     job_id: String::from(job_reference.job_id.clone()),
            //     start_index: Some(String::from("0")),
            //     page_token: None,
            //     max_results: Some(50),
            //     timeout_ms: None,
            //     location: Some(String::from(job_reference.location.clone())),
            // };

            // let results = bq_jobs.get_query_results(request).await;
            // if results.is_some() {
            //     let query_result_response = results.unwrap();

            //     console::log_1(&JsValue::from_str(
            //         &serde_json::to_string(&query_result_response).unwrap(),
            //     ));

            //     let div_for_table = &createElement("div");
            //     q1.set_inner_html(&"");
            //     q1.append_child(div_for_table).unwrap();

            //     query_result_response.plot_table(div_for_table);
            // }
        }
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}

async fn show_error(q1: &web_sys::Element, external_request: &ExternalRequest) {
    q1.set_inner_html(&"Loading...");

    let error: &Option<ExternalRequestError> = &external_request.error;
    if error.is_some() {
        let error = error.as_ref().unwrap();
        q1.set_inner_html(&"");

        let title = &createElement("h3");
        title.set_inner_text(&"ERROR");
        q1.append_child(title).unwrap();

        let div_for_table = &createElement("div");
        q1.append_child(div_for_table).unwrap();

        error.plot_table(div_for_table);
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}
