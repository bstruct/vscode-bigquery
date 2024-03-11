use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, Node};

use crate::{
    bigquery::jobs::{GetJobRequest, GetListRequest, Job},
    parse_to_usize,
};

use super::{
    base_element::BaseElement,
    base_element_trait::BaseElementTrait,
    bq_common_custom_element::{get_attribute, remove_attribute, set_attribute},
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
    num_child_jobs: Option<usize>,
}

impl BigqueryScriptCustomElement {
    pub(crate) fn base_new(
        element_id: String,
        job_id: String,
        project_id: String,
        location: String,
        token: String,
        num_child_jobs: Option<usize>,
    ) -> BigqueryScriptCustomElement {
        BigqueryScriptCustomElement {
            element: None,
            element_id,
            job_id,
            project_id,
            location,
            token,
            jobs: None,
            num_child_jobs: num_child_jobs,
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
            num_child_jobs: None,
        }
    }

    fn as_job_request(&self) -> GetJobRequest {
        GetJobRequest {
            project_id: self.project_id.clone(),
            job_id: self.job_id.clone(),
            location: Some(self.location.clone()),
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

    fn with_job_info(&self, job: &Job, jobs: &Vec<Job>) -> BigqueryScriptCustomElement {
        let num_child_jobs = job
            .statistics
            .as_ref()
            .expect("job statistics not found")
            .num_child_jobs
            .clone();
        let num_child_jobs = parse_to_usize(num_child_jobs);
        let job_reference = job.job_reference.as_ref().expect("job_reference not found");

        BigqueryScriptCustomElement {
            element: self.element.to_owned(),
            element_id: self.element_id.clone(),
            job_id: job_reference.job_id.clone(),
            project_id: job_reference.project_id.clone(),
            location: job_reference.location.clone(),
            token: self.token.clone(),
            jobs: Some(jobs.clone()),
            num_child_jobs: num_child_jobs,
        }
    }

    fn set_refresh_timeout(&self, parent_node: &Node) {
        if self.num_child_jobs.is_none() {
            if let Some(window) = web_sys::window() {
                let dispatch_event = Closure::wrap(Box::new(|node: Node| {
                    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                    //     "1) dispatch_event: {:?}",
                    //     node.type_id()
                    // )));

                    let event = &Event::new(ELEMENT_INTERSECTED_EVENT_NAME).unwrap();
                    node.first_child().unwrap().dispatch_event(event).unwrap();
                }) as Box<dyn FnMut(_)>);

                window
                    .set_timeout_with_callback_and_timeout_and_arguments(
                        dispatch_event.as_ref().unchecked_ref(),
                        5000,
                        &js_sys::Array::of1(parent_node),
                    )
                    .unwrap();

                dispatch_event.forget();
            }
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

        //refresh data in x seconds if the load is not complete
        self.set_refresh_timeout(parent_node);

        BaseElement::new_and_append(parent_node, TAG_NAME, &self.element_id)
            .apply_fn(&set_attributes, self)
            // .append_shadow()
            .append_child_style(css_content, "style1")
            .append_sibling("div", "jobs")
            .apply_fn(&resolve_jobs, &self)
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
    if let Some(num_child_jobs) = bq_table.num_child_jobs {
        set_attribute(&element, "num_child_jobs", &num_child_jobs.to_string());
    } else {
        remove_attribute(&element, "num_child_jobs");
    }
}

fn resolve_jobs(element: &BaseElement, script_element: &BigqueryScriptCustomElement) {
    if let Some(num_child_jobs) = script_element.num_child_jobs {
        for index in 0..num_child_jobs {
            let element_id = &format!("job_{}", index);

            let mut job_name: &str = "?";

            if let Some(jobs) = &script_element.jobs {
                let job = jobs.into_iter().find(|job| {
                    job.id.is_some() && job.id.clone().unwrap().ends_with(&format!("_{}", index))
                });

                if job.is_some() && job.unwrap().id.is_some() {
                    job_name = &job.as_ref().unwrap().id.as_ref().unwrap();
                }
            }

            BaseElement::new_and_append(&element.element(), "DIV", element_id)
                .apply_fn(&resolve_job, &String::from(job_name));
        }
    } else {
        if let Some(jobs) = &script_element.jobs {
            // for job in jobs
            for index in 0..jobs.len() {
                let element_id = &format!("job_{}", index);

                let job = jobs.into_iter().find(|job| {
                    job.id.is_some() && job.id.clone().unwrap().ends_with(&format!("_{}", index))
                });

                let job_name: &str = if job.is_some() && job.unwrap().id.is_some() {
                    &job.as_ref().unwrap().id.as_ref().unwrap()
                } else {
                    "?"
                };

                // let job_name = job.id.clone().unwrap_or(String::from("?"));
                // let job_reference = job.job_reference.expect("job_reference not found");
                // let job_full_id = format!("{}:{}.{}", job_reference.project_id, job_reference.location, job_reference.job_id);

                BaseElement::new_and_append(&element.element(), "DIV", element_id)
                // .append_child("div", &format!("d{}", col_index))
                .apply_fn(&resolve_job, &String::from(job_name))
                // .apply_fn(&set_resize_actions, &col_index)
                ;
            }
        }
    }

    // let num_child_jobs = p.0;
    // let project_id = p.1;
    // let location = p.2;
    // let job_id = p.3;

    // if let Some(jobs) = jobs {
    //     assert!(num_child_jobs >= jobs.len());

    // let mut index = 0;
    //     for index in 0..num_child_jobs {
    //         // element
    //         //     .append_child("DIV", "xxx")// &format!("job_{}", index))
    //         //     // .apply_fn(&resolve_job, &job)
    //         //     ;
    //         // let job = if jobs.len() > index {
    //         //     Some(&jobs[index])
    //         // } else {
    //         //     None
    //         // };

    //         // let job_name = if job.is_none() {
    //         //     String::from("?")
    //         // } else {
    //         //     job.as_ref().unwrap().id.clone().unwrap_or(String::from("?"))
    //         // }.to_string();
    //         let job_name = format!("{}:{}.{}_{}", project_id, location, job_id, index);

    //         BaseElement::new_and_append(&element.element(), "DIV", &format!("job_{}", index))
    //             // .append_child("div", &format!("d{}", col_index))
    //             .apply_fn(&resolve_job, &job_name)
    //             // .apply_fn(&set_resize_actions, &col_index)
    //         ;

    //         // index += 1;
    //     // }
    // }
}

fn resolve_job(element: &BaseElement, job_name: &String) {
    // let content: &str = if let Some(query) = &job.configuration.as_ref().unwrap().query {
    //     &query.query
    // } else {
    //     "xxx?"
    // };

    element.element().set_text_content(Some(job_name));
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
    let get_request = bq_script_element.as_job_request();
    let get_list_request = bq_script_element.as_job_list_request();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "on_render event on element: {:?}, request: {:?}",
        element.id(),
        get_list_request
    )));

    let jobs = crate::bigquery::jobs::Jobs::new(&bq_script_element.token);
    let parent_node = element.parent_node().unwrap();

    spawn_local(async move {
        let get_job_response = jobs.get(get_request).await;
        let get_list_response = jobs.get_list(get_list_request).await;

        if let Some(job) = get_job_response {
            if let Some(list) = get_list_response {
                // response
                //     .to_bq_script(&bq_script_element)
                //     .render(&parent_node);

                bq_script_element
                    .with_job_info(&job, &list.jobs)
                    .render(&parent_node);
            }
        } else {
            element.set_inner_html(&format!("unexpected response: {:?}", get_job_response));
        }
    });
}
