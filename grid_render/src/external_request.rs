use crate::custom_elements::table_plot::{render_table, TableItem};
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
        let header = &[
            "message".to_string(),
            "reason".to_string(),
        ]
        .to_vec();
        let rows = [[
            Some(TableItem::from_string(&self.message)),
            Some(TableItem::from_string(&self.reason)),
        ]
        .to_vec()]
        .to_vec();

        render_table(false, element, header, &rows, 1);
    }
}
