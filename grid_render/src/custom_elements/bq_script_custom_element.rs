use web_sys::Element;

use super::{base_element::BaseElement, base_element_trait::BaseElementTrait};

const TAG_NAME: &'static str = "bq-script";

pub(crate) struct BigqueryScriptCustomElement {
    element: Option<Element>,
    element_id: String,
    job_id: String,
    project_id: String,
    location: String,
    token: String,
}

impl BigqueryScriptCustomElement{
    pub(crate) fn base_new(
        element_id: String,
        job_id: String,
        project_id: String,
        location: String,
        token: String,
    ) -> BigqueryScriptCustomElement {
        BigqueryScriptCustomElement {
            element: None,
            element_id,
            job_id,
            project_id,
            location,
            token
        }
    }
}

impl BaseElementTrait for BigqueryScriptCustomElement {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/bqscript.css"));

        BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            // .apply_fn(&set_attributes, self)
            // .append_shadow()
            .append_child_style(css_content, "style1")
            // .append_sibling("div", "spacer")
            // .apply_fn(&configure_spacer, &None)
            // .append_sibling_base_element(&self.to_data_table_controls())
            // .append_sibling_base_element(&self.to_data_table("t1"))
    }
}