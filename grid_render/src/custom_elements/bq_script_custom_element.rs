use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

use crate::bigquery::jobs::{GetListRequest, GetListResponse, Job};

use super::{
    base_element::BaseElement,
    base_element_trait::BaseElementTrait,
    bq_common_custom_element::{get_attribute, set_attribute},
    custom_element_definition::CustomElementDefinition,
};

const TAG_NAME: &'static str = "bq-script";
const ELEMENT_INTERSECTED_EVENT_NAME: &str = "element_intersected";

pub(crate) struct BigqueryScriptCustomElement {
    element: Option<Element>,
    element_id: String,
    job_id: String,
    project_id: String,
    location: String,
    token: String,
    jobs: Option<Vec<Job>>,
}

impl BigqueryScriptCustomElement {
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
            token,
            jobs: None,
        }
    }

    pub(crate) fn from_element(element: &Element) -> BigqueryScriptCustomElement {
        let element_id = BaseElement::from_element(element)
            .id()
            .as_ref()
            .unwrap()
            .to_string();

        BigqueryScriptCustomElement {
            element: Some(element.to_owned()),
            element_id,
            job_id: get_attribute(element, "job_id"),
            project_id: get_attribute(element, "project_id"),
            location: get_attribute(element, "location"),
            token: get_attribute(element, "token"),
            jobs: None,
        }
    }

    fn as_job_list_request(&self) -> GetListRequest {
        GetListRequest {
            project_id: self.project_id.clone(),
            parent_job_id: Some(self.job_id.clone()),
            max_results: None,
            projection: Some(crate::bigquery::jobs::Projection::FULL),
        }
    }

    fn with_jobs_info(&self, jobs: &Vec<Job>) -> BigqueryScriptCustomElement {
        BigqueryScriptCustomElement {
            element: self.element.to_owned(),
            element_id: self.element_id.clone(),
            job_id: self.job_id.clone(),
            project_id: self.project_id.clone(),
            location: self.location.clone(),
            token: self.token.clone(),
            jobs: Some(jobs.clone()),
        }
    }
}

impl BaseElementTrait for BigqueryScriptCustomElement {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        let css_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/bqscript.css"
        ));

        BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            .apply_fn(&set_attributes, self)
            // .append_shadow()
            .append_child_style(css_content, "style1")
            .append_sibling("div", "jobs")
            .apply_fn(&resolve_jobs, &self.jobs)
        //.append_sibling_base_element(&self.to_data_table_controls())
        // .append_sibling_base_element(&self.to_data_table("t1"))
    }
}

fn set_attributes(base_element: &BaseElement, bq_table: &BigqueryScriptCustomElement) {
    let element = base_element.element();
    element.set_id(&bq_table.element_id);

    set_attribute(&element, "job_id", bq_table.job_id.as_str());
    set_attribute(&element, "project_id", bq_table.project_id.as_str());
    set_attribute(&element, "location", bq_table.location.as_str());
    set_attribute(&element, "token", bq_table.token.as_str());
}

fn resolve_jobs(element: &BaseElement, jobs: &Option<Vec<Job>>) {
    if let Some(jobs) = jobs {
        let mut index = 0;
        for job in jobs {
            // element
            //     .append_child("DIV", "xxx")// &format!("job_{}", index))
            //     // .apply_fn(&resolve_job, &job)
            //     ;

            BaseElement::new_and_append(&element.element(), "DIV", &format!("job_{}", index))
                // .append_child("div", &format!("d{}", col_index))
                .apply_fn(&resolve_job, &job)
                // .apply_fn(&set_resize_actions, &col_index)
            ;

            index += 1;
        }
    }
}

fn resolve_job(element: &BaseElement, job: &Job) {
    let content: &str = if let Some(query) = &job.configuration.as_ref().unwrap().query {
        &query.query
    } else {
        "xxx?"
    };

    element.element().set_text_content(Some(content));
}

impl CustomElementDefinition for BigqueryScriptCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::Element) {
        // let on_event_type_closure =
        //     Closure::wrap(Box::new(BigqueryScriptCustomElement::on_render_query)
        //         as Box<dyn Fn(&web_sys::Event)>);

        // element
        //     .add_event_listener_with_callback(
        //         RENDER_QUERY_EVENT_NAME,
        //         on_event_type_closure.as_ref().unchecked_ref(),
        //     )
        //     .unwrap();

        // on_event_type_closure.forget();

        //ELEMENT_INTERSECTED_EVENT_NAME
        let on_event_type_closure =
            Closure::wrap(Box::new(on_render) as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                ELEMENT_INTERSECTED_EVENT_NAME,
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        on_event_type_closure.forget();
    }
}

impl GetListResponse {
    pub(crate) fn to_bq_script(
        &self,
        bq_script_requested: &BigqueryScriptCustomElement,
    ) -> BigqueryScriptCustomElement {
        bq_script_requested.with_jobs_info(&self.jobs)
    }
}

fn on_render(event: &web_sys::Event) {
    let element = event
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "1) on_render_table event on element: {:?}",
        element.id()
    )));

    let bq_script_element = BigqueryScriptCustomElement::from_element(&element);
    let request = bq_script_element.as_job_list_request();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "on_render event on element: {:?}, request: {:?}",
        element.id(),
        request
    )));

    let jobs = crate::bigquery::jobs::Jobs::new(&bq_script_element.token);
    let parent_node = element.parent_node().unwrap();

    spawn_local(async move {
        let response = jobs.get_list(request).await;
        if let Some(response) = response {
            response
                .to_bq_script(&bq_script_element)
                .render(&parent_node);
        } else {
            element.set_inner_html(&format!("unexpected response: {:?}", response));
        }
    });
}
