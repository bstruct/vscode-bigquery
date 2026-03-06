use wasm_bindgen::JsValue;
use web_sys::{DocumentFragment, Element, Node, ShadowRoot};

use super::base_element_trait::BaseElementTrait;

pub(crate) struct BaseElement {
    id: Option<String>,
    node_type: u16,
    pub(crate) node: Box<Node>,
}

impl BaseElement {
    pub(crate) fn id(&self) -> &Option<String> {
        &self.id
    }

    pub(crate) fn element(&self) -> Element {
        assert_eq!(
            self.node_type,
            Node::ELEMENT_NODE,
            "node is not of the type element"
        );

        // &self.element
        let element: Element = wasm_bindgen::JsCast::dyn_into(self.node.value_of())
            .expect("unexpected error on casting Node to Element");

        element
    }

    pub(crate) fn node(&self) -> Node {
        assert_eq!(
            self.node_type,
            Node::DOCUMENT_FRAGMENT_NODE,
            "node is not of the type element"
        );

        // &self.element
        let element: Node = wasm_bindgen::JsCast::dyn_into(self.node.value_of())
            .expect("unexpected error on casting Node");

        element
    }

    pub(crate) fn apply_fn<T>(
        &self,
        element_fn: &dyn Fn(&BaseElement, &T),
        fn_parameter: &T,
    ) -> BaseElement {
        element_fn(self, fn_parameter);

        self.clone()
    }

    pub(crate) fn apply_class_name(&self, class_name: &str) -> BaseElement {
        self.element().set_class_name(class_name);

        self.clone()
    }

    #[doc = "Set the class of the html element if empty. So that there's action when creating the element, not on re-render."]
    pub(crate) fn apply_default_class_name(&self, class_name: &str) -> BaseElement {
        if self.element().class_name().len() == 0 {
            self.element().set_class_name(class_name);
        }

        self.clone()
    }

    pub(crate) fn append_shadow(&self) -> BaseElement {
        let element = self.element();

        if let Some(shadow_root) = element.shadow_root() {
            BaseElement::from_shadow(&shadow_root)
        } else {
            let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
            match element.attach_shadow(&shadow_init) {
                Ok(shadow_root) => BaseElement::from_shadow(&shadow_root),
                Err(e) => {
                    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                        "append_shadow: failed to attach shadow root: {:?}",
                        e
                    )));
                    BaseElement::from_element(&element)
                }
            }
        }
    }

    pub(crate) fn new_and_append(
        parent_node: &Node,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        match parent_node.node_type() {
            Node::DOCUMENT_FRAGMENT_NODE => {
                // web_sys::console::log_1(&JsValue::from(format!("DOCUMENT_FRAGMENT_NODE, {}, {}, {}", tag_name, base_element_id, parent_node.node_name())));

                let element: DocumentFragment =
                    match wasm_bindgen::JsCast::dyn_into(parent_node.value_of()) {
                        Ok(el) => el,
                        Err(_) => {
                            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                                "new_and_append: failed to cast Node to DocumentFragment",
                            ));
                            return BaseElement::create_element(tag_name, base_element_id);
                        }
                    };

                BaseElement::new_and_append_internal(
                    &|| element.query_selector(&format!(":host > [be_id='{0}']", base_element_id)),
                    &|node: &Node| element.append_child(node),
                    tag_name,
                    base_element_id,
                )
            }
            Node::ELEMENT_NODE => {
                let element: Element = match wasm_bindgen::JsCast::dyn_into(parent_node.value_of())
                {
                    Ok(el) => el,
                    Err(_) => {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                            "new_and_append: failed to cast Node to Element",
                        ));
                        return BaseElement::create_element(tag_name, base_element_id);
                    }
                };

                // web_sys::console::log_1(&JsValue::from(format!("ELEMENT_NODE, {}, {}, {}, {:?}", tag_name, base_element_id, parent_node.node_name(), element.get_attribute("be_id"))));
                BaseElement::new_and_append_internal(
                    &|| element.query_selector(&format!("[be_id='{0}']", base_element_id)),
                    &|node: &Node| element.append_child(node),
                    tag_name,
                    base_element_id,
                )
            }
            _ => {
                web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                    "new_and_append: unsupported node type {} for element '{}' — only element and shadow nodes are supported",
                    parent_node.node_type(),
                    base_element_id
                )));
                BaseElement::create_element(tag_name, base_element_id)
            }
        }
    }

    fn new_and_append_internal(
        query_selector: &dyn Fn() -> Result<Option<Element>, JsValue>,
        append_child: &dyn Fn(&Node) -> Result<Node, JsValue>,
        tag_name: &str,
        base_element_id: &str,
    ) -> BaseElement {
        match query_selector() {
            Ok(Some(existing_element)) => {
                // web_sys::console::log_1(&JsValue::from("existing_element exists"));
                BaseElement::from_element(&existing_element)
            }
            Ok(None) => {
                // web_sys::console::log_1(&JsValue::from("existing_element does NOT exists"));
                let new_element = BaseElement::create_element(tag_name, base_element_id);
                if let Err(e) = append_child(&new_element.element()) {
                    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                        "new_and_append_internal: failed to append child '{}': {:?}",
                        base_element_id, e
                    )));
                }
                new_element
            }
            Err(e) => {
                web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                    "new_and_append_internal: query_selector failed for '{}': {:?}",
                    base_element_id, e
                )));
                BaseElement::create_element(tag_name, base_element_id)
            }
        }
    }

    pub fn create_element(tag_name: &str, base_element_id: &str) -> BaseElement {
        let element = &crate::createElement(tag_name);
        if let Err(e) = element.set_attribute("be_id", base_element_id) {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "create_element: failed to set be_id '{}': {:?}",
                base_element_id, e
            )));
        }
        BaseElement {
            id: Some(base_element_id.to_owned()),
            node_type: element.node_type(),
            node: Box::new(element.to_owned().into()),
        }
    }

    pub fn from_element(element: &Element) -> BaseElement {
        let id = element.get_attribute("be_id").unwrap_or_else(|| {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "from_element: element is missing 'be_id' attribute",
            ));
            String::new()
        });
        BaseElement {
            id: Some(id),
            node_type: element.node_type(),
            node: Box::new(element.to_owned().into()),
        }
    }

    pub fn from_shadow(shadow_root: &ShadowRoot) -> BaseElement {
        BaseElement {
            id: None,
            node_type: shadow_root.node_type(),
            node: Box::new(shadow_root.to_owned().into()),
        }
    }

    /*
    When using this function, if the node is not of the type ELEMENT_NODE,
    it's not really possible to determine if it's a base element or not.
    ( the id is missing )
    */
    pub fn from_node(node: &Node) -> BaseElement {
        match node.node_type() {
            Node::ELEMENT_NODE => {
                match wasm_bindgen::JsCast::dyn_into::<Element>(node.value_of()) {
                    Ok(element) => BaseElement::from_element(&element),
                    Err(_) => {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                            "from_node: failed to cast ELEMENT_NODE to Element",
                        ));
                        BaseElement {
                            id: None,
                            node_type: node.node_type(),
                            node: Box::new(node.to_owned().into()),
                        }
                    }
                }
            }
            _ => BaseElement {
                id: None,
                node_type: node.node_type(),
                node: Box::new(node.to_owned().into()),
            },
        }
    }

    pub fn clone(&self) -> BaseElement {
        BaseElement {
            id: self.id.clone(),
            node_type: self.node_type,
            node: self.node.to_owned(),
        }
    }

    pub fn append_child(&self, tag_name: &str, base_element_id: &str) -> BaseElement {
        BaseElement::append::<usize>(
            &self,
            tag_name,
            base_element_id,
            &|| self.node.first_child(),
            &|| self.node.to_owned(),
            None,
            None,
        )
    }

    pub fn append_base_child(&self, base_element: &dyn BaseElementTrait) -> BaseElement {
        base_element.render(&self.node);

        //return the top level of the item just added.
        // otherwise will return the element that was added last in the render method
        // being inner element or not
        match self.node.last_child() {
            Some(child) => BaseElement::from_node(&child),
            None => {
                web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                    "append_base_child: parent node has no children after render",
                ));
                self.clone()
            }
        }
    }

    pub fn append_child_fn<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        element_fn: &dyn Fn(&BaseElement, &T),
        fn_parameter: &T,
    ) -> BaseElement {
        BaseElement::append_child(&self, tag_name, base_element_id)
            .apply_fn(element_fn, fn_parameter)
    }

    pub fn append_sibling(&self, tag_name: &str, base_element_id: &str) -> BaseElement {
        let get_parent = || -> Box<Node> {
            match self.node.parent_node() {
                Some(parent) => Box::new(parent),
                None => {
                    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                        "append_sibling: parent node not found for '{}'",
                        base_element_id
                    )));
                    self.node.to_owned()
                }
            }
        };
        BaseElement::append::<usize>(
            &self,
            tag_name,
            base_element_id,
            &|| self.node.next_sibling(),
            &get_parent,
            None,
            None,
        )
    }

    pub fn append_sibling_fn<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        element_fn: &dyn Fn(&BaseElement, &T),
        fn_parameter: &T,
    ) -> BaseElement {
        BaseElement::append_sibling(&self, tag_name, base_element_id)
            .apply_fn(element_fn, fn_parameter)
    }

    fn append<T>(
        &self,
        tag_name: &str,
        base_element_id: &str,
        get_node: &dyn Fn() -> Option<Node>,
        get_parent: &dyn Fn() -> Box<Node>,
        element_fn: Option<&dyn Fn(&BaseElement, T)>,
        fn_parameter: Option<T>,
    ) -> BaseElement {
        if let Some(existing_node) = get_node() {
            if existing_node.node_type() == Node::ELEMENT_NODE {
                let existing_element: Element =
                    match wasm_bindgen::JsCast::dyn_into(existing_node.value_of()) {
                        Ok(el) => el,
                        Err(_) => {
                            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                                "append: failed to cast existing node to Element",
                            ));
                            return self.clone();
                        }
                    };

                let existing_element_be_id = existing_element
                    .get_attribute("be_id")
                    .unwrap_or_default();

                if existing_element_be_id != base_element_id {
                    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                        "append: be_id mismatch: expected '{}', found '{}'",
                        base_element_id, existing_element_be_id
                    )));
                }

                let existing_element = BaseElement::from_element(&existing_element);
                if let Some(element_fn) = element_fn {
                    if let Some(param) = fn_parameter {
                        element_fn(&existing_element, param);
                    }
                }
                return existing_element;
            }
        }

        let new_element = BaseElement::create_element(tag_name, base_element_id);
        if let Err(e) = get_parent().append_child(&new_element.element()) {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                "append: failed to append child '{}': {:?}",
                base_element_id, e
            )));
        }
        if let Some(element_fn) = element_fn {
            if let Some(param) = fn_parameter {
                element_fn(&new_element, param);
            }
        }
        new_element
    }

    pub(crate) fn append_child_style(
        &self,
        css_content: &str,
        base_element_id: &str,
    ) -> BaseElement {
        self.append_child("style", base_element_id).apply_fn(
            &|base_element: &BaseElement, value: &Option<&str>| {
                if let Some(css) = value {
                    base_element.element().set_inner_html(css);
                }
            },
            &Some(css_content),
        )
    }

    pub(crate) fn append_nodes(&self, render: &[website_component_table::HtmlNode]) {
        match self.node_type {
            Node::ELEMENT_NODE => {
                let element: Element =
                    match wasm_bindgen::JsCast::dyn_into(self.node.value_of()) {
                        Ok(el) => el,
                        Err(_) => {
                            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                                "append_nodes: failed to cast ELEMENT_NODE to Element",
                            ));
                            return;
                        }
                    };

                for item in render.iter().filter_map(|n| n.to_element_node().ok()) {
                    if let Err(e) = element.append_child(&item) {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                            "append_nodes: failed to append child to element: {:?}",
                            e
                        )));
                    }
                }
            }
            Node::DOCUMENT_FRAGMENT_NODE => {
                let document_fragment: DocumentFragment =
                    match wasm_bindgen::JsCast::dyn_into(self.node.value_of()) {
                        Ok(df) => df,
                        Err(_) => {
                            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                                "append_nodes: failed to cast DOCUMENT_FRAGMENT_NODE to DocumentFragment",
                            ));
                            return;
                        }
                    };

                for item in render.iter().filter_map(|n| n.to_element_node().ok()) {
                    if let Err(e) = document_fragment.append_child(&item) {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                            "append_nodes: failed to append child to document fragment: {:?}",
                            e
                        )));
                    }
                }
            }
            Node::DOCUMENT_NODE => {
                let document: web_sys::Document =
                    match wasm_bindgen::JsCast::dyn_into(self.node.value_of()) {
                        Ok(doc) => doc,
                        Err(_) => {
                            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                                "append_nodes: failed to cast DOCUMENT_NODE to Document",
                            ));
                            return;
                        }
                    };

                for item in render.iter().filter_map(|n| n.to_element_node().ok()) {
                    if let Err(e) = document.append_child(&item) {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                            "append_nodes: failed to append child to document: {:?}",
                            e
                        )));
                    }
                }
            }
            _ => {
                web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format!(
                    "append_nodes: unsupported node type {}",
                    self.node_type
                )));
            }
        }
    }

    // pub(crate) fn next_sibling(&self) -> Option<BaseElement> {
    //     match self.node.next_sibling() {
    //         Some(node) => Some(BaseElement::from_node(&node)),
    //         None => None,
    //     }
    // }

    // pub(crate) fn remove_child(&self, child_base_element: &BaseElement) {
    //     self.node.remove_child(&child_base_element.node).unwrap();
    // }

    // pub(crate) fn clear_content(&self) -> BaseElement {
    //     self.node.set_text_content(None);

    //     self.clone()
    // }
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
        let element = &crate::createElement("div");

        let test_number = &Some(1);

        let test_f = &|base_element: &BaseElement, number: &Option<i32>| {
            base_element
                .element()
                .set_inner_html(&format!("{}", number.unwrap_or(10)));
        };

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .apply_fn(test_f, test_number);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     ": {:?}",
        //     &element.outer_html()
        // )));

        assert_eq!(
            &element.outer_html(),
            "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1</span></div></div></div>"
        );

        let test_number = &Some(2);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .apply_fn(test_f, test_number);

        assert_eq!(
            &element.outer_html(),
            "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">2</span></div></div></div>"
        );

        let test_number = &Some(3);

        BaseElement::new_and_append(element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .apply_fn(test_f, test_number);

        assert_eq!(
            &element.outer_html(),
            "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">3</span></div></div></div>"
        );
    }

    #[wasm_bindgen_test]
    fn generate_html_with_function_2() {
        let element = &crate::createElement("div");
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let parent_element = element.attach_shadow(&shadow_init).unwrap();

        let test_number = &Some(1);

        let test_f = &|base_element: &BaseElement, number: &Option<i32>| {
            base_element
                .element()
                .set_inner_html(&format!("{}", number.unwrap_or(10)));
        };

        BaseElement::new_and_append(&parent_element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .apply_fn(test_f, test_number);

        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     ": {:?}",
        //     &element.outer_html()
        // )));

        assert_eq!(
            &parent_element.inner_html(),
            "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">1</span></div></div>"
        );

        let test_number = &Some(2);

        BaseElement::new_and_append(&parent_element, "div", "controls-background")
            .append_child("div", "controls")
            .append_child("span", "paging")
            .apply_fn(test_f, test_number);

        assert_eq!(
            &parent_element.inner_html(),
            "<div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">2</span></div></div>"
        );

        // let test_number = Some(3);

        // BaseElement::new_and_append(&parent_element, "div", "controls-background")
        //     .append_child("div", "controls")
        //     .append_child_fn("span", "paging", test_f, test_number);

        // assert_eq!(&element.outer_html(), "<div><div be_id=\"controls-background\"><div be_id=\"controls\"><span be_id=\"paging\">3</span></div></div></div>");
    }

    #[wasm_bindgen_test]
    fn query_element_test() {
        let element = &crate::createElement("div");

        BaseElement::new_and_append(element, "div", "c0")
            .append_child("div", "c1")
            .append_child("div", "c2")
            .append_child("div", "c3");

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
        let parent_element = element.attach_shadow(&shadow_init).unwrap();

        BaseElement::new_and_append(&parent_element, "div", "c0")
            .append_child("div", "c1")
            .append_child("div", "c2")
            .append_child("div", "c3");

        //https://chromium.googlesource.com/chromium/blink/+/36dc1140906c0a8dc074df283029cb9d6ade46a9/LayoutTests/fast/dom/shadow/querySelector-with-shadow-all-and-shadow-deep.html#53
        let c0 = parent_element
            .query_selector(":host > [be_id='c0']")
            .unwrap();
        assert!(c0.is_some());
        let c1 = parent_element
            .query_selector(":host > [be_id='c1']")
            .unwrap();
        assert!(!c1.is_some());
        let c2 = parent_element
            .query_selector(":host > [be_id='c2']")
            .unwrap();
        assert!(!c2.is_some());
        let c3 = parent_element
            .query_selector(":host > [be_id='c3']")
            .unwrap();
        assert!(!c3.is_some());
    }

    #[wasm_bindgen_test]
    fn node_test() {
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let element = &crate::createElement("div");
        let parent_element = &element.attach_shadow(&shadow_init).unwrap();

        let node: &web_sys::Node = parent_element;

        assert_eq!("#document-fragment", &node.node_name());
    }

    #[wasm_bindgen_test]
    fn node_test_2() {
        let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
        let element = &crate::createElement("div");
        let parent_element = &element.attach_shadow(&shadow_init).unwrap();

        let node: &web_sys::Node = parent_element;

        assert_eq!("#document-fragment", &node.node_name());
        assert_eq!(web_sys::Node::DOCUMENT_FRAGMENT_NODE, node.node_type());

        let document_fragment: web_sys::DocumentFragment =
            wasm_bindgen::JsCast::dyn_into(parent_element.value_of()).expect("msg");

        document_fragment
            .append_child(&crate::createElement("span"))
            .unwrap();
    }
}
