use crate::custom_elements::{data_table_element::{DataTableItem, DataTable}, data_table_shadow_element::DataTableShadow};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExternalRequestError {
    pub message: String,
    pub reason: String,
}

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

impl ExternalRequestError {
    pub fn plot_table(&self, element: &web_sys::Element) {
        let header = &["message".to_string(), "reason".to_string()].to_vec();
        let rows = [[
            Some(DataTableItem::from_string(&self.message)),
            Some(DataTableItem::from_string(&self.reason)),
        ]
        .to_vec()]
        .to_vec();

        let shadow_root = &DataTableShadow::init_shadow(element);

        DataTable::render_table(shadow_root, header, &rows);
    }
}