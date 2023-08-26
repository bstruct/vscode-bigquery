// use web_sys::Node;

// use crate::{createElement, web_events::dialog_user_entry_event::DialogUserEntryEvent};

use crate::bigquery::jobs::{GetQueryResultsRequest, Jobs};

use super::custom_element_definition::CustomElementDefinition;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub struct QueryResultsWithControls;

impl CustomElementDefinition for QueryResultsWithControls {
    fn define(_document: &web_sys::Document, element: &web_sys::HtmlElement) {
        // element.add_event_listener_with_callback("type_", listener)

        let on_event_type_closure = Closure::wrap(Box::new(
            QueryResultsWithControls::on_render_table,
        ) as Box<dyn Fn(&web_sys::Event)>);
        // form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));

        element
            .add_event_listener_with_callback(
                "render_table",
                on_event_type_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        on_event_type_closure.forget();
    }
}

impl QueryResultsWithControls {
    pub fn on_render_table(event: &web_sys::Event) {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        //clear out the content
        element.set_inner_html("");

        let job_id = element.get_attribute("jobId").unwrap();
        let project_id = element.get_attribute("projectId").unwrap();
        let location = element.get_attribute("location").unwrap();
        let token = element.get_attribute("token").unwrap();

        let jobs = Jobs::new(&token);
        let request = GetQueryResultsRequest {
            project_id: project_id,
            job_id: job_id,
            start_index: None,
            page_token: None,
            max_results: None,
            timeout_ms: None,
            location: Some(location),
        };

        spawn_local(async move {
            let response = jobs.get_query_results(request).await;
            if response.is_some() {
                let render_table = QueryResultsWithControls::render_table;
                render_table(&element, &response.unwrap());
            }

            // element.set_inner_text(&format!("xxx: {:?}", response));
        });
    }

    fn render_table(
        element: &HtmlElement,
        query_response: &crate::bigquery::jobs::GetQueryResultsResponse,
    ) {
        // https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/src/data-grid/README.md
        //<vscode-data-grid>
        let grid = crate::createElement("vscode-data-grid");
        element.append_child(&grid).unwrap();

        //<vscode-data-grid-row>
        let row = crate::createElement("vscode-data-grid-row");
        grid.append_child(&row).unwrap();

        //<vscode-data-grid-cell>
        let header_cell_style = "background-color: var(--list-hover-background);";

        let cell = crate::createElement("vscode-data-grid-cell");
        cell.set_attribute("cell-type", "columnheader").unwrap();
        cell.set_attribute("style", header_cell_style).unwrap();
        cell.set_attribute("grid-column", "1").unwrap();
        cell.set_inner_html("Row");
        row.append_child(&cell).unwrap();

        let fields = &query_response.schema.to_owned().unwrap().fields.to_vec();

        let mut index = 1;
        for column in fields {
            let cell = crate::createElement("vscode-data-grid-cell");
            cell.set_attribute("cell-type", "columnheader").unwrap();
            cell.set_attribute("style", header_cell_style).unwrap();
            cell.set_attribute("grid-column", &(index + 1).to_string())
                .unwrap();
            cell.set_inner_html(&column.name);
            index = index + 1;
            row.append_child(&cell).unwrap();
        }

        //
        let mut index = 0;
        for query_response_row in &query_response.rows {
            let row = crate::createElement("vscode-data-grid-row");
            grid.append_child(&row).unwrap();

            let cell = crate::createElement("vscode-data-grid-cell");
            cell.set_attribute("style", header_cell_style).unwrap();
            cell.set_attribute("grid-column", &(index + 1).to_string())
                .unwrap();
            cell.set_inner_html("xx");
            index = index + 1;
            row.append_child(&cell).unwrap();

            for _column in fields {

                let cell = crate::createElement("vscode-data-grid-cell");
                cell.set_attribute("style", header_cell_style).unwrap();
                cell.set_attribute("grid-column", &(index + 1).to_string())
                    .unwrap();
                cell.set_inner_html(query_response_row.get(index).unwrap().as_str().unwrap());
                index = index + 1;
                row.append_child(&cell).unwrap();
            }
        }

        // const cells: preact.VNode[] = [preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '1' }, 'Row')];
    }
}
