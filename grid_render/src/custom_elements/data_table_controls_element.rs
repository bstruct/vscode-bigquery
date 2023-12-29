use web_sys::{Element, ShadowRoot};

use super::base_element::BaseElement;

pub(crate) struct DataTableControls;

impl DataTableControls {
    pub(crate) fn render_control(
        shadow: &ShadowRoot,
        number_of_rows_in_batch: usize,
        number_of_rows_total: usize,
        start_index: usize,
    ) {
        // let row_count = number_of_rows_in_batch;
        // let total_rows = crate::parse_to_usize(Some(query_response.total_rows.to_owned())).unwrap_or_default();

        // control"s background div
        let div = crate::createElement("div");
        div.set_id("controls-background");
        shadow.append_child(&div).unwrap();

        // control"s div
        let div = crate::createElement("div");
        div.set_id("controls");
        shadow.append_child(&div).unwrap();

        //span for page information
        let span_page_information = crate::createElement("span");
        span_page_information.set_id("paging");
        span_page_information.set_inner_html(&format!(
            "{} - {} of {}",
            start_index + 1,
            start_index + number_of_rows_in_batch,
            number_of_rows_total
        ));
        div.append_child(&span_page_information).unwrap();

        //first page
        let button = crate::createElement("button");
        button.set_inner_html("<< First page");
        // button.set_class_name("button");
        button.set_id("btn_first_page");
        div.append_child(&button).unwrap();

        // previous page
        let button = crate::createElement("button");
        button.set_inner_html("< Previous page");
        // button.set_class_name("button");
        button.set_id("btn_first_page");
        div.append_child(&button).unwrap();

        //next page
        let button = crate::createElement("button");
        button.set_inner_html("> Next page");
        // button.set_class_name("button");
        button.set_id("btn_next_page");
        div.append_child(&button).unwrap();

        // last page
        let button = crate::createElement("button");
        button.set_inner_html(">> Last page");
        // button.set_class_name("button");
        button.set_id("btn_last_page");
        div.append_child(&button).unwrap();
    }
}

fn create_self(parent_element: &Element) {


    BaseElement::new_and_append(parent_element, "div", "controls-background")
        .append_child("div", "controls")
        .append_child("span", "paging")
        ;

}

// #[cfg(test)]
// mod tests {
//     use wasm_bindgen_test::*;

//     use super::create_self;

//     wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     fn generate_html() {
      
//         let element = &crate::createElement("div");

//         create_self(element);

//         assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>");

//         create_self(element);

//         assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>");

//         create_self(element);

//         assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>");

//     }
// }
