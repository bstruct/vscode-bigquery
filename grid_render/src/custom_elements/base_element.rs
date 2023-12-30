use wasm_bindgen::JsValue;
use web_sys::{Element, Node, ShadowRoot};

pub(crate) struct BaseElement {
    id: String,
    element: Box<Element>,
}

impl BaseElement {
    pub(crate) fn id(&self) -> &String {
        &self.id
    }

    pub(crate) fn element(&self) -> &Box<Element> {
        &self.element
    }

    pub(crate) fn new_and_shadow_append(
        element: &ShadowRoot,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        BaseElement::new_and_append(
            &|| element.query_selector(&format!(":host > [be_id='{0}']", base_element_id)),
            &|node: &Node| element.append_child(node),
            tag_name,
            base_element_id,
        )
    }

    pub(crate) fn new_and_element_append(
        element: &Element,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        BaseElement::new_and_append(
            &|| element.query_selector(&format!(":scope > [be_id='{0}']", base_element_id)),
            &|node: &Node| element.append_child(node),
            tag_name,
            base_element_id,
        )
    }

    fn new_and_append(
        query_selector: &dyn Fn() -> Result<Option<Element>, JsValue>,
        append_child: &dyn Fn(&Node) -> Result<Node, JsValue>,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        let existing_element = query_selector();

        assert!(existing_element.is_ok());

        if let Some(existing_element) = existing_element.unwrap() {
            BaseElement::from(&existing_element)
        } else {
            let new_element = BaseElement::new(tag_name, base_element_id);
            append_child(&new_element.element).unwrap();
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
        BaseElement::append::<usize>(
            &self,
            tag_name,
            base_element_id,
            &|| self.element.first_element_child(),
            &|| self.element.to_owned(),
            None,
            None,
        )
    }

    pub fn append_child_fn<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        element_fn: &dyn Fn(&BaseElement, Option<T>),
        fn_parameter: Option<T>,
    ) -> BaseElement {
        BaseElement::append(
            &self,
            tag_name,
            base_element_id,
            &|| self.element.first_element_child(),
            &|| self.element.to_owned(),
            Some(element_fn),
            fn_parameter,
        )
    }

    pub fn append_sibling(&self, tag_name: &str, base_element_id: &str) -> BaseElement {
        BaseElement::append::<usize>(
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
            None,
            None,
        )
    }

    pub fn append_sibling_fn<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        element_fn: &dyn Fn(&BaseElement, Option<T>),
        fn_parameter: Option<T>,
    ) -> BaseElement {
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
            Some(element_fn),
            fn_parameter,
        )
    }

    fn append<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        get_element: &dyn Fn() -> Option<Element>,
        get_parent: &dyn Fn() -> Box<Element>,
        element_fn: Option<&dyn Fn(&BaseElement, Option<T>)>,
        fn_parameter: Option<T>,
    ) -> BaseElement {
        if let Some(existing_element) = get_element() {
            let existing_element_be_id = existing_element
                .get_attribute("be_id")
                .expect("not a base element");

            assert_eq!(existing_element_be_id, base_element_id);

            let existing_element = BaseElement::from(&existing_element);
            if let Some(element_fn) = element_fn {
                element_fn(&existing_element, fn_parameter);
            }
            existing_element
        } else {
            let new_element = BaseElement::new(tag_name, base_element_id);
            get_parent().append_child(&new_element.element).unwrap();
            if let Some(element_fn) = element_fn {
                element_fn(&new_element, fn_parameter);
            }
            new_element
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::custom_elements::base_element::BaseElement;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generate_html_1() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging");

        assert_eq!(&element.outer_html(), expected_output);
    }

    #[wasm_bindgen_test]
    fn generate_html_2() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span><button be_id=\"btn_first_page\"></button></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page");

        assert_eq!(&element.outer_html(), expected_output);
    }

    #[wasm_bindgen_test]
    fn generate_html_3() {
        let expected_output = "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\"></span><button be_id=\"btn_first_page\"></button><button be_id=\"btn_previous_page\"></button></div></div></div>";
        let element = &crate::createElement("div");

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .append_sibling("button", "btn_first_page")
            .append_sibling("button", "btn_previous_page");

        assert_eq!(&element.outer_html(), expected_output);
    }

    #[wasm_bindgen_test]
    fn generate_html_with_function_1() {
        let element = &crate::createElement("div");

        let test_number = Some(1);

        let test_f = &|base_element: &BaseElement, number: Option<i32>| {
            base_element
                .element
                .set_inner_html(&format!("{}", number.unwrap_or(10)));
        };

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", "paging", test_f, test_number);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     ": {:?}",
        //     &element.outer_html()
        // )));

        assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1</span></div></div></div>");

        let test_number = Some(2);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", "paging", test_f, test_number);

        assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">2</span></div></div></div>");

        let test_number = Some(3);

        BaseElement::new_and_element_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", "paging", test_f, test_number);

        assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">3</span></div></div></div>");
    }

    #[wasm_bindgen_test]
    fn generate_html_with_function_2() {
        let element = &crate::createElement("div");
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let parent_element = element.attach_shadow(&shadow_init)
            .unwrap();

        let test_number = Some(1);

        let test_f = &|base_element: &BaseElement, number: Option<i32>| {
            base_element
                .element
                .set_inner_html(&format!("{}", number.unwrap_or(10)));
        };

        BaseElement::new_and_shadow_append(&parent_element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", "paging", test_f, test_number);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     ": {:?}",
        //     &element.outer_html()
        // )));

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1</span></div></div>");

        let test_number = Some(2);

        BaseElement::new_and_shadow_append(&parent_element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child_fn("span", "paging", test_f, test_number);

        assert_eq!(&parent_element.inner_html(), "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">2</span></div></div>");

        // let test_number = Some(3);

        // BaseElement::new_and_shadow_append(&parent_element, "div", "controls-background")
        //     .append_child("div", "controls")
        //     .append_child_fn("span", "paging", test_f, test_number);

        // assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">3</span></div></div></div>");
    }

    #[wasm_bindgen_test]
    fn query_element_test() {
        let element = &crate::createElement("div");

        BaseElement::new_and_element_append(element, "div", "c0")
            .append_child("div", "c1")
            .append_child("div", "c2")
            .append_child("div", "c3")
            ;

            let c0 = element.query_selector(":scope > [be_id='c0']").unwrap();
            assert!(c0.is_some());
            let c1 = element.query_selector(":scope > [be_id='c1']").unwrap();
            assert!(!c1.is_some());
            let c2 = element.query_selector(":scope > [be_id='c2']").unwrap();
            assert!(!c2.is_some());
            let c3 = element.query_selector(":scope > [be_id='c3']").unwrap();
            assert!(!c3.is_some());

    }

    #[wasm_bindgen_test]
    fn query_shadow_test() {
        let element = &crate::createElement("div");
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let parent_element = element.attach_shadow(&shadow_init)
            .unwrap();

        BaseElement::new_and_shadow_append(&parent_element, "div", "c0")
            .append_child("div", "c1")
            .append_child("div", "c2")
            .append_child("div", "c3")
            ;

            //https://chromium.googlesource.com/chromium/blink/+/36dc1140906c0a8dc074df283029cb9d6ade46a9/LayoutTests/fast/dom/shadow/querySelector-with-shadow-all-and-shadow-deep.html#53
            let c0 = parent_element.query_selector(":host > [be_id='c0']").unwrap();
            assert!(c0.is_some());
            let c1 = parent_element.query_selector(":host > [be_id='c1']").unwrap();
            assert!(!c1.is_some());
            let c2 = parent_element.query_selector(":host > [be_id='c2']").unwrap();
            assert!(!c2.is_some());
            let c3 = parent_element.query_selector(":host > [be_id='c3']").unwrap();
            assert!(!c3.is_some());

    }
}
