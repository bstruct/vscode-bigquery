use super::{
    base_element_trait::BaseElementTrait,
    bq_common_custom_element::{
        get_attribute, get_num_attribute, get_opt_attribute, get_opt_num_attribute,
        handle_page_nav_event, set_attribute, set_optional_attribute,
    },
    custom_element_definition::CustomElementDefinition,
    data_table_controls_element::{
        DataTableControls, EVENT_GO_TO_FIRST_PAGE, EVENT_GO_TO_LAST_PAGE, EVENT_GO_TO_NEXT_PAGE,
        EVENT_GO_TO_PREVIOUS_PAGE,
    },
};
use crate::{
    bigquery::jobs::{GetJobRequest, GetQueryResultsRequest, JobReference},
    custom_elements::base_element::BaseElement,
    set_state,
    utils::render_standalone,
};
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use website_component_table::{HtmlNodeRender, TableBuilder};

const TAG_NAME: &'static str = "bq-query";
const PAGE_START_INDEX_ATT: &str = "page_start_index";
const PAGE_SIZE_ATT: &str = "page_size";
const ROWS_IN_PAGE_ATT: &str = "rows_in_page";
const ROWS_TOTAL_ATT: &str = "rows_total";
pub(crate) const RENDER_QUERY_EVENT_NAME: &str = "render_table";

pub(crate) struct BigqueryQueryCustomElement {
    element: Option<Element>,
    element_id: String,
    job_id: String,
    project_id: String,
    location: String,
    token: String,
    statement_type: Option<String>,

    page_start_index: usize,
    page_size: usize,
    rows_in_page: Option<usize>,
    rows_total: Option<usize>,

    table_builder: Option<TableBuilder>,
}

impl BigqueryQueryCustomElement {
    pub(crate) fn base_new(
        element_id: String,
        job_id: String,
        project_id: String,
        location: String,
        token: String,
        statement_type: Option<String>,
    ) -> BigqueryQueryCustomElement {
        BigqueryQueryCustomElement {
            element: None,
            element_id,
            job_id,
            project_id,
            location,
            token,
            statement_type,

            page_start_index: 0,
            page_size: 50,
            rows_in_page: None,
            rows_total: None,

            table_builder: None,
        }
    }
    
    pub(crate) fn to_data_table_controls(&self) -> DataTableControls {
        DataTableControls::new(
            Some(self.page_start_index),
            self.rows_in_page,
            self.rows_total,
            Some(self.as_job_reference()),
            None,
        )
    }

    pub(crate) fn as_job_reference(&self) -> JobReference {
        JobReference {
            project_id: self.project_id.to_string(),
            job_id: self.job_id.to_string(),
            location: self.location.to_string(),
        }
    }

    pub(super) fn with_table_info(
        &self,
        rows_in_page: Option<usize>,
        rows_total: Option<usize>,
        table_builder: Option<TableBuilder>,
    ) -> BigqueryQueryCustomElement {
        // Only save state for top-level bq-query elements (i.e. run directly,
        // not as a child of bq-script).  Child bq-query elements live in the
        // light DOM of bq-script, so closest("bq-script") finds an ancestor.
        // When running inside a script, bq-script::render() already saved the
        // correct top-level job_id and we must not let child queries overwrite it.
        let is_child_of_script = self.element.as_ref()
            .and_then(|el| el.closest("bq-script").ok().flatten())
            .is_some();

        if !is_child_of_script {
            let state = serde_json::json!({
                "jobId":     self.job_id.to_string(),
                "projectId": self.project_id.to_string(),
                "location":  self.location.to_string(),
            });
            set_state(&state.to_string()).unwrap_or(());
        }

        BigqueryQueryCustomElement {
            element: self.element.to_owned(),
            element_id: self.get_element_id().to_string(),
            job_id: self.job_id.to_string(),
            project_id: self.project_id.to_string(),
            location: self.location.to_string(),
            token: self.token.to_string(),
            statement_type: self.statement_type.clone(),
            page_start_index: self.page_start_index.clone(),
            page_size: self.page_size.clone(),
            rows_in_page,
            rows_total,
            table_builder,
        }
    }

    pub(crate) fn from_element(element: &Element) -> Option<BigqueryQueryCustomElement> {
        let element_id = element.get_attribute("be_id")?;
        let job_id = element.get_attribute("job_id")?;
        let project_id = element.get_attribute("project_id")?;
        let location = element.get_attribute("location")?;
        let token = element.get_attribute("token")?;
        let page_size = get_opt_num_attribute(element, PAGE_SIZE_ATT)?;

        Some(BigqueryQueryCustomElement {
            element: Some(element.to_owned()),
            element_id,
            job_id,
            project_id,
            location,
            token,
            statement_type: get_opt_attribute(element, "statement_type"),
            page_start_index: get_opt_num_attribute(element, PAGE_START_INDEX_ATT).unwrap_or(0),
            page_size,
            rows_in_page: get_opt_num_attribute(element, ROWS_IN_PAGE_ATT),
            rows_total: get_opt_num_attribute(element, ROWS_TOTAL_ATT),
            table_builder: None,
        })
    }

    fn as_query_results_request(&self) -> GetQueryResultsRequest {
        GetQueryResultsRequest {
            project_id: self.project_id.clone(),
            job_id: self.job_id.clone(),
            start_index: Some(self.page_start_index.clone().to_string()),
            page_token: None,
            max_results: Some(self.page_size),
            timeout_ms: None,
            location: Some(self.location.clone()),
        }
    }

    fn as_job_request(&self) -> GetJobRequest {
        GetJobRequest {
            project_id: self.project_id.clone(),
            job_id: self.job_id.clone(),
            location: Some(self.location.clone()),
        }
    }

    fn on_render_query(event: &web_sys::Event) {
        let element = match event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            Some(e) => e,
            None => {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                    "on_render_query: event target is not an element",
                ));
                return;
            }
        };

        if !element.has_attribute("loaded") {
            if element.set_attribute("loaded", "1").is_err() {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                    "on_render_query: could not set 'loaded' attribute",
                ));
                return;
            }

            let bq_query_element = match BigqueryQueryCustomElement::from_element(&element) {
                Some(e) => e,
                None => {
                    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                        "on_render_query: element is missing required attributes",
                    ));
                    return;
                }
            };
            let is_dml_statement = bq_query_element.is_dml_statement();

            let jobs = crate::bigquery::jobs::Jobs::new(&bq_query_element.token);
            let parent_node = match element.parent_element() {
                Some(p) => p,
                None => {
                    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                        "on_render_query: bq-query element has no parent element",
                    ));
                    return;
                }
            };

            if is_dml_statement {
                let request = bq_query_element.as_job_request();

                spawn_local(async move {
                    match jobs.get(request).await {
                        Some(response) => {
                            if response.has_error() {
                                render_standalone(&response.to_error_table(), &parent_node);
                            } else {
                                render_standalone(&response.to_dml_table(), &parent_node);
                            }
                        }
                        None => {
                            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                                "on_render_query: job get returned None",
                            ));
                        }
                    }
                });
            } else {
                let request = bq_query_element.as_query_results_request();

                spawn_local(async move {
                    match jobs.get_query_results(request).await {
                        Some(response) => {
                            response.to_bq_query(&bq_query_element).render(&parent_node);
                        }
                        None => {}
                    }
                });
            }
        }
    }

    pub(crate) fn get_page_start_index(&self) -> usize {
        self.page_start_index
    }

    pub(crate) fn first_page(&self) -> bool {
        let element = match self.element.as_ref() {
            Some(e) => e,
            None => return false,
        };

        let previous_value = element.get_attribute(PAGE_START_INDEX_ATT);
        if element.set_attribute(PAGE_START_INDEX_ATT, "0").is_err() {
            return false;
        }
        let current_value = element.get_attribute(PAGE_START_INDEX_ATT);

        if previous_value != current_value {
            let _ = element.remove_attribute("loaded");
        }

        previous_value != current_value
    }

    pub(crate) fn previous_page(&self) -> bool {
        let element = match self.element.as_ref() {
            Some(e) => e,
            None => return false,
        };
        let start_index = get_opt_num_attribute(element, PAGE_START_INDEX_ATT).unwrap_or(0);
        let page_size = get_opt_num_attribute(element, PAGE_SIZE_ATT).unwrap_or(50);

        let new_value = if start_index > page_size {
            start_index - page_size
        } else {
            0
        };

        let previous_value = element.get_attribute(PAGE_START_INDEX_ATT);
        if element
            .set_attribute(PAGE_START_INDEX_ATT, &format!("{0}", new_value))
            .is_err()
        {
            return false;
        }
        let current_value = element.get_attribute(PAGE_START_INDEX_ATT);

        if previous_value != current_value {
            let _ = element.remove_attribute("loaded");
        }

        previous_value != current_value
    }

    pub(crate) fn next_page(&self) -> bool {
        let element = match self.element.as_ref() {
            Some(e) => e,
            None => return false,
        };
        let rows_total = match get_opt_num_attribute(element, ROWS_TOTAL_ATT) {
            Some(r) => r,
            None => return false,
        };

        let start_index = get_opt_num_attribute(element, PAGE_START_INDEX_ATT).unwrap_or(0);
        let page_size = get_opt_num_attribute(element, PAGE_SIZE_ATT).unwrap_or(50);

        let new_value = if start_index + page_size >= rows_total {
            start_index
        } else {
            start_index + page_size
        };

        let previous_value = element.get_attribute(PAGE_START_INDEX_ATT);
        if element
            .set_attribute(PAGE_START_INDEX_ATT, &format!("{0}", new_value))
            .is_err()
        {
            return false;
        }
        let current_value = element.get_attribute(PAGE_START_INDEX_ATT);

        if previous_value != current_value {
            let _ = element.remove_attribute("loaded");
        }

        previous_value != current_value
    }

    pub(crate) fn last_page(&self) -> bool {
        let element = match self.element.as_ref() {
            Some(e) => e,
            None => return false,
        };
        let page_size = get_opt_num_attribute(element, PAGE_SIZE_ATT).unwrap_or(50);
        let rows_total = get_opt_num_attribute(element, ROWS_TOTAL_ATT).unwrap_or(0);

        let new_value = if page_size > rows_total {
            0
        } else {
            let start_index =
                (f64::floor((rows_total as f64) / page_size as f64) * page_size as f64) as usize;
            if start_index == rows_total {
                start_index - page_size
            } else {
                start_index
            }
        };

        let previous_value = element.get_attribute(PAGE_START_INDEX_ATT);
        if element
            .set_attribute(PAGE_START_INDEX_ATT, &format!("{0}", new_value))
            .is_err()
        {
            return false;
        }
        let current_value = element.get_attribute(PAGE_START_INDEX_ATT);

        if previous_value != current_value {
            let _ = element.remove_attribute("loaded");
        }

        previous_value != current_value
    }

    fn is_dml_statement(&self) -> bool {
        if let Some(statement_type) = &self.statement_type {
            let statement_type = statement_type.as_str();

            if ["INSERT", "UPDATE", "DELETE", "MERGE"].contains(&statement_type) {
                return true;
            }
        }

        false
    }
}

impl CustomElementDefinition for BigqueryQueryCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::Element) {
        let on_event_type_closure =
            Closure::wrap(Box::new(BigqueryQueryCustomElement::on_render_query)
                as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                RENDER_QUERY_EVENT_NAME,
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();

        //EVENT_GO_TO_FIRST_PAGE
        let on_event_type_closure =
            Closure::wrap(Box::new(first_page) as Box<dyn Fn(&web_sys::Event)>);
        element
            .add_event_listener_with_callback_and_bool(
                EVENT_GO_TO_FIRST_PAGE,
                on_event_type_closure.as_ref().unchecked_ref(),
                false,
            )
            .unwrap();
        on_event_type_closure.forget();

        //EVENT_GO_TO_PREVIOUS_PAGE
        let on_event_type_closure =
            Closure::wrap(Box::new(previous_page) as Box<dyn Fn(&web_sys::Event)>);
        element
            .add_event_listener_with_callback_and_bool(
                EVENT_GO_TO_PREVIOUS_PAGE,
                on_event_type_closure.as_ref().unchecked_ref(),
                false,
            )
            .unwrap();
        on_event_type_closure.forget();

        //EVENT_GO_TO_NEXT_PAGE
        let on_event_type_closure =
            Closure::wrap(Box::new(next_page) as Box<dyn Fn(&web_sys::Event)>);
        element
            .add_event_listener_with_callback(
                EVENT_GO_TO_NEXT_PAGE,
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        on_event_type_closure.forget();

        //EVENT_GO_TO_LAST_PAGE
        let on_event_type_closure =
            Closure::wrap(Box::new(last_page) as Box<dyn Fn(&web_sys::Event)>);
        element
            .add_event_listener_with_callback_and_bool(
                EVENT_GO_TO_LAST_PAGE,
                on_event_type_closure.as_ref().unchecked_ref(),
                false,
            )
            .unwrap();
        on_event_type_closure.forget();
    }
}

fn first_page(event: &web_sys::Event) {
    handle_page_nav_event(event, TAG_NAME, |e| BigqueryQueryCustomElement::from_element(e).map(|el| el.first_page()).unwrap_or(false), RENDER_QUERY_EVENT_NAME);
}

fn previous_page(event: &web_sys::Event) {
    handle_page_nav_event(event, TAG_NAME, |e| BigqueryQueryCustomElement::from_element(e).map(|el| el.previous_page()).unwrap_or(false), RENDER_QUERY_EVENT_NAME);
}

fn next_page(event: &web_sys::Event) {
    handle_page_nav_event(event, TAG_NAME, |e| BigqueryQueryCustomElement::from_element(e).map(|el| el.next_page()).unwrap_or(false), RENDER_QUERY_EVENT_NAME);
}

fn last_page(event: &web_sys::Event) {
    handle_page_nav_event(event, TAG_NAME, |e| BigqueryQueryCustomElement::from_element(e).map(|el| el.last_page()).unwrap_or(false), RENDER_QUERY_EVENT_NAME);
}

impl BaseElementTrait for BigqueryQueryCustomElement {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        let bq_query = BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            .apply_fn(&set_attributes, self);

        let shadow = bq_query.append_shadow();

        // Remove any loading placeholder (no be_id) added by on_click during navigation.
        while let Some(last) = shadow.node.last_child() {
            if last.node_type() == web_sys::Node::ELEMENT_NODE {
                if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(last.value_of()) {
                    if el.get_attribute("be_id").is_none() {
                        let _ = shadow.node.remove_child(&el);
                        continue;
                    }
                }
            }
            break;
        }

        let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));
        shadow.append_child_style(css_content, "style1");

        if let Some(table_builder) = &self.table_builder {
            if let Ok(render) = table_builder.render() {
                if self.is_dml_statement() {
                    shadow.append_nodes(&render);
                } else {
                    let _ = &self.to_data_table_controls().render(&shadow.node());
                    shadow.append_nodes(&render);
                }
            }
        }

        bq_query
    }
}

fn set_attributes(base_element: &BaseElement, bq_table: &BigqueryQueryCustomElement) {
    let element = base_element.element();
    element.set_id(&bq_table.element_id);

    set_attribute(&element, "job_id", bq_table.job_id.as_str());
    set_attribute(&element, "project_id", bq_table.project_id.as_str());
    set_attribute(&element, "location", bq_table.location.as_str());
    set_attribute(&element, "token", bq_table.token.as_str());
    set_attribute(
        &element,
        "statement_type",
        bq_table.statement_type.clone().unwrap_or_default().as_str(),
    );
    set_optional_attribute(
        &element,
        PAGE_START_INDEX_ATT,
        &Some(bq_table.page_start_index),
    );
    set_attribute(&element, PAGE_SIZE_ATT, &bq_table.page_size.to_string());
    set_optional_attribute(&element, ROWS_IN_PAGE_ATT, &bq_table.rows_in_page);
    set_optional_attribute(&element, ROWS_TOTAL_ATT, &bq_table.rows_total);
}


#[cfg(test)]
mod tests {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = document, js_name = "toLocaleString")]
        fn set_state(state_json: &str);
    }

    use super::{BigqueryQueryCustomElement, set_attributes};
    use crate::custom_elements::{
        base_element_trait::BaseElementTrait,
        bq_query_custom_element::{PAGE_START_INDEX_ATT, TAG_NAME},
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn attributes_set_and_read() {
        let parent_node = &crate::createElement(super::TAG_NAME);
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );
        let base_element = &bq_table.render(parent_node);

        set_attributes(base_element, bq_table);

        let bq_table_from = BigqueryQueryCustomElement::from_element(&base_element.element())
            .expect("test: element should have all required attributes");

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
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);
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
        assert_eq!(c.tag_name().to_lowercase(), "bstruct-table");

        // assert_eq!(c.outer_html(), "...");
    }

    #[wasm_bindgen_test]
    pub fn render_thrice_test_1() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test = include_str!("test_resources/all_types_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let rows_total = complex_object_array_test
            .clone()
            .total_rows
            .unwrap_or("0".to_string())
            .parse::<usize>()
            .unwrap_or(0);
        let rows_in_page = complex_object_array_test.rows.iter().len();
        let table_builder = complex_object_array_test.to_table_builder(1);

        let bq_table =
            bq_table.with_table_info(Some(rows_in_page), Some(rows_total), Some(table_builder));

        //1
        bq_table.render(parent_node);

        let first_html_output = parent_node.outer_html();

        //2 - render again
        bq_table.render(parent_node);

        assert_eq!(parent_node.outer_html(), first_html_output);

        //3 - render again
        bq_table.render(parent_node);

        assert_eq!(parent_node.outer_html(), first_html_output);

        append_to_body(&parent_node);
    }

    #[wasm_bindgen_test]
    pub fn last_page_test_1() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test = include_str!("test_resources/all_types_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);
        //1
        bq_table.render(parent_node);

        let element = &parent_node.first_element_child().unwrap();
        let bq_table = BigqueryQueryCustomElement::from_element(element)
            .expect("test: element should have all required attributes");

        bq_table.last_page();

        let page_start_index = element.get_attribute(PAGE_START_INDEX_ATT).unwrap();

        assert_eq!(page_start_index, "0");
    }

    #[wasm_bindgen_test]
    pub fn last_page_test_2() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);
        //1
        bq_table.render(parent_node);

        let element = &parent_node.first_element_child().unwrap();
        let bq_table = BigqueryQueryCustomElement::from_element(element)
            .expect("test: element should have all required attributes");

        bq_table.last_page();

        let page_start_index = element.get_attribute(PAGE_START_INDEX_ATT).unwrap();

        assert_eq!("989250", page_start_index);
    }

    #[wasm_bindgen_test]
    pub fn last_page_test_3() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test = include_str!("test_resources/100_rows.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);
        //1
        bq_table.render(parent_node);

        let element = &parent_node.first_element_child().unwrap();
        let bq_table = BigqueryQueryCustomElement::from_element(element)
            .expect("test: element should have all required attributes");

        bq_table.last_page();

        let page_start_index = element.get_attribute(PAGE_START_INDEX_ATT).unwrap();

        assert_eq!("50", page_start_index);
    }

    #[test]
    fn test_last_page() {
        assert_eq!(
            51.0,
            (f64::floor(((100 as f64) - 1.0) / 50 as f64) * 50 as f64) + 1.0
        );
    }

    #[wasm_bindgen_test]
    pub fn no_rows_1() {
        let parent_node = &crate::createElement("div");
        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let complex_object_array_test = include_str!("test_resources/no_rows.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);
        //1
        bq_table.render(parent_node);

        let element = &parent_node.first_element_child().unwrap();
        let bq_table = BigqueryQueryCustomElement::from_element(element)
            .expect("test: element should have all required attributes");

        bq_table.last_page();

        let page_start_index = element.get_attribute(PAGE_START_INDEX_ATT).unwrap();

        assert_eq!(page_start_index, "0");
    }

    #[wasm_bindgen_test]
    pub fn complex_object_array_test4_test_1() {
        let parent_node = &crate::createElement("div");

        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test4.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_table = &BigqueryQueryCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            Some("statement_type".to_string()),
        );

        let bq_query_information = complex_object_array_test.to_bq_query(bq_table);

        let rows_in_page = bq_query_information.rows_in_page;
        let rows_total = bq_query_information.rows_total;
        let table_builder = bq_query_information.table_builder;

        let bq_table = bq_table.with_table_info(rows_in_page, rows_total, table_builder);

        let start = instant::Instant::now();

        assert!(
            bq_table
                .render(&parent_node)
                .element()
                .outer_html()
                .contains("exclusive_access")
        );

        let elapsed = start.elapsed().as_millis();
        assert_eq!(elapsed, 22);
    }

    fn append_to_body(node: &web_sys::Node) {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(node)
            .unwrap();
    }

}
