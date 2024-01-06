use super::{
    base_element_trait::BaseElementTrait,
    custom_element_definition::CustomElementDefinition,
    data_table_controls_element::DataTableControls,
    data_table_element::{DataTable, DataTableItem},
};
use crate::{
    bigquery::jobs::GetQueryResultsRequest, custom_elements::base_element::BaseElement,
    parse_to_usize,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

const TAG_NAME: &'static str = "bq-table";

pub(crate) struct BigqueryTableCustomElement {
    element_id: String,
    job_id: String,
    project_id: String,
    location: String,
    token: String,

    page_start_index: Option<usize>,
    page_size: usize,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,

    header: Option<Vec<String>>,
    rows: Option<Vec<Vec<Option<DataTableItem>>>>,
}

impl BigqueryTableCustomElement {
    pub(crate) fn base_new(
        element_id: String,
        job_id: String,
        project_id: String,
        location: String,
        token: String,
    ) -> BigqueryTableCustomElement {
        BigqueryTableCustomElement {
            element_id,
            job_id,
            project_id,
            location,
            token,

            page_start_index: None,
            page_size: 50,
            rows_in_page: None,
            rows_total: None,

            header: None,
            rows: None,
        }
    }
}

impl CustomElementDefinition for BigqueryTableCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::Element) {
        let on_event_type_closure =
            Closure::wrap(Box::new(BigqueryTableCustomElement::on_render_table)
                as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                "render_table",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl BaseElementTrait for BigqueryTableCustomElement {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));

        BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            .apply_fn(&set_attributes, self)
            .append_shadow()
            .append_child_style(css_content, "style1")
            .append_sibling_base_element(&self.to_data_table_controls())
            .append_sibling_base_element(&self.to_data_table("t1"))
    }
}

fn set_attributes(base_element: &BaseElement, bq_table: &BigqueryTableCustomElement) {
    let element = base_element.element();
    element.set_id(&bq_table.element_id);

    set_attribute(&element, "job_id", bq_table.job_id.as_str());
    set_attribute(&element, "project_id", bq_table.project_id.as_str());
    set_attribute(&element, "location", bq_table.location.as_str());
    set_attribute(&element, "token", bq_table.token.as_str());
    set_optional_attribute(&element, "page_start_index", &bq_table.page_start_index);
    set_attribute(&element, "page_size", &bq_table.page_size.to_string());
    set_optional_attribute(&element, "rows_in_page", &bq_table.rows_in_page);
    set_optional_attribute(&element, "rows_total", &bq_table.rows_total);
}

fn set_attribute(element: &web_sys::Element, attribute_name: &str, value: &str) {
    element.set_attribute(attribute_name, value).unwrap();
}

fn set_optional_attribute(element: &web_sys::Element, attribute_name: &str, value: &Option<usize>) {
    if value.is_some() {
        element
            .set_attribute(attribute_name, &value.unwrap().to_string())
            .unwrap();
    } else {
        element.remove_attribute(attribute_name).unwrap();
    }
}

impl BigqueryTableCustomElement {
    pub(crate) fn to_data_table_controls(&self) -> DataTableControls {
        DataTableControls::new(self.page_start_index, self.rows_in_page, self.rows_total)
    }

    pub(crate) fn to_data_table(&self, element_id: &str) -> DataTable {
        DataTable::new(element_id, &self.header, &self.rows)
    }

    pub(super) fn with_table_info(
        &self,
        rows_in_page: Option<usize>,
        rows_total: Option<usize>,
        header: Option<Vec<String>>,
        rows: Option<Vec<Vec<Option<DataTableItem>>>>,
    ) -> BigqueryTableCustomElement {
        BigqueryTableCustomElement {
            element_id: self.get_element_id().to_string(),
            job_id: self.job_id.to_string(),
            project_id: self.project_id.to_string(),
            location: self.location.to_string(),
            token: self.token.to_string(),
            page_start_index: self.page_start_index.clone(),
            page_size: self.page_size.clone(),
            rows_in_page,
            rows_total,
            header,
            rows,
        }
    }

    pub(crate) fn from_element(element: &Element) -> BigqueryTableCustomElement {
        let element_id = BaseElement::from_element(element)
            .id()
            .as_ref()
            .unwrap()
            .to_string();

        BigqueryTableCustomElement {
            element_id,
            job_id: get_attribute(element, "job_id"),
            project_id: get_attribute(element, "project_id"),
            location: get_attribute(element, "location"),
            token: get_attribute(element, "token"),
            page_start_index: get_opt_num_attribute(element, "page_start_index"),
            page_size: get_num_attribute(element, "page_size"),
            rows_in_page: get_opt_num_attribute(element, "rows_in_page"),
            rows_total: get_opt_num_attribute(element, "rows_total"),
            header: None,
            rows: None,
        }
    }

    fn as_query_results_request(&self) -> GetQueryResultsRequest {
        GetQueryResultsRequest {
            project_id: self.project_id.clone(),
            job_id: self.job_id.clone(),
            start_index: Some(self.page_start_index.clone().unwrap_or(0).to_string()),
            page_token: None,
            max_results: Some(self.page_size),
            timeout_ms: None,
            location: Some(self.location.clone()),
        }
    }

    fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap();

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "on_render_table event on element: {:?}",
            element.id()
        )));

        let bq_table_element = BigqueryTableCustomElement::from_element(&element);
        let jobs = crate::bigquery::jobs::Jobs::new(&bq_table_element.token);
        let request = bq_table_element.as_query_results_request();

        spawn_local(async move {
            let response = jobs.get_query_results(request).await;
            if let Some(response) = response {
                response.to_bq_table(&bq_table_element).render(&element);
            } else {
                element.set_inner_html(&format!("unexpected response: {:?}", response));
            }
        });
    }
}

fn get_attribute(element: &Element, attribute_name: &str) -> String {
    let att = element.get_attribute(attribute_name);
    assert!(att.is_some(), "attribute not found: {}", attribute_name);
    att.unwrap()
}

fn get_opt_num_attribute(element: &Element, attribute_name: &str) -> Option<usize> {
    parse_to_usize(element.get_attribute(attribute_name))
}

fn get_num_attribute(element: &Element, attribute_name: &str) -> usize {
    match parse_to_usize(element.get_attribute(attribute_name)) {
        Some(num) => num,
        None => panic!("attribute not found: {attribute_name}"),
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::{set_attributes, BigqueryTableCustomElement};
    use crate::custom_elements::{
        base_element_trait::BaseElementTrait, bq_table_custom_element::TAG_NAME,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn attributes_set_and_read() {
        let parent_node = &crate::createElement(super::TAG_NAME);
        let bq_table = &BigqueryTableCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
        );
        let base_element = &bq_table.render(parent_node);

        set_attributes(base_element, bq_table);

        let bq_table_from = BigqueryTableCustomElement::from_element(&base_element.element());

        assert_eq!(bq_table.job_id, bq_table_from.job_id);
        assert_eq!(bq_table.project_id, bq_table_from.project_id);
        assert_eq!(bq_table.location, bq_table_from.location);
        assert_eq!(bq_table.token, bq_table_from.token);
        assert_eq!(bq_table.page_start_index, bq_table_from.page_start_index);
        assert_eq!(bq_table.page_size, bq_table_from.page_size);
        assert_eq!(bq_table.rows_in_page, bq_table_from.rows_in_page);
        assert_eq!(bq_table.rows_total, bq_table_from.rows_total);
    }

    #[wasm_bindgen_test]
    pub fn render_test_1() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryTableCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
        );

        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_table_information = complex_object_array_test.to_bq_table(bq_table);

        let rows_in_page = bq_table_information.rows_in_page;
        let rows_total = bq_table_information.rows_total;
        let header = bq_table_information.header;
        let rows = bq_table_information.rows;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, header, rows);
        bq_table.render(parent_node);

        let c = parent_node.first_child().unwrap();
        assert_eq!(c.node_type(), web_sys::Node::ELEMENT_NODE);
        let element: web_sys::Element = wasm_bindgen::JsCast::dyn_into(c.value_of())
            .expect("unexpected error on casting Node to Element");
        assert_eq!(element.tag_name().to_lowercase(), TAG_NAME);

        assert!(element.shadow_root().is_some());
        let shadow = element.shadow_root().unwrap();

        let c = shadow.first_element_child().unwrap();
        assert_eq!(c.tag_name().to_lowercase(), "style");

        let c = c.next_element_sibling().unwrap();
        assert_eq!(c.tag_name().to_lowercase(), "div");
        assert_eq!(c.get_attribute("be_id").unwrap(), "controls-background");

        let c = c.next_element_sibling().unwrap();
        assert_eq!(c.tag_name().to_lowercase(), "table");

        // assert_eq!(c.outer_html(), "...");
    }
}
