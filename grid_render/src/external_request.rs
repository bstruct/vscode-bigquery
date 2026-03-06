use serde::Deserialize;
use website_component_table::{
    TableBuilder, TableColumn, TableColumnDefinition, TableRow, TableStyle, TableValue,
};

use crate::{
    custom_elements::{
        bq_script_custom_element::BigqueryScriptCustomElement,
        bq_table_custom_element::BigqueryTableCustomElement,
    },
    utils::render_standalone,
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
    pub fn to_bq_table(&self, element_id: &str) -> Option<BigqueryTableCustomElement> {
        let project_id = self.project_id.as_ref()?.to_string();
        let dataset_id = self.dataset_id.as_ref()?.to_string();
        let table_id = self.table_id.as_ref()?.to_string();
        let token = self.token.as_ref()?.to_string();

        Some(BigqueryTableCustomElement::base_new(
            element_id.to_string(),
            project_id,
            dataset_id,
            table_id,
            token,
        ))
    }

    pub fn to_bq_script(&self, element_id: &str) -> Option<BigqueryScriptCustomElement> {
        let job = self.job.as_ref()?;
        let job_reference = job.job_reference.as_ref()?;
        let job_id = job_reference.job_id.to_string();
        let location = job_reference.location.to_string();
        let project_id = self.project_id.as_ref()?.to_string();
        let token = self.token.as_ref()?.to_string();

        let num_child_jobs = if (job.is_query_select() || job.is_dml_statement() || job.is_ddl_statement())
            && job.is_complete()
        {
            Some(1)
        } else {
            None
        };

        Some(BigqueryScriptCustomElement::base_new(
            element_id.to_string(),
            job_id,
            project_id,
            location,
            token,
            num_child_jobs,
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct ExternalRequestError {
    pub message: String,
    pub reason: String,
}

impl ExternalRequestError {
    pub fn plot_table(&self, element: &web_sys::Element) {
        let columns = [
            TableColumnDefinition::Column(TableColumn {
                name: "message".to_string(),
                text: "message".to_string(),
                width_px: 200,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "reason".to_string(),
                text: "reason".to_string(),
                width_px: 200,
            }),
        ].to_vec();
        let row = TableRow {
            cells: vec![
                TableValue::String(self.message.clone()),
                TableValue::String(self.reason.clone()),
            ],
        };

        let table_builder = TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: columns,
            rows: vec![row],
        };

        render_standalone(&table_builder, &element);
    }
}
