pub(crate) mod bq_table_custom_element;
pub(crate) mod bq_query_custom_element;
mod bq_to_table;
mod custom_element_definition;
mod data_table_controls_element;
pub(crate) mod data_table_element;
// pub(crate) mod data_table_shadow_element;
mod base_element;
pub(crate) mod base_element_trait;

use self::custom_element_definition::CustomElementDefinition;
use std::{error::Error, fmt, str::FromStr};
use wasm_bindgen::prelude::*;

/**
ADD NEW COMPONENT STEP 1: add the name and respective HTML tag to the enum
*/
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum CustomElement {
    BqTable = "bq-table",
    BqQuery = "bq-query",
}

impl CustomElement {
    /**
    ADD NEW COMPONENT STEP 2: make it available on the full list
    */
    pub fn get_all() -> Vec<CustomElement> {
        vec![CustomElement::BqTable, CustomElement::BqQuery]
    }

    /**
    ADD NEW COMPONENT STEP 3: create new match line for the component to be defined
    */
    pub fn define_custom_component(
        &self,
        element: &web_sys::Element,
    ) -> Result<(), Box<dyn Error>> {
        let window = &web_sys::window().expect("no window exists");
        let document = &window.document().expect("window should have a document");

        match self {
            CustomElement::BqQuery => {
                bq_query_custom_element::BigqueryQueryCustomElement::define(document, element)
            }
            CustomElement::BqTable => {
                bq_table_custom_element::BigqueryTableCustomElement::define(document, element)
            }
            _ => eprintln!("definition for custom element not found"),
        };

        Ok(())
    }
}

impl fmt::Display for CustomElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl FromStr for CustomElement {
    type Err = ();

    fn from_str(input: &str) -> Result<CustomElement, Self::Err> {
        for custom_element in CustomElement::get_all() {
            if custom_element.to_str() == input {
                return Ok(custom_element);
            }
        }

        Err(())
    }
}
