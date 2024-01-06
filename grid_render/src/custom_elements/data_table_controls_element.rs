use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Element;

const PAGING: &str = "paging";
const BTN_FIRST_PAGE: &str = "btn_first_page";
const BTN_PREVIOUS_PAGE: &str = "btn_prev_page";
const BTN_NEXT_PAGE: &str = "btn_next_page";
const BTN_LAST_PAGE: &str = "btn_last_page";

#[derive(Debug)]
pub(crate) struct DataTableControls {
    page_start_index: Option<usize>,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,
}

impl DataTableControls {
    pub(crate) fn new(
        page_start_index: Option<usize>,
        rows_in_page: Option<usize>,
        rows_total: Option<usize>,
    ) -> DataTableControls {
        DataTableControls {
            page_start_index: page_start_index,
            rows_in_page: rows_in_page,
            rows_total: rows_total,
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
    }
}

fn modify_controls(base_element: &BaseElement, settings: &DataTableControls) {
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
            add_event_listener(element);
            element.set_inner_html("<< First page");
            // if settings.is_none() {
            //     element.set_attribute("disabled", "disabled").unwrap();
            // } else {
            //     element.remove_attribute("disabled").unwrap();
            // }
        }
        BTN_PREVIOUS_PAGE => {
            let element = &base_element.element();
            add_event_listener(element);
            element.set_inner_html("< Previous page");
        }
        BTN_NEXT_PAGE => {
            let element = &base_element.element();
            add_event_listener(element);
            element.set_inner_html("> Next page");
        }
        BTN_LAST_PAGE => {
            let element = &base_element.element();
            add_event_listener(element);
            element.set_inner_html(">> Last page");
        }
        _ => {}
    }
}

fn add_event_listener(element: &Element) {
    let on_event_type_closure = Closure::wrap(Box::new(on_click) as Box<dyn Fn(&web_sys::Event)>);

    element
        .add_event_listener_with_callback("click", on_event_type_closure.as_ref().unchecked_ref())
        .unwrap();

    on_event_type_closure.forget();
}

fn on_click(event: &web_sys::Event) {
    let element = event
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap();

    let base_element = BaseElement::from_element(&element);

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "on_click event on element: {:?}",
        element.get_attribute("be_id").unwrap_or(element.id())
    )));

    // let bq_table = getElementById("test-1");

    // if let Some(bq_table) = bq_table {
    //     let bq_table = BigqueryTableCustomElement::from_element(&bq_table);

    //     match base_element.id().as_str() {
    //         BTN_FIRST_PAGE => bq_table.first_page(),
    //         BTN_PREVIOUS_PAGE => bq_table.previous_page(),
    //         BTN_NEXT_PAGE => bq_table.next_page(),
    //         BTN_LAST_PAGE => bq_table.last_page(),
    //         _ => {}
    //     };
    // }
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

        DataTableControls::new(Some(0), Some(10), Some(100)).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1 - 10 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(10), Some(10), Some(100)).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">11 - 20 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(20), Some(10), Some(100)).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">21 - 30 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(Some(30), Some(10), Some(100)).render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">31 - 40 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");
    }
}
