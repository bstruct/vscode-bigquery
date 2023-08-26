use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

pub struct Jobs {
    token: String,
}

pub struct GetQueryResultsRequest {
    pub project_id: String,
    pub job_id: String,
    pub start_index: Option<String>,
    pub page_token: Option<String>,
    pub max_results: Option<u8>,
    pub timeout_ms: Option<u8>,
    pub location: Option<String>,
    // formatOptions: DataFormatOptions;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorProto {
    pub reason: String,
    pub location: String,
    #[serde(alias = "debugInfo")]
    pub debug_info: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobReference {
    #[serde(alias = "projectId")]
    pub project_id: String,
    #[serde(alias = "jobId")]
    pub job_id: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFieldSchema {
    pub name: String,
    // pub type: String,
    pub mode: Option<String>,
    pub fields: Option<Vec<TableFieldSchema>>,
    pub description: Option<String>,
    // pub policyTags: xxx,
    // pub policyTags.names[]: xxx,
    #[serde(alias = "maxLength")]
    pub max_length: Option<String>,
    pub precision: Option<String>,
    pub scale: Option<String>,
    // pub roundingMode: xxx,
    pub collation: Option<String>,
    #[serde(alias = "defaultValueExpression")]
    pub default_value_expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub fields: Vec<TableFieldSchema>,
}

#[derive(Debug, Deserialize)]
pub struct GetQueryResultsResponse {
    pub kind: String,
    pub etag: String,
    pub schema: Option<TableSchema>,
    #[serde(alias = "jobReference")]
    pub job_reference: JobReference,
    #[serde(alias = "totalRows")]
    pub total_rows: String,
    #[serde(alias = "pageToken")]
    pub page_token: Option<String>,
    pub rows: Vec<serde_json::Value>,
    #[serde(alias = "totalBytesProcessed")]
    pub total_bytes_processed: String,
    #[serde(alias = "jobComplete")]
    pub job_complete: bool,
    pub errors: Option<Vec<ErrorProto>>,
    #[serde(alias = "cacheHit")]
    pub cache_hit: bool,
    #[serde(alias = "numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,
}

impl Jobs {
    pub fn new(token: &str) -> Jobs {
        Jobs {
            token: String::from(token),
        }
    }

    /*
    https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/getQueryResults#http-request
    https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html
    https://crates.io/crates/serde-wasm-bindgen
    */
    pub async fn get_query_results(
        self: &Self,
        request: GetQueryResultsRequest,
    ) -> Option<GetQueryResultsResponse> {
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

        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/queries/{}",
            request.project_id, request.job_id
        );

        // if (request.location) { url.searchParams.append("location", request.location); }
        // if (request.maxResults !== null) { url.searchParams.append("maxResults", request.maxResults.toString()); }
        // if (request.pageToken) { url.searchParams.append("pageToken", request.pageToken); }
        // if (request.startIndex) { url.searchParams.append("startIndex", request.startIndex); }
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
            // Use serde to parse the JSON into a struct.
            let bq_response = serde_wasm_bindgen::from_value::<GetQueryResultsResponse>(json);

            if bq_response.is_err() {
                console::log_1(&JsValue::from_str(&format!(
                    "error: {:?}",
                    bq_response.err().unwrap().to_string()
                )));
            } else {
                return Some(bq_response.unwrap());
            }
        }
        // else {
        // let text_response = JsFuture::from(resp.text().unwrap()).await.unwrap();

        // let job_reference = JobReference {
        //     project_id: String::from("1"),
        //     location: String::from("2"),
        //     job_id: String::from("3"),
        // };

        // Some(GetQueryResultsResponse {
        //     kind: text_response.as_string().unwrap(),
        //     etag: String::from("sss"),
        //     total_rows: String::from("sss"),
        //     total_bytes_processed: String::from("sss"),
        //     cache_hit: true,
        //     job_complete: true,
        //     num_dml_affected_rows: None,
        //     page_token: None,
        //     schema: None,
        //     errors: None,
        //     job_reference: job_reference,
        // })
        // }
        None
    }
}
