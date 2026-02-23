use website_component_table::{
    TableBuilder, TableColumn, TableColumnDefinition, TableRow, TableStyle, TableValue,
};

use super::{
    bq_query_custom_element::BigqueryQueryCustomElement,
    bq_table_custom_element::BigqueryTableCustomElement,
    to_table_builder::patch_all_column_widths,
};
use crate::{
    bigquery::{
        jobs::{GetQueryResultsResponse, Job},
        table_data::TableDataListResponse,
        tables::Table,
    },
    parse_to_usize,
};

impl GetQueryResultsResponse {
    pub(crate) fn to_bq_query(
        &self,
        bq_query_requested: &BigqueryQueryCustomElement,
    ) -> BigqueryQueryCustomElement {
        let page_start_index = bq_query_requested.get_page_start_index();
        let rows_in_page = self.rows.as_ref().map_or(0, |r| r.len());
        let rows_total = self.get_rows_total();
        let table_builder = self.to_table_builder(page_start_index + 1);

        bq_query_requested.with_table_info(Some(rows_in_page), rows_total, Some(table_builder))
    }

    fn get_rows_total(&self) -> Option<usize> {
        match &self.total_rows {
            Some(v) => Some(parse_to_usize(Some(v.clone())).unwrap_or(0)),
            None => None,
        }
    }
}

impl Table {
    pub(crate) fn to_bq_table(
        &self,
        bq_table_element: &crate::custom_elements::bq_table_custom_element::BigqueryTableCustomElement,
        response_rows: &Option<TableDataListResponse>,
    ) -> BigqueryTableCustomElement {
        let rows_total = self.get_rows_total();

        let rows = match response_rows {
            Some(response_rows) => &response_rows.rows,
            None => &None,
        };

        let row_index = bq_table_element.get_page_start_index() + 1;
        let table_builder = self.to_table_builder(rows, row_index);
        let rows_in_page = rows.as_ref().map_or(0, |r| r.len());
        
        bq_table_element.with_table_info(Some(rows_in_page), Some(rows_total), Some(table_builder))
    }

    fn get_rows_total(&self) -> usize {
        parse_to_usize(Some(self.num_rows.clone().unwrap_or(String::from("0")))).unwrap_or(0)
    }
}

impl Job {
    pub(crate) fn to_error_table(&self) -> TableBuilder {
        let mut columns = [
            TableColumnDefinition::Column(TableColumn {
                name: "message".to_string(),
                text: "message".to_string(),
                width_px: 400,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "reason".to_string(),
                text: "reason".to_string(),
                width_px: 400,
            }),
        ]
        .to_vec();

        let rows: Vec<TableRow> = match self.status.as_ref() {
            Some(status) => match &status.error_result {
                Some(error_result) => [TableRow {
                    cells: vec![
                        TableValue::String(
                            error_result
                                .message
                                .as_ref()
                                .unwrap_or(&"".to_string())
                                .to_string(),
                        ),
                        TableValue::String(
                            error_result
                                .reason
                                .as_ref()
                                .unwrap_or(&"".to_string())
                                .to_string(),
                        ),
                    ],
                }]
                .to_vec(),
                None => Self::get_errors_rows_default(),
            },
            None => Self::get_errors_rows_default(),
        };

        patch_all_column_widths(&mut columns, &rows);
        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns,
            rows,
        }
    }

    fn get_errors_rows_default() -> Vec<TableRow> {
        vec![TableRow {
            cells: vec![
                TableValue::String("--".to_string()),
                TableValue::String("--".to_string()),
            ],
        }]
    }

    pub(crate) fn to_dml_table(&self) -> TableBuilder {
        let mut columns: Vec<TableColumnDefinition> = [
            "inserted_row_count".to_string(),
            "updated_row_count".to_string(),
            "deleted_row_count".to_string(),
        ]
        .iter()
        .map(|h| {
            TableColumnDefinition::Column(TableColumn {
                name: h.clone(),
                text: h.clone(),
                width_px: 200,
            })
        })
        .collect();

        let dml_stats = self.get_dml_stats();

        let rows = match dml_stats {
            Some(dml_stats) => {
                let inserted_row_count =
                    TableValue::String(dml_stats.inserted_row_count.unwrap_or_default());
                let updated_row_count =
                    TableValue::String(dml_stats.updated_row_count.unwrap_or_default());
                let deleted_row_count =
                    TableValue::String(dml_stats.deleted_row_count.unwrap_or_default());
                let row1 = [inserted_row_count, updated_row_count, deleted_row_count];

                vec![TableRow {
                    cells: row1.to_vec(),
                }]
            }
            None => Vec::<TableRow>::new(),
        };

        patch_all_column_widths(&mut columns, &rows);
        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns,
            rows,
        }
    }
}
