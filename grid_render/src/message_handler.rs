use crate::{custom_elements::base_element_trait::BaseElementTrait, observe_element};
use wasm_bindgen::JsValue;

use crate::{
    createElement,
    external_request::{ExternalRequest, ExternalRequestError},
    getElementById,
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
                "preview_table" => {
                    preview_table(q1, external_request).await;
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
            //The job is marked as SCRIPT, one of 2 scenarios are possible:
            // - Just one line script, for example "DECLARE x STRING...."
            // - Multiple queries are placed
            //With the information passed, there's no way to determine which case it is, so another call to BQ must be made

            let element_id = "bq_script_1";
            let bq_table = external_request.to_bq_script(element_id);
            bq_table.render(q1);

            observe_element(&q1.last_element_child().unwrap());

            // if job.is_multi_query_job() {
            //     let job_reference = job.job_reference.as_ref().unwrap();
            //     q1.set_inner_html(&format!("multiple results {}", job_reference.job_id));
            // } else{
            //     let job_reference = job.job_reference.as_ref().unwrap();
            //     q1.set_inner_html(&format!("script {}", job_reference.job_id));

            // }

        } else {
            let element_id = "bq_query_1";
            let bq_table = external_request.to_bq_query(element_id);
            bq_table.render(q1);

            //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
            observe_element(&q1.last_element_child().unwrap());
        }
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}

async fn preview_table(q1: &web_sys::Element, external_request: &ExternalRequest) {
    //clear the div
    q1.set_inner_html(&"");

    if external_request.project_id.is_some()
        && external_request.dataset_id.is_some()
        && external_request.table_id.is_some()
    {
        let element_id = "bq_table_1";
        let bq_table = external_request.to_bq_table(element_id);
        bq_table.render(q1);

        //https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
        observe_element(&q1.last_element_child().unwrap());
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
        title.set_inner_html(&"ERROR");
        q1.append_child(title).unwrap();

        let div_for_table = &createElement("div");
        q1.append_child(div_for_table).unwrap();

        error.plot_table(div_for_table);
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}
