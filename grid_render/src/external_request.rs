use serde::Deserialize;

use crate::custom_elements::{
        bq_query_custom_element::BigqueryQueryCustomElement,
        bq_script_custom_element::BigqueryScriptCustomElement,
        bq_table_custom_element::BigqueryTableCustomElement,
        data_table_element::{DataTable, DataTableItem},
    };

#[derive(Debug, Deserialize)]
pub struct ExternalRequest {
    #[serde(alias = "requestType")]
    pub request_type: String,
    #[serde(alias = "projectId")]
    pub project_id: Option<String>,
    pub token: Option<String>,
    #[serde(alias = "datasetId")]
    pub dataset_id: Option<String>,
    #[serde(alias = "tableId")]
    pub table_id: Option<String>,
    pub job: Option<crate::bigquery::jobs::Job>,
    pub error: Option<ExternalRequestError>,
}

impl ExternalRequest {
    pub fn to_bq_table(&self, element_id: &str) -> BigqueryTableCustomElement {
        let project_id = self.project_id.as_ref().unwrap().to_string();
        let dataset_id = self.dataset_id.as_ref().unwrap().to_string();
        let table_id = self.table_id.as_ref().unwrap().to_string();
        let token = (&self.token.as_ref().unwrap()).to_string();

        BigqueryTableCustomElement::base_new(
            element_id.to_string(),
            project_id,
            dataset_id,
            table_id,
            token,
        )
    }

    pub fn to_bq_query(&self, element_id: &str) -> BigqueryQueryCustomElement {
        let job = self.job.as_ref().unwrap().job_reference.as_ref().unwrap();
        let job_id = job.job_id.to_string();
        let location = job.location.to_string();
        let project_id = self.project_id.as_ref().unwrap().to_string();
        let token = (&self.token.as_ref().unwrap()).to_string();

        BigqueryQueryCustomElement::base_new(
            element_id.to_string(),
            job_id,
            project_id,
            location,
            token,
            None
        )
    }

    pub fn to_bq_script(&self, element_id: &str) -> BigqueryScriptCustomElement {
        let job = self.job.as_ref().unwrap();
        let job_reference = job.job_reference.as_ref().unwrap();
        let job_id = job_reference.job_id.to_string();
        let location = job_reference.location.to_string();
        let project_id = self.project_id.as_ref().unwrap().to_string();
        let token = (&self.token.as_ref().unwrap()).to_string();

        BigqueryScriptCustomElement::base_new(
            element_id.to_string(),
            job_id,
            project_id,
            location,
            token,
            None
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct ExternalRequestError {
    pub message: String,
    pub reason: String,
}

impl ExternalRequestError {
    pub fn plot_table(&self, element: &web_sys::Element) {
        let header = ["message".to_string(), "reason".to_string()].to_vec();
        let rows = [[
            Some(DataTableItem::from_string(&self.message)),
            Some(DataTableItem::from_string(&self.reason)),
        ]
        .to_vec()]
        .to_vec();

        DataTable::new("e1", &Some(header), &Some(rows)).render_standalone(element);
    }
}
