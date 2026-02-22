use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, Node};
use website_component_table::TableBuilder;

use crate::{
    bigquery::jobs::{GetJobRequest, GetListRequest, Job, JobStatus},
    parse_to_usize, set_state, utils::render_standalone,
};

use super::{
    base_element::BaseElement,
    base_element_trait::BaseElementTrait,
    bq_common_custom_element::{get_attribute, get_opt_attribute, remove_attribute, set_attribute},
    bq_query_custom_element::{BigqueryQueryCustomElement, RENDER_QUERY_EVENT_NAME},
    custom_element_definition::CustomElementDefinition,
};

const TAG_NAME: &'static str = "bq-script";
const RENDER_SCRIPT_EVENT_NAME: &str = "render_script";

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
            num_child_jobs: parse_to_usize(get_opt_attribute(element, "num_child_jobs")),
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
        let num_child_jobs = if job.is_dml_statement() || job.is_query_select() {
            Some(1)
        } else {
            match job.statistics.as_ref() {
                Some(statistics) => parse_to_usize(statistics.num_child_jobs.clone()),
                None => None,
            }
        };

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
        let all_jobs_completed = self.all_jobs_completed() && self.num_child_jobs.is_some();

        if !all_jobs_completed {
            if let Some(window) = web_sys::window() {
                let parent_node = parent_node.clone();
                let on_timeout = Closure::once(Box::new(move || {
                    if let Some(child) = parent_node.first_child() {
                        if let Ok(element) = child.dyn_into::<web_sys::Element>() {
                            if let Ok(event) = web_sys::Event::new(RENDER_SCRIPT_EVENT_NAME) {
                                let _ = element.dispatch_event(&event);
                            }
                        }
                    }
                }) as Box<dyn FnOnce()>);
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    on_timeout.as_ref().unchecked_ref(),
                    5000,
                );
                on_timeout.forget();
            }
        }
    }

    pub(crate) fn dispatch_on_render_event(&self, element: &Element) {
        if let Some(first_child) = element.first_child() {
            if let Ok(first_child) = first_child.dyn_into::<web_sys::Element>() {
                if let Ok(event) = web_sys::Event::new(RENDER_SCRIPT_EVENT_NAME) {
                    let _ = first_child.dispatch_event(&event);
                }
            }
        }
    }

    fn all_jobs_completed(&self) -> bool {
        if let Some(jobs) = &self.jobs {
            return jobs.iter().all(|j| j.is_complete());
        }

        true
    }
}

impl BaseElementTrait for BigqueryScriptCustomElement {
    fn get_element_id(&self) -> &str {
        &self.element_id
    }

    fn render(&self, parent_node: &web_sys::Node) -> BaseElement {
        // Persist the top-level script job so VS Code can restore all child
        // jobs on restart.  Child bq-query elements must NOT overwrite this.
        let state = serde_json::json!({
            "jobId":     self.job_id,
            "projectId": self.project_id,
            "location":  self.location,
        });
        set_state(&state.to_string()).unwrap_or(());

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
    //loading

    let is_loaded = (script_element.num_child_jobs.is_some()
        && script_element.all_jobs_completed())
        || (script_element.jobs.is_some()
            && script_element.jobs.as_ref().unwrap().len() == 1
            && script_element.jobs.as_ref().unwrap()[0].is_complete());

    let loading_class_name = if is_loaded { "loaded" } else { "" };
    BaseElement::new_and_append(&element.element(), "DIV", "job_loading")
        .apply_fn(&resolve_loading, &Some(loading_class_name));

    //
    let num_child_jobs = if script_element.num_child_jobs.is_some() {
        script_element.num_child_jobs.unwrap()
    } else {
        if let Some(jobs) = &script_element.jobs {
            jobs.len()
        } else {
            0
        }
    };

    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    //     "num_child_jobs: {}",
    //     num_child_jobs
    // )));

    for index in 0..num_child_jobs {
        let chid_job: Option<&Job> = match &script_element.jobs {
            Some(jobs) => {
                let job_search = jobs.into_iter().find(|job| {
                    job.id.is_some() && job.id.clone().unwrap().ends_with(&format!("_{}", index))
                });

                if job_search.is_some() && job_search.unwrap().id.is_some() {
                    Some(job_search.unwrap())
                } else {
                    if num_child_jobs == 1 {
                        Some(&script_element.jobs.as_ref().unwrap()[0])
                    } else {
                        None
                    }
                }
            }
            None => None,
        };

        let (job_name, job_status) = if chid_job.is_some() && chid_job.unwrap().id.is_some() {
            let job = chid_job.as_ref().unwrap();

            (
                job.id.as_ref().unwrap().to_string(),
                job.status.as_ref().clone(),
            )
        } else {
            ("?".to_string(), None)
        };

        let job_body =
            BaseElement::new_and_append(&element.element(), "DIV", &format!("job_{}", index))
                .append_child("DIV", &format!("job_title_{}", index))
                .apply_fn(&resolve_job_title, &(job_name, job_status))
                .append_sibling("DIV", &format!("job_body_{}", index));

        //insert bq-query custom element only when the job is complete
        if chid_job.is_some() {
            let child_job = chid_job.as_ref().unwrap();

            //check if job is in error
            if child_job.has_error() {
                job_body
                    .apply_default_class_name("job_body_open")
                    .apply_fn(&inser_error_table, &child_job.to_error_table());
            } else if child_job.is_complete() {
                // Open the last job body, collapse the rest
                if index == (num_child_jobs - 1) {
                    let _ = &job_body.apply_class_name("job_body_open");
                } else {
                    let _ = &job_body.apply_class_name("job_body_closed");
                }

                let job_reference = child_job.job_reference.as_ref().unwrap();
                let bq_query = BigqueryQueryCustomElement::base_new(
                    format!("job_query_{}", index),
                    job_reference.job_id.clone(),
                    job_reference.project_id.clone(),
                    job_reference.location.clone(),
                    script_element.token.clone(),
                    child_job.get_statement_type(),
                );

                let bq_query_element = job_body.append_base_child(&bq_query);
                // Dispatch render_table to trigger the async query results fetch.
                // Only done when job is complete, so on_render_query won't race against
                // a still-running job and permanently mark loaded=1 with no data.
                // Clear 'loaded' first: on re-render (refresh / polling cycle) the
                // bq-query element already exists with loaded=1, and on_render_query
                // would silently skip the fetch without this reset.
                let _ = bq_query_element.element().remove_attribute("loaded");
                if let Ok(event) = web_sys::Event::new(RENDER_QUERY_EVENT_NAME) {
                    let _ = bq_query_element.element().dispatch_event(&event);
                }
            } else {
                // Job is still pending/running — keep body closed, results not yet available
                let _ = &job_body.apply_class_name("job_body_closed");
            }
        }
    }

    fn inser_error_table(base_element: &BaseElement, table: &TableBuilder) {
        let element = base_element.element();
        render_standalone(table, &element);
    }
}

fn resolve_loading(element: &BaseElement, class_name: &Option<&str>) {
    let html_element = element.element();
    html_element.set_inner_html("Loading<span>...</span>");
    html_element.set_class_name(class_name.unwrap_or_default());
}

fn resolve_job_title(element: &BaseElement, (job_name, job_status): &(String, Option<&JobStatus>)) {
    let content = if job_status.is_some() {
        let job_status = job_status.as_ref().unwrap();
        if job_status.error_result.is_some() {
            format!("ERROR - {}", job_name)
        } else {
            format!("{} - {}", job_status.state, job_name)
        }
    } else {
        format!("? - {}", job_name)
    };

    let html_element = element.element();
    // Use inner HTML so we can embed the refresh button alongside the text.
    html_element.set_inner_html(&format!(
        r#"<span class="job-title-text">{}</span><button class="job-refresh-btn" title="Refresh">&#x21BB;</button>"#,
        content
    ));

    // Add refresh click listener to the button each time (button is recreated by set_inner_html).
    if let Some(btn) = html_element.query_selector(".job-refresh-btn").ok().flatten() {
        let on_refresh = Closure::wrap(Box::new(|event: web_sys::Event| {
            event.stop_propagation();
            let target = match event
                .current_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
                Some(e) => e,
                None => return,
            };
            let script = match target.closest(TAG_NAME).ok().flatten() {
                Some(e) => e,
                None => return,
            };
            let _ = script.remove_attribute("loaded");
            if let Ok(render_event) = web_sys::Event::new(RENDER_SCRIPT_EVENT_NAME) {
                let _ = script.dispatch_event(&render_event);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = btn.add_event_listener_with_callback("click", on_refresh.as_ref().unchecked_ref());
        on_refresh.forget();
    }

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "job_status: {:?}",
        job_status
    )));

    //https://cloud.google.com/bigquery/docs/reference/rest/v2/Job#JobStatus
    // Valid states include 'PENDING', 'RUNNING', and 'DONE'.
    if job_status.is_some() && job_status.unwrap().state == "DONE" {
        html_element.set_class_name("title ready");
    } else {
        html_element.set_class_name("title");
    }

    //toggle job_body on click
    if html_element.get_attribute("bee").unwrap_or_default() != "1" {
        let on_click_event = Closure::wrap(Box::new(|event: Event| {
            let element = event.current_target().unwrap();
            let element = element.dyn_into::<web_sys::Element>().unwrap();
            if let Some(next_element) = element.next_element_sibling() {
                match next_element.class_name().as_str() {
                    "job_body_closed" => next_element.set_class_name("job_body_open"),
                    _ => next_element.set_class_name("job_body_closed"),
                }
            }
        }) as Box<dyn FnMut(_)>);

        html_element
            .add_event_listener_with_callback("click", on_click_event.as_ref().unchecked_ref())
            .unwrap();

        on_click_event.forget();

        html_element.set_attribute("bee", "1").unwrap();
    }
}

impl CustomElementDefinition for BigqueryScriptCustomElement {
    fn define(_document: &web_sys::Document, element: &web_sys::Element) {
        let on_event_type_closure =
            Closure::wrap(Box::new(on_render) as Box<dyn Fn(&web_sys::Event)>);

        element
            .add_event_listener_with_callback(
                RENDER_SCRIPT_EVENT_NAME,
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

    if !element.has_attribute("loaded") {
        let bq_script_element = BigqueryScriptCustomElement::from_element(&element);

        let jobs = crate::bigquery::jobs::Jobs::new(&bq_script_element.token);
        let parent_node = element.parent_node().unwrap();

        spawn_local(async move {
            let get_request = bq_script_element.as_job_request();
            let get_job_response = jobs.get(get_request).await;

            if let Some(job) = get_job_response {
                //mark as done.
                // if job.has_error() {
                //     element.set_attribute("loaded", "1").unwrap();
                // }

                //TODO: confirm what is the information when one of the jobs is in error
                if job.is_dml_statement() || job.is_query_select() || job.is_unsupported_type() {
                    // element.set_attribute("loaded", "1").unwrap();
                    if job.is_complete() {
                        element.set_attribute("loaded", "1").unwrap();
                    }

                    bq_script_element
                        .with_job_info(&job, &[job.clone()].to_vec())
                        .render(&parent_node);
                } else {
                    let get_list_request = bq_script_element.as_job_list_request();
                    let get_list_response = jobs.get_list(get_list_request).await;

                    if let Some(list) = get_list_response {
                        if let Some(jobs) = list.jobs {
                            let all_jobs_done = jobs.iter().all(|j| j.is_complete());

                            if let Some(statistics) = &job.statistics {
                                if statistics.num_child_jobs.is_some() && all_jobs_done {
                                    element.set_attribute("loaded", "1").unwrap();
                                }
                            }

                            bq_script_element
                                .with_job_info(&job, &jobs)
                                .render(&parent_node);
                        }
                    }
                }
            } else {
                element.set_attribute("loaded", "1").unwrap();
                element.set_inner_html(&format!("unexpected response: {:?}", get_job_response));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::custom_elements::base_element_trait::BaseElementTrait;

    use super::BigqueryScriptCustomElement;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn render_test_1() {
        let parent_node = &crate::createElement("div");
        let bq_script = &BigqueryScriptCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
            None,
        );

        let job = &crate::bigquery::jobs::Job {
            kind: None,
            etag: None,
            id: None,
            self_link: None,
            user_email: None,
            configuration: None,
            job_reference: Some(crate::bigquery::jobs::JobReference {
                project_id: "projectId".to_string(),
                job_id: "jobId".to_string(),
                location: "location".to_string(),
            }),
            statistics: None,
            status: None,
            principal_subject: None,
            job_creation_reason: None,
        };

        let get_jobs = include_str!("test_resources/get_jobs_with_error.json");
        let get_jobs =
            &serde_json::from_str::<crate::bigquery::jobs::GetListResponse>(get_jobs).unwrap();

        bq_script.with_job_info(job, &get_jobs.jobs.as_ref().unwrap().clone());
        bq_script.render(parent_node);
        // let bq_table_information = complex_object_array_test.to(bq_table);

        // let rows_in_page = bq_table_information.rows_in_page;
        // let rows_total = bq_table_information.rows_total;
        // let header = bq_table_information.header;
        // let rows = bq_table_information.rows;

        // let bq_table = bq_table.with_table_info(rows_in_page, rows_total, header, rows);
        // bq_table.render(parent_node);

        // let c = parent_node.first_child().unwrap();
        // assert_eq!(c.node_type(), web_sys::Node::ELEMENT_NODE);
        // let element: web_sys::Element = wasm_bindgen::JsCast::dyn_into(c.value_of())
        //     .expect("unexpected error on casting Node to Element");
        // assert_eq!(element.tag_name().to_lowercase(), TAG_NAME);

        // assert!(element.shadow_root().is_some());
        // let shadow = element.shadow_root().unwrap();

        // let c = shadow.first_element_child().unwrap();
        // assert_eq!(c.tag_name().to_lowercase(), "style");

        // let c = c.next_element_sibling().unwrap();
        // assert_eq!(c.tag_name().to_lowercase(), "div");
        // assert_eq!(c.get_attribute("be_id").unwrap(), "controls-background");

        // let c = c.next_element_sibling().unwrap();
        // assert_eq!(c.tag_name().to_lowercase(), "table");

        // // assert_eq!(c.outer_html(), "...");
    }
}
