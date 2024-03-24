use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;


pub struct TableData {
    token: String,
}

//https://cloud.google.com/bigquery/docs/reference/rest/v2/tabledata/list#query-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDataListRequest{
    #[serde(alias = "projectId")]
    pub project_id: String,
    #[serde(alias = "datasetId")]
    pub dataset_id: String,
    #[serde(alias = "tableId")]
    pub table_id: String,
    #[serde(alias = "maxResults")]
    pub max_results: Option<usize>,
    // #[serde(alias = "pageToken")]
    // pub page_token: Option<String>,
    #[serde(alias = "startIndex")]
    pub start_index: Option<String>,
    //selectedFields
    //formatOptions
}

//https://cloud.google.com/bigquery/docs/reference/rest/v2/tabledata/list#response-body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDataListResponse{
    pub kind: String,
    pub etag: String,
    #[serde(alias = "totalRows")]
    pub total_rows: String,
    #[serde(alias = "pageToken")]
    pub page_token: Option<String>,
    pub rows: Vec<serde_json::Value>,
}

impl TableData {
    pub fn new(token: &str) -> TableData {
        TableData {
            token: String::from(token),
        }
    }
    pub async fn list(
        self: &Self,
        request: TableDataListRequest,
    ) -> Option<TableDataListResponse> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        opts.mode(web_sys::RequestMode::Cors);
        let headers = web_sys::Headers::new().unwrap();
        // headers.set("Accept", "application/json").unwrap();
        headers.set("Content-Type", "application/json").unwrap();
        headers
            .set("Authorization", &format!("Bearer {}", &self.token))
            .unwrap();
        opts.headers(&headers);

        let mut url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets/{}/tables/{}/data",
            request.project_id, 
            request.dataset_id,
            request.table_id,
        );

        // if (request.location) { url.searchParams.append("location", request.location); }
        if request.max_results.is_some() {
            url = format!("{}?maxResults={}", url, request.max_results.unwrap());
        } else {
            url = format!("{}?maxResults=50", url);
        }
        if request.start_index.is_some() {
            url = format!("{}&startIndex={}", url, request.start_index.unwrap());
        }

        console::log_1(&JsValue::from_str(&url));

        // if (request.pageToken) { url.searchParams.append("pageToken", request.pageToken); }
        // if (request.timeoutMs !== null) { url.searchParams.append("timeoutMs", request.timeoutMs.toString()); }

        let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        assert!(wasm_bindgen::JsCast::is_instance_of::<web_sys::Response>(
            &resp_value
        ));

        let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

        if resp.status() == 200 {
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            // web_sys::console::log_1(json.as_ref());

            // Use serde to parse the JSON into a struct.
            let bq_response = serde_wasm_bindgen::from_value::<TableDataListResponse>(json);

            if bq_response.is_err() {
                console::log_1(&JsValue::from_str(&format!(
                    "error: {:?}",
                    bq_response.err().unwrap().to_string()
                )));
            } else {
                return Some(bq_response.unwrap());
            }
        }
        None
    }
}