use crate::bigquery::{base::TableReference, jobs::JobReference};

use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use wasm_bindgen::{closure::Closure, JsCast};
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
    // //set the attribute PARENT_BQ_TABLE_ATT, with the id of the parent bq-table element, to later be possible to capture on the click event
    // base_element
    //     .element()
    //     .set_attribute(
    //         PARENT_BQ_TABLE_ATT,
    //         &settings.parent_bq_table_id.to_string(),
    //     )
    //     .unwrap();

    //
    match base_element.id().as_ref().unwrap().as_str() {
        PAGING => {
            if settings.rows_in_page.is_some()
                && settings.rows_total.is_some()
                && settings.page_start_index.is_some()
            {
                let rows_in_page = settings.rows_in_page.unwrap_or(0);
                let rows_total = settings.rows_total.unwrap_or(0);
                let page_start_index = settings.page_start_index.unwrap_or(0);

                base_element.element().set_inner_html(&format!(
                    "{} - {} of {}",
                    page_start_index + 1,
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
            // if settings.is_none() {
            //     element.set_attribute("disabled", "disabled").unwrap();
            // } else {
            //     element.remove_attribute("disabled").unwrap();
            // }
        }
        BTN_PREVIOUS_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_PREVIOUS_PAGE);
            element.set_inner_html("< Previous page");
        }
        BTN_NEXT_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_NEXT_PAGE);
            element.set_inner_html("> Next page");
        }
        BTN_LAST_PAGE => {
            let element = &base_element.element();
            add_event_listener(element, EVENT_GO_TO_LAST_PAGE);
            element.set_inner_html(">> Last page");
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
            add_event_listener_command(element, BTN_SEND_PUBSUB, settings);
            element.set_inner_html("Send to Pub/Sub");
        }
        _ => {}
    }
}

fn add_event_listener(element: &Element, _event_type: &str) {
    if element.get_attribute("bee").is_none() {
        let on_event_type_closure =
            Closure::wrap(Box::new(on_click) as Box<dyn Fn(&web_sys::Event)>);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     "add_event_listener - {:?}",
        //     element.get_attribute("be_id")
        // )));

        element
            .add_event_listener_with_callback(
                "click",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        element.set_attribute("bee", "1").unwrap();

        on_event_type_closure.forget();
    }
}

fn add_event_listener_command(
    element: &Element,
    button_name: &str,
    datatable_controls: &DataTableControls,
) {
    if element.get_attribute("bee").is_none() {
        // let command_name = match button_name {
        //     BTN_DOWNLOAD_CSV => "download_csv",
        //     BTN_DOWNLOAD_JSONL => "download_jsonl",
        //     BTN_SEND_PUBSUB => "send_pubsub",
        //     _ => panic!("unexpected button"),
        // };

        // let function_body = if let Some(job_reference) = &datatable_controls.job_reference {
        //     stringify!({
        //         "command" : command_name,
        //         "type": "job_reference",
        //         "job_reference": {
        //             "location": job_reference.location,
        //             "project_id": job_reference.project_id,
        //             "job_id": job_reference.job_id
        //         }
        //     })
        // } else {
        //     if let Some(table_reference) = &datatable_controls.table_reference {
        //         stringify!({
        //             "command" : command_name,
        //             "type": "table_reference",
        //             "table_reference": {
        //                 "project_id": table_reference.project_id,
        //                 "dataset_id": table_reference.dataset_id,
        //                 "table_id": table_reference.table_id
        //             }
        //         })
        //     } else {
        //         panic!("Unexpected. No job_reference nor table_reference found");
        //     }
        // };

        let function_body = "123";
        let call_command = js_sys::Function::new_no_args(&function_body);

        element
            .add_event_listener_with_callback("click", call_command.as_ref())
            .unwrap();

        element.set_attribute("bee", "1").unwrap();
    }
}

fn on_click(event: &web_sys::Event) {
    let element = event
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap();

    let mut custom_event_init = web_sys::CustomEventInit::new();
    custom_event_init.bubbles(true);
    custom_event_init.cancelable(true);
    custom_event_init.composed(true);

    let base_element = BaseElement::from_element(&element);
    let type_ = match base_element.id().as_ref().unwrap().as_str() {
        BTN_FIRST_PAGE => EVENT_GO_TO_FIRST_PAGE,
        BTN_PREVIOUS_PAGE => EVENT_GO_TO_PREVIOUS_PAGE,
        BTN_NEXT_PAGE => EVENT_GO_TO_NEXT_PAGE,
        BTN_LAST_PAGE => EVENT_GO_TO_LAST_PAGE,
        _ => panic!("unknown button"),
    };

    let action_event =
        web_sys::CustomEvent::new_with_event_init_dict(type_, &custom_event_init).unwrap();

    element.dispatch_event(&action_event).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::custom_elements::base_element_trait::BaseElementTrait;

    use super::DataTableControls;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generate_html() {
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let element = &crate::createElement("div");
        let parent_element = &element.attach_shadow(&shadow_init).unwrap();
        // let parent_bq_table_id = "parent_bq_table_id";

        DataTableControls::new(Some(0), Some(10), Some(100), None, None).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\" parent_bq_table=\"parent_bq_table_id\">1 - 10 of 100</span><button be_id=\"btn_first_page\" parent_bq_table=\"parent_bq_table_id\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\" parent_bq_table=\"parent_bq_table_id\">&lt; Previous page</button><button be_id=\"btn_next_page\" parent_bq_table=\"parent_bq_table_id\">&gt; Next page</button><button be_id=\"btn_last_page\" parent_bq_table=\"parent_bq_table_id\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(10), Some(10), Some(100), None, None).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\" parent_bq_table=\"parent_bq_table_id\">11 - 20 of 100</span><button be_id=\"btn_first_page\" parent_bq_table=\"parent_bq_table_id\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\" parent_bq_table=\"parent_bq_table_id\">&lt; Previous page</button><button be_id=\"btn_next_page\" parent_bq_table=\"parent_bq_table_id\">&gt; Next page</button><button be_id=\"btn_last_page\" parent_bq_table=\"parent_bq_table_id\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(20), Some(10), Some(100), None, None).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\" parent_bq_table=\"parent_bq_table_id\">21 - 30 of 100</span><button be_id=\"btn_first_page\" parent_bq_table=\"parent_bq_table_id\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\" parent_bq_table=\"parent_bq_table_id\">&lt; Previous page</button><button be_id=\"btn_next_page\" parent_bq_table=\"parent_bq_table_id\">&gt; Next page</button><button be_id=\"btn_last_page\" parent_bq_table=\"parent_bq_table_id\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(30), Some(10), Some(100), None, None).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\" parent_bq_table=\"parent_bq_table_id\">31 - 40 of 100</span><button be_id=\"btn_first_page\" parent_bq_table=\"parent_bq_table_id\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\" parent_bq_table=\"parent_bq_table_id\">&lt; Previous page</button><button be_id=\"btn_next_page\" parent_bq_table=\"parent_bq_table_id\">&gt; Next page</button><button be_id=\"btn_last_page\" parent_bq_table=\"parent_bq_table_id\">&gt;&gt; Last page</button></div></div>");
    }
}
