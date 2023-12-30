
use serde::de::value;
use web_sys::Element;

pub(crate) struct BaseElement {
    id: String,
    element: Box<Element>,
}

impl BaseElement {
    pub(crate) fn new_and_append(
        element: &Element,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        let query = &format!(":scope > [be_id='{0}']", base_element_id);
        let existing_element = element.query_selector(query);

        assert!(existing_element.is_ok());

        if let Some(existing_element) = existing_element.unwrap() {
            BaseElement::from(&existing_element)
        } else {
            let new_element = BaseElement::new(tag_name, base_element_id);
            element.append_child(&new_element.element).unwrap();
            new_element
        }
    }

    fn new(tag_name: &str, base_element_id: &str) -> BaseElement {
        let element = &crate::createElement(tag_name);
        element.set_attribute("be_id", base_element_id).unwrap();
        BaseElement {
            id: base_element_id.to_owned(),
            element: Box::new(element.to_owned()),
        }
    }

    pub fn from(element: &Element) -> BaseElement {
        let id = element.get_attribute("be_id").expect("not a base element");
        BaseElement {
            id,
            element: Box::new(element.to_owned()),
        }
    }

    pub fn append_child(&self, tag_name: &str, base_element_id: &str) -> BaseElement {
        BaseElement::append(
            &self,
            tag_name,
            base_element_id,
            &|| self.element.first_element_child(),
            &|| self.element.to_owned(),
        )
    }

    pub fn append_child_f<T>(&self, 
        tag_name: &str, 
        base_element_id: &str,
        funct: &dyn Fn(&BaseElement, T),
        value:T
    ) -> BaseElement {


        funct(self, value);


        BaseElement::append(
            &self,
            tag_name,
            base_element_id,
            &|| self.element.first_element_child(),
            &|| self.element.to_owned(),
        )
    }

    pub fn append_sibling(&self, tag_name: &str, base_element_id: &str) -> BaseElement {
        BaseElement::append(
            &self,
            tag_name,
            base_element_id,
            &|| self.element.next_element_sibling(),
            &|| {
                Box::new(
                    self.element
                        .parent_element()
                        .expect("parent element not found"),
                )
            },
        )
    }

    fn append(
        &self,
        tag_name: &str,
        base_element_id: &str,
        get_element: &dyn Fn() -> Option<Element>,
        get_parent: &dyn Fn() -> Box<Element>,
    ) -> BaseElement {
        if let Some(existing_element) = get_element() {
            let existing_element_be_id = existing_element
                .get_attribute("be_id")
                .expect("not a base element");

            assert_eq!(existing_element_be_id, base_element_id);

            BaseElement::from(&existing_element)
        } else {
            let new_element = BaseElement::new(tag_name, base_element_id);
            get_parent().append_child(&new_element.element).unwrap();
            new_element
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{*, __rt::__wbgtest_console_info};

    use crate::custom_elements::base_element::BaseElement;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generate_html_1() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);
    }

    #[wasm_bindgen_test]
    fn generate_html_2() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span><button be_id=\"btn_first_page\"></button></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);
    }

    #[wasm_bindgen_test]
    fn generate_html_3() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span><button be_id=\"btn_first_page\"></button><button be_id=\"btn_previous_page\"></button></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);
    }


    #[wasm_bindgen_test]
    fn generate_html_with_function_1() {
        // let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>";
        let element = &crate::createElement("div");


        let test_number = 1;


        let test_f = &|base_element: &BaseElement, number: i32| {
            base_element.element.set_inner_html(&format!("{}", number));
        };



        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_f("span", "paging", test_f, test_number);


            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                ": {:?}",
                &element.outer_html()
            )));

        // assert_eq!(&element.outer_html(), expected_output);

        // BaseElement::new_and_append(element, "div", "controls-background")
        //     .append_child("div", "controls")
        //     .append_child("span", "paging");

        // assert_eq!(&element.outer_html(), expected_output);

        // BaseElement::new_and_append(element, "div", "controls-background")
        //     .append_child("div", "controls")
        //     .append_child("span", "paging");

        // assert_eq!(&element.outer_html(), expected_output);
    }
}
