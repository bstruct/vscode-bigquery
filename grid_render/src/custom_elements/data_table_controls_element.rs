use crate::bigquery::{base::TableReference, jobs::JobReference};

use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use serde_json::json;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::Element;

const PAGING: &str = "paging";
const BTN_FIRST_PAGE: &str = "btn_first_page";
const BTN_PREVIOUS_PAGE: &str = "btn_prev_page";
const BTN_NEXT_PAGE: &str = "btn_next_page";
const BTN_LAST_PAGE: &str = "btn_last_page";
const BTN_DOWNLOAD_CSV: &str = "btn_download_csv";
const BTN_DOWNLOAD_JSONL: &str = "btn_download_json";
const BTN_SEND_PUBSUB: &str = "btn_send_pubsub";

pub(crate) const EVENT_GO_TO_FIRST_PAGE: &str = "go_to_first_page";
pub(crate) const EVENT_GO_TO_PREVIOUS_PAGE: &str = "go_to_previous_page";
pub(crate) const EVENT_GO_TO_NEXT_PAGE: &str = "go_to_next_page";
pub(crate) const EVENT_GO_TO_LAST_PAGE: &str = "go_to_last_page";

#[derive(Debug)]
pub(crate) struct DataTableControls {
    // parent_bq_table_id: String,
    page_start_index: Option<usize>,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,

    job_reference: Option<JobReference>,
    table_reference: Option<TableReference>,
}

impl DataTableControls {
    pub(crate) fn new(
        page_start_index: Option<usize>,
        rows_in_page: Option<usize>,
        rows_total: Option<usize>,
        job_reference: Option<JobReference>,
        table_reference: Option<TableReference>,
    ) -> DataTableControls {
        DataTableControls {
            page_start_index: page_start_index,
            rows_in_page: rows_in_page,
            rows_total: rows_total,
            job_reference: job_reference,
            table_reference: table_reference,
        }
    }
}

impl BaseElementTrait for DataTableControls {
    fn get_element_id(&self) -> &str {
        "controls-background"
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        BaseElement::new_and_append(parent_node, "div", &self.get_element_id())
            .append_child("div", "controls")
            .append_child_fn("span", PAGING, &modify_controls, self)
            .append_sibling_fn("button", BTN_FIRST_PAGE, &modify_controls, self)
            .append_sibling_fn("button", BTN_PREVIOUS_PAGE, &modify_controls, self)
            .append_sibling_fn("button", BTN_NEXT_PAGE, &modify_controls, self)
            .append_sibling_fn("button", BTN_LAST_PAGE, &modify_controls, self)
            .append_sibling_fn("button", BTN_DOWNLOAD_CSV, &modify_controls, self)
            .append_sibling_fn("button", BTN_DOWNLOAD_JSONL, &modify_controls, self)
            .append_sibling_fn("button", BTN_SEND_PUBSUB, &modify_controls, self)
    }
}

fn modify_controls(base_element: &BaseElement, settings: &DataTableControls) {
    let id = match base_element.id().as_deref() {
        Some(id) => id,
        None => return,
    };
    match id {
        PAGING => {
            if settings.rows_in_page.is_some()
                && settings.rows_total.is_some()
                && settings.page_start_index.is_some()
            {
                let rows_in_page = settings.rows_in_page.unwrap_or(0);
                let rows_total = settings.rows_total.unwrap_or(0);
                let page_start_index = settings.page_start_index.unwrap_or(0);

                let page_start = if rows_in_page == 0 {
                    0
                } else {
                    page_start_index + 1
                };

                base_element.element().set_inner_html(&format!(
                    "{} - {} of {}",
                    page_start,
                    page_start_index + rows_in_page,
                    rows_total
                ));
            } else {
                base_element.element().set_inner_html("");
            }
        }
        BTN_FIRST_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_FIRST_PAGE);
            element.set_inner_html("<< First page");
            if settings.page_start_index.unwrap_or(0) == 0 {
                let _ = element.set_attribute("disabled", "disabled");
            } else {
                let _ = element.remove_attribute("disabled");
            }
        }
        BTN_PREVIOUS_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_PREVIOUS_PAGE);
            element.set_inner_html("< Previous page");
            if settings.page_start_index.unwrap_or(0) == 0 {
                let _ = element.set_attribute("disabled", "disabled");
            } else {
                let _ = element.remove_attribute("disabled");
            }
        }
        BTN_NEXT_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_NEXT_PAGE);
            element.set_inner_html("> Next page");

            let start_index = settings.page_start_index.unwrap_or(0);
            let page_size = settings.rows_in_page.unwrap_or(0);
            let rows_total = settings.rows_total.unwrap_or(0);
            let has_next_page = start_index + page_size < rows_total;
            if has_next_page {
                let _ = element.remove_attribute("disabled");
            } else {
                let _ = element.set_attribute("disabled", "disabled");
            }
        }
        BTN_LAST_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_LAST_PAGE);
            element.set_inner_html(">> Last page");

            let start_index = settings.page_start_index.unwrap_or(0);
            let page_size = settings.rows_in_page.unwrap_or(0);
            let rows_total = settings.rows_total.unwrap_or(0);
            let has_next_page = start_index + page_size < rows_total;
            if has_next_page {
                let _ = element.remove_attribute("disabled");
            } else {
                let _ = element.set_attribute("disabled", "disabled");
            }
        }
        BTN_DOWNLOAD_CSV => {
            let element = &base_element.element();
            add_event_listener_command(element, BTN_DOWNLOAD_CSV, settings);
            element.set_inner_html("Download CSV");
        }
        BTN_DOWNLOAD_JSONL => {
            let element = &base_element.element();
            add_event_listener_command(element, BTN_DOWNLOAD_JSONL, settings);
            element.set_inner_html("Download JSONL");
        }
        BTN_SEND_PUBSUB => {
            let element = &base_element.element();
            if settings.job_reference.is_some() {
                add_event_listener_command(element, BTN_SEND_PUBSUB, settings);
                element.set_inner_html("Send to Pub/Sub");
            } else {
                let _ = element.set_attribute("style", "display: none;");
            }
        }
        _ => {}
    }
}

fn add_event_listener(element: &Element, _event_type: &str) {
    if element.get_attribute("bee").is_none() {
        let on_event_type_closure =
            Closure::wrap(Box::new(on_click) as Box<dyn Fn(&web_sys::Event)>);

        let _ = element.add_event_listener_with_callback(
            "click",
            on_event_type_closure.as_ref().unchecked_ref(),
        );

        let _ = element.set_attribute("bee", "1");

        on_event_type_closure.forget();
    }
}

fn add_event_listener_command(
    element: &Element,
    button_name: &str,
    datatable_controls: &DataTableControls,
) {
    if element.get_attribute("bee").is_none() {
        let command_name = match button_name {
            BTN_DOWNLOAD_CSV => "download_csv",
            BTN_DOWNLOAD_JSONL => "download_jsonl",
            BTN_SEND_PUBSUB => "send_pubsub",
            other => {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                    "add_event_listener_command: unexpected button '{}'",
                    other
                )));
                return;
            }
        };

        // let job_reference = datatable_controls.job_reference.as_ref();

        let function_body = if let Some(job_reference) = &datatable_controls.job_reference {
            json!({
                "command" : command_name,
                "type": "job_reference",
                "job_reference": {
                    "location": job_reference.location,
                    "projectId": job_reference.project_id,
                    "jobId": job_reference.job_id
                }
            })
        } else {
            // let table_reference = datatable_controls.table_reference.as_ref();

            if let Some(table_reference) = &datatable_controls.table_reference {
                json!({
                    "command" : command_name,
                    "type": "table_reference",
                    "table_reference": {
                        "projectId": table_reference.project_id,
                        "datasetId": table_reference.dataset_id,
                        "tableId": table_reference.table_id
                    }
                })
            } else {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                    "add_event_listener_command: neither job_reference nor table_reference found",
                ));
                return;
            }
        };

        let function_body = format!("vscode.postMessage({0});", function_body);
        let call_command = js_sys::Function::new_no_args(&function_body);

        let _ = element.add_event_listener_with_callback("click", call_command.as_ref());

        let _ = element.set_attribute("bee", "1");
    }
}

fn on_click(event: &web_sys::Event) {
    let element = match event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        Some(e) => e,
        None => {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                "on_click: event target is not an element",
            ));
            return;
        }
    };

    let custom_event_init = web_sys::CustomEventInit::new();
    custom_event_init.set_bubbles(true);
    custom_event_init.set_cancelable(true);
    custom_event_init.set_composed(true);

    let base_element = BaseElement::from_element(&element);
    let type_ = match base_element.id().as_deref() {
        Some(BTN_FIRST_PAGE) => EVENT_GO_TO_FIRST_PAGE,
        Some(BTN_PREVIOUS_PAGE) => EVENT_GO_TO_PREVIOUS_PAGE,
        Some(BTN_NEXT_PAGE) => EVENT_GO_TO_NEXT_PAGE,
        Some(BTN_LAST_PAGE) => EVENT_GO_TO_LAST_PAGE,
        other => {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                "on_click: unknown button id '{:?}'",
                other
            )));
            return;
        }
    };

    if let Ok(Some(controls)) = element.closest(":host > [be_id=\"controls-background\"]") {
        if let Some(shadow) = controls.parent_node() {
            if let Some(bstruct_table) = shadow.last_child() {
                let _ = shadow.remove_child(&bstruct_table);

                let loading_div = &crate::createElement("div");
                loading_div.set_text_content(Some("Loading..."));
                let _ = shadow.append_child(loading_div);
            }
        }
    }

    if let Ok(action_event) =
        web_sys::CustomEvent::new_with_event_init_dict(type_, &custom_event_init)
    {
        let _ = element.dispatch_event(&action_event);
    }
}
