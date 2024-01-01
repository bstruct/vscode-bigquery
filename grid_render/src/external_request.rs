use serde::Deserialize;

use crate::custom_elements::{bq_table_custom_element::BigqueryTableCustomElement, data_table_element::DataTableItem};

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    #[serde(alias = "projectId")]
    pub project_id: Option<String>,
    pub token: Option<String>,
    pub query: Option<String>,
    pub job: Option<crate::bigquery::jobs::Job>,
    pub error: Option<ExternalRequestError>,
}

impl ExternalRequest {
    pub fn to_bq_table(&self, element_id: &str) -> BigqueryTableCustomElement {

        let job = self.job.as_ref().unwrap().job_reference.as_ref().unwrap();
        let job_id = job.job_id.to_string();
        let location = job.location.to_string();
        let project_id = self.project_id.as_ref().unwrap().to_string();
        let token = (&self.token.as_ref().unwrap()).to_string();

        BigqueryTableCustomElement::base_new(element_id.to_string(), job_id, project_id, location, token)
    }
}

#[derive(Debug, Deserialize)]
pub struct ExternalRequestError {
    pub message: String,
    pub reason: String,
}

impl ExternalRequestError {
    pub fn plot_table(&self, element: &web_sys::Element) {
        let header = &["message".to_string(), "reason".to_string()].to_vec();
        let rows = [[
            Some(DataTableItem::from_string(&self.message)),
            Some(DataTableItem::from_string(&self.reason)),
        ]
        .to_vec()]
        .to_vec();

        // let shadow_root = &DataTableShadow::init_shadow(element);

        // DataTable::render_table(shadow_root, header, &rows);
    }
}
