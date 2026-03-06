use crate::{custom_elements::base_element_trait::BaseElementTrait};
use wasm_bindgen::JsValue;

use crate::{
    createElement,
    external_request::{ExternalRequest, ExternalRequestError},
    getElementById,
};

pub async fn handle(event: &web_sys::MessageEvent) {
    let external_request = match serde_wasm_bindgen::from_value::<ExternalRequest>(event.data()) {
        Ok(r) => r,
        Err(e) => {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                "error parsing json: {:?}",
                e
            )));
            return;
        }
    };

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "request_type: {}",
        external_request.request_type
    )));

    let q1 = match getElementById("q1") {
        Some(el) => el,
        None => {
            web_sys::console::log_1(&JsValue::from("q1 not found"));
            return;
        }
    };

    match external_request.request_type.as_str() {
        "clear" => {
            q1.set_inner_html(&"Loading...");
        }
        "execute_query" => {
            q1.set_inner_html(&"Loading...");
            execute_query(&q1, &external_request);
        }
        "preview_table" => {
            q1.set_inner_html(&"Loading...");
            preview_table(&q1, &external_request);
        }
        "error" => {
            show_error(&q1, &external_request);
        }
        _ => {}
    }
}

fn execute_query(q1: &web_sys::Element, external_request: &ExternalRequest) {
    //clear the div
    q1.set_inner_html(&"");

    if let Some(job) = &external_request.job {
        web_sys::console::log_1(&JsValue::from(format!(
            "job statistics: {:?}",
            job.statistics
        )));

        let element_id = "bq_script_1";
        match external_request.to_bq_script(element_id) {
            Some(bq_script) => {
                bq_script.render(q1);
                bq_script.dispatch_on_render_event(q1);
            }
            None => {
                web_sys::console::error_1(&JsValue::from_str(
                    "execute_query: missing required fields (job_reference, project_id, or token)",
                ));
                q1.set_inner_html(&"Unexpected error occured.");
            }
        }
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}

fn preview_table(q1: &web_sys::Element, external_request: &ExternalRequest) {
    q1.set_inner_html(&"");

    let element_id = "bq_table_1";
    match external_request.to_bq_table(element_id) {
        Some(bq_table) => {
            bq_table.render(q1);
            bq_table.dispatch_on_render_event(q1);
        }
        None => {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "preview_table: missing required fields (project_id, dataset_id, table_id, or token)",
            ));
            q1.set_inner_html(&"Unexpected error occured.");
        }
    }
}

fn show_error(q1: &web_sys::Element, external_request: &ExternalRequest) {
    q1.set_inner_html(&"Loading...");

    if let Some(error) = &external_request.error {
        q1.set_inner_html(&"");

        let title = &createElement("h3");
        title.set_inner_html(&"ERROR");
        if let Err(e) = q1.append_child(title) {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "show_error: failed to append title: {:?}",
                e
            )));
        }

        let div_for_table = &createElement("div");
        if let Err(e) = q1.append_child(div_for_table) {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "show_error: failed to append error table container: {:?}",
                e
            )));
            return;
        }

        error.plot_table(div_for_table);
    } else {
        q1.set_inner_html(&"Unexpected error occured.");
    }
}
