use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Element;

pub(crate) struct DataTableControlsSettings {
    page_start_index: Option<usize>,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,
}

impl DataTableControlsSettings {
    pub(crate) fn new(
        page_start_index: usize,
        rows_in_page: usize,
        rows_total: usize,
    ) -> DataTableControlsSettings {
        DataTableControlsSettings {
            page_start_index: Some(page_start_index),
            rows_in_page: Some(rows_in_page),
            rows_total: Some(rows_total),
        }
    }
}

pub(crate) struct DataTableControls {
    page_start_index: Option<usize>,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,
    element_id: String,
}

const PAGING: &str = "paging";
const BTN_FIRST_PAGE: &str = "btn_first_page";
const BTN_PREV_PAGE: &str = "btn_prev_page";
const BTN_NEXT_PAGE: &str = "btn_next_page";
const BTN_LAST_PAGE: &str = "btn_last_page";

impl BaseElementTrait<DataTableControlsSettings> for DataTableControls {
    fn new(id: &str, value: &Option<DataTableControlsSettings>) -> Self {
        assert_eq!(
            id,
            DataTableControls::BASE_ID,
            "please use the the constant `DataTableControls::BASE_ID` as id parameter"
        );

        let value = value.as_ref().unwrap();

        DataTableControls {
            page_start_index: value.page_start_index,
            rows_in_page: value.rows_in_page,
            rows_total: value.rows_total,
            element_id: id.to_owned(),
        }
    }

    fn render(&self, parent_node: &web_sys::Node) {
        let parameter_1 = Some(self);

        BaseElement::new_and_append(parent_node, "div", &self.element_id)
            .append_child("div", "controls")
            .append_child_fn("span", PAGING, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_FIRST_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_PREV_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_NEXT_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_LAST_PAGE, &modify_controls, parameter_1);
    }
}

impl DataTableControls {
    pub const BASE_ID: &'static str = "controls-background";
}

fn modify_controls(base_element: &BaseElement, values: Option<&DataTableControls>) {
    let rows_in_page = values.unwrap().rows_in_page.unwrap_or(0);
    let rows_total = values.unwrap().rows_total.unwrap_or(0);
    let page_start_index = values.unwrap().page_start_index.unwrap_or(0);

    match base_element.id().as_str() {
        PAGING => {
            base_element.element().set_inner_html(&format!(
                "{} - {} of {}",
                page_start_index + 1,
                page_start_index + rows_in_page,
                rows_total
            ));
        }
        BTN_FIRST_PAGE => {
            let element = base_element.element();
            add_event_listener(element);
            element.set_inner_html("<< First page");
        }
        BTN_PREV_PAGE => {
            let element = base_element.element();
            add_event_listener(element);
            element.set_inner_html("< Previous page");
        }
        BTN_NEXT_PAGE => {
            let element = base_element.element();
            add_event_listener(element);
            element.set_inner_html("> Next page");
        }
        BTN_LAST_PAGE => {
            let element = base_element.element();
            add_event_listener(element);
            element.set_inner_html(">> Last page");
        }
        _ => {}
    }
}

fn add_event_listener(element: &Box<Element>) {
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

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "on_click event on element: {:?}",
        element.id()
    )));
}

#[cfg(test)]
mod tests {
    use crate::custom_elements::{
        base_element_trait::BaseElementTrait,
        data_table_controls_element::DataTableControlsSettings,
    };

    use super::DataTableControls;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generate_html() {
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let element = &crate::createElement("div");
        let parent_element = &element.attach_shadow(&shadow_init).unwrap();

        DataTableControls::new(
            DataTableControls::BASE_ID,
            &Some(DataTableControlsSettings::new(0, 10, 100)),
        )
        .render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1 - 10 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(
            DataTableControls::BASE_ID,
            &Some(DataTableControlsSettings::new(10, 10, 100)),
        )
        .render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">11 - 20 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(
            DataTableControls::BASE_ID,
            &Some(DataTableControlsSettings::new(20, 10, 100)),
        )
        .render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">21 - 30 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(
            DataTableControls::BASE_ID,
            &Some(DataTableControlsSettings::new(30, 10, 100)),
        )
        .render(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">31 - 40 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");
    }

    #[test]
    #[should_panic(
        expected = "please use the the constant `DataTableControls::BASE_ID` as id parameter"
    )]
    fn wrong_id() {
        DataTableControls::new("123", &Some(DataTableControlsSettings::new(20, 10, 100)));
    }
}
