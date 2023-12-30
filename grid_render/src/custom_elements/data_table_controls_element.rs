use super::base_element::BaseElement;
use web_sys::ShadowRoot;

pub(crate) struct DataTableControls {
    page_start_index: usize,
    rows_in_page: usize,
    rows_total: usize,
}

const PAGING: &str = "paging";
const BTN_FIRST_PAGE: &str = "btn_first_page";
const BTN_PREV_PAGE: &str = "btn_prev_page";
const BTN_NEXT_PAGE: &str = "btn_next_page";
const BTN_LAST_PAGE: &str = "btn_last_page";

impl DataTableControls {
    pub(crate) fn new(
        page_start_index: usize,
        rows_in_page: usize,
        rows_total: usize,
    ) -> DataTableControls {
        DataTableControls {
            page_start_index,
            rows_in_page,
            rows_total,
        }
    }

    pub(crate) fn render_control(&self, parent_element: &ShadowRoot) {
        let parameter_1 = Some(self);

        BaseElement::new_and_shadow_append(parent_element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", PAGING, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_FIRST_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_PREV_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_NEXT_PAGE, &modify_controls, parameter_1)
            .append_sibling_fn("button", BTN_LAST_PAGE, &modify_controls, parameter_1);
    }
}

fn modify_controls(base_element: &BaseElement, values: Option<&DataTableControls>) {

    let rows_in_page = values.unwrap().rows_in_page;
    let rows_total = values.unwrap().rows_total;
    let page_start_index = values.unwrap().page_start_index;

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
            base_element.element().set_inner_html("<< First page");
        }
        BTN_PREV_PAGE => {
            base_element.element().set_inner_html("< Previous page");
        }
        BTN_NEXT_PAGE => {
            base_element.element().set_inner_html("> Next page");
        }
        BTN_LAST_PAGE => {
            base_element.element().set_inner_html(">> Last page");
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::DataTableControls;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generate_html() {
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let element = &crate::createElement("div");
        let parent_element = &element.attach_shadow(&shadow_init).unwrap();

        DataTableControls::new(0, 10, 100).render_control(parent_element);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     "data_table_controls_element: {:?}",
        //     &parent_element.inner_html()
        // )));

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1 - 10 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

        DataTableControls::new(10, 10, 100).render_control(parent_element);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">11 - 20 of 100</span><button be_id=\"btn_first_page\">&lt;&lt; First page</button><button be_id=\"btn_prev_page\">&lt; Previous page</button><button be_id=\"btn_next_page\">&gt; Next page</button><button be_id=\"btn_last_page\">&gt;&gt; Last page</button></div></div>");

    }
}
