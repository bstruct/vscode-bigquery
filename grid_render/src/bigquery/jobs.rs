use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

pub struct Jobs {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobConfiguration {
    #[serde(alias = "jobType")]
    pub job_type: String,
    pub query: JobConfigurationQuery,
    pub load: Option<JobConfigurationLoad>,
    pub copy: Option<JobConfigurationTableCopy>,
    pub extract: Option<JobConfigurationExtract>,
    #[serde(alias = "dryRun")]
    pub dry_run: bool,
    #[serde(alias = "jobTimeoutMs")]
    pub job_timeout_ms: Option<String>,
    // "labels": {
    //   string: string,
    //   ...
    // }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobConfigurationQuery {
    pub query: String,
    //   "destinationTable": {
    //     object (TableReference)
    //   },
    //   "tableDefinitions": {
    //     string: {
    //       object (ExternalDataConfiguration)
    //     },
    //     ...
    //   },
    //   "userDefinedFunctionResources": [
    //     {
    //       object (UserDefinedFunctionResource)
    //     }
    //   ],
    //   "createDisposition": string,
    //   "writeDisposition": string,
    //   "defaultDataset": {
    //     object (DatasetReference)
    //   },
    //   "priority": string,
    //   "preserveNulls": boolean,
    //   "allowLargeResults": boolean,
    //   "useQueryCache": boolean,
    //   "flattenResults": boolean,
    //   "maximumBillingTier": integer,
    //   "maximumBytesBilled": string,
    //   "useLegacySql": boolean,
    //   "parameterMode": string,
    //   "queryParameters": [
    //     {
    //       object (QueryParameter)
    //     }
    //   ],
    //   "schemaUpdateOptions": [
    //     string
    //   ],
    //   "timePartitioning": {
    //     object (TimePartitioning)
    //   },
    //   "rangePartitioning": {
    //     object (RangePartitioning)
    //   },
    //   "clustering": {
    //     object (Clustering)
    //   },
    //   "destinationEncryptionConfiguration": {
    //     object (EncryptionConfiguration)
    //   },
    //   "scriptOptions": {
    //     object (ScriptOptions)
    //   },
    //   "connectionProperties": [
    //     {
    //       object (ConnectionProperty)
    //     }
    //   ],
    //   "createSession": boolean,
    //   "systemVariables": {
    //     object (SystemVariables)
    //   }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobConfigurationLoad {}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobConfigurationTableCopy {}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobConfigurationExtract {}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatistics {
    // "creationTime": string,
    // "startTime": string,
    // "endTime": string,
    // "totalBytesProcessed": string,
    // "completionRatio": number,
    // "quotaDeferments": [
    //   string
    // ],
    // "query": {
    //   object (JobStatistics2)
    // },
    // "load": {
    //   object (JobStatistics3)
    // },
    // "extract": {
    //   object (JobStatistics4)
    // },
    // "totalSlotMs": string,
    // "reservationUsage": [
    //   {
    //     "name": string,
    //     "slotMs": string
    //   }
    // ],
    // "reservation_id": string,
    // "numChildJobs": string,
    // "parentJobId": string,
    // "scriptStatistics": {
    //   object (ScriptStatistics)
    // },
    // "rowLevelSecurityStatistics": {
    //   object (RowLevelSecurityStatistics)
    // },
    // "dataMaskingStatistics": {
    //   object (DataMaskingStatistics)
    // },
    // "transactionInfo": {
    //   object (TransactionInfo)
    // },
    // "sessionInfo": {
    //   object (SessionInfo)
    // },
    // "finalExecutionDurationMs": string
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatus {
    #[serde(alias = "errorResult")]
    pub error_result: Option<ErrorProto>,
    pub errors: Option<Vec<ErrorProto>>,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub kind: Option<String>,
    pub etag: Option<String>,
    pub id: Option<String>,
    #[serde(alias = "selfLink")]
    pub self_link: Option<String>,
    pub user_email: Option<String>,
    pub configuration: JobConfiguration,
    #[serde(alias = "jobReference")]
    pub job_reference: Option<JobReference>,
    pub statistics: Option<JobStatistics>,
    pub status: Option<JobStatus>,
    pub principal_subject: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QueryRequest {
    pub query: String,
    #[serde(alias = "maxResults")]
    pub max_results: Option<usize>,
    //incomplete
    // https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query#QueryRequest
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryResponseSessionInfo {
    #[serde(alias = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryResponseSessionDmlStats {
    #[serde(alias = "insertedRowCount")]
    pub inserted_row_count: String,
    #[serde(alias = "deletedRowCount")]
    pub deleted_row_count: String,
    #[serde(alias = "updatedRowCount")]
    pub updated_row_count: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub kind: String,
    pub schema: Option<TableSchema>,
    #[serde(alias = "jobReference")]
    pub job_reference: JobReference,
    #[serde(alias = "totalRows")]
    pub total_rows: String,
    #[serde(alias = "pageToken")]
    pub page_token: String,
    pub rows: Vec<serde_json::Value>,
    #[serde(alias = "totalBytesProcessed")]
    pub total_bytes_processed: String,
    #[serde(alias = "jobComplete")]
    pub job_complete: bool,
    pub errors: Vec<serde_json::Value>,
    #[serde(alias = "cacheHit")]
    pub cache_hit: bool,
    #[serde(alias = "numDmlAffectedRows")]
    pub num_dml_affected_rows: String,
    #[serde(alias = "sessionInfo")]
    pub session_info: QueryResponseSessionInfo,
    #[serde(alias = "dmlStats")]
    pub dml_stats: QueryResponseSessionDmlStats,
}

#[derive(Debug)]
pub struct GetQueryResultsRequest {
    pub project_id: String,
    pub job_id: String,
    pub start_index: Option<String>,
    pub page_token: Option<String>,
    pub max_results: Option<usize>,
    pub timeout_ms: Option<usize>,
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
    #[serde(alias = "type")]
    pub r#type: String,
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

    /* https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/insert
     */
    pub async fn insert(self: &Self, project_id: &str, request: Job) -> Option<Job> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.mode(web_sys::RequestMode::NoCors);
        let headers = web_sys::Headers::new().unwrap();
        // headers.set("Accept", "application/json").unwrap();
        headers.set("Content-Type", "application/json").unwrap();
        headers
            .set("Authorization", &format!("Bearer {}", &self.token))
            .unwrap();
        opts.headers(&headers);
        let body = serde_wasm_bindgen::to_value(&request).unwrap();
        opts.body(Some(&body));

        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/jobs",
            project_id
        );

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
            let bq_response = serde_wasm_bindgen::from_value::<Job>(json);

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

    /* https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query
     */
    pub async fn query(
        self: &Self,
        project_id: &str,
        request: QueryRequest,
    ) -> Option<QueryResponse> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.mode(web_sys::RequestMode::NoCors);
        let headers = web_sys::Headers::new().unwrap();
        // headers.set("Accept", "application/json").unwrap();
        headers.set("Content-Type", "application/json").unwrap();
        headers
            .set("Authorization", &format!("Bearer {}", &self.token))
            .unwrap();
        opts.headers(&headers);
        let body = serde_wasm_bindgen::to_value(&request).unwrap();
        opts.body(Some(&body));

        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/queries",
            project_id
        );

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
            let bq_response = serde_wasm_bindgen::from_value::<QueryResponse>(json);

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

        let mut url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/queries/{}",
            request.project_id, request.job_id
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
        None
    }
}
