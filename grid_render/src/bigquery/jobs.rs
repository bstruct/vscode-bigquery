use std::ops::Index;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

use crate::parse_to_usize;

use super::base::TableSchema;

pub struct Jobs {
    token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobConfiguration {
    #[serde(alias = "jobType")]
    pub job_type: String,
    pub query: Option<JobConfigurationQuery>,
    pub load: Option<JobConfigurationLoad>,
    pub copy: Option<JobConfigurationTableCopy>,
    pub extract: Option<JobConfigurationExtract>,
    #[serde(alias = "dryRun")]
    pub dry_run: Option<bool>,
    #[serde(alias = "jobTimeoutMs")]
    pub job_timeout_ms: Option<String>,
    // "labels": {
    //   string: string,
    //   ...
    // }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobConfigurationLoad {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobConfigurationTableCopy {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobConfigurationExtract {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobStatistics2 {
    //     "queryPlan": [
    //     {
    //       object (ExplainQueryStage)
    //     }
    //   ],
    //   "estimatedBytesProcessed": string,
    //   "timeline": [
    //     {
    //       object (QueryTimelineSample)
    //     }
    //   ],
    //   "totalPartitionsProcessed": string,
    //   "totalBytesProcessed": string,
    //   "totalBytesProcessedAccuracy": string,
    //   "totalBytesBilled": string,
    //   "billingTier": integer,
    //   "totalSlotMs": string,
    //   "reservationUsage": [
    //     {
    //       "name": string,
    //       "slotMs": string
    //     }
    //   ],
    //   "cacheHit": boolean,
    //   "referencedTables": [
    //     {
    //       object (TableReference)
    //     }
    //   ],
    //   "referencedRoutines": [
    //     {
    //       object (RoutineReference)
    //     }
    //   ],
    //   "schema": {
    //     object (TableSchema)
    //   },
    //   "numDmlAffectedRows": string,
    #[serde(alias = "numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,

    #[serde(alias = "dmlStats")]
    pub dml_stats: Option<DmlStats>,

    //   "undeclaredQueryParameters": [
    //     {
    //       object (QueryParameter)
    //     }
    //   ],
    #[serde(alias = "statementType")]
    pub statement_type: String,
    //   "ddlOperationPerformed": string,
    //   "ddlTargetTable": {
    //     object (TableReference)
    //   },
    //   "ddlDestinationTable": {
    //     object (TableReference)
    //   },
    //   "ddlTargetRowAccessPolicy": {
    //     object (RowAccessPolicyReference)
    //   },
    //   "ddlAffectedRowAccessPolicyCount": string,
    //   "ddlTargetRoutine": {
    //     object (RoutineReference)
    //   },
    //   "ddlTargetDataset": {
    //     object (DatasetReference)
    //   },
    //   "mlStatistics": {
    //     object (MlStatistics)
    //   },
    //   "exportDataStatistics": {
    //     object (ExportDataStatistics)
    //   },
    //   "externalServiceCosts": [
    //     {
    //       object (ExternalServiceCost)
    //     }
    //   ],
    //   "biEngineStatistics": {
    //     object (BiEngineStatistics)
    //   },
    //   "loadQueryStatistics": {
    //     object (LoadQueryStatistics)
    //   },
    //   "dclTargetTable": {
    //     object (TableReference)
    //   },
    //   "dclTargetView": {
    //     object (TableReference)
    //   },
    //   "dclTargetDataset": {
    //     object (DatasetReference)
    //   },
    //   "searchStatistics": {
    //     object (SearchStatistics)
    //   },
    //   "vectorSearchStatistics": {
    //     object (VectorSearchStatistics)
    //   },
    //   "performanceInsights": {
    //     object (PerformanceInsights)
    //   },
    //   "queryInfo": {
    //     object (QueryInfo)
    //   },
    //   "sparkStatistics": {
    //     object (SparkStatistics)
    //   },
    //   "transferredBytes": string,
    //   "materializedViewStatistics": {
    //     object (MaterializedViewStatistics)
    //   },
    //   "metadataCacheStatistics": {
    //     object (MetadataCacheStatistics)
    //   }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DmlStats {
    #[serde(alias = "insertedRowCount")]
    pub inserted_row_count: Option<String>,
    #[serde(alias = "deletedRowCount")]
    pub deleted_row_count: Option<String>,
    #[serde(alias = "updatedRowCount")]
    pub updated_row_count: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobStatistics {
    #[serde(alias = "creationTime")]
    pub creation_time: Option<String>,
    #[serde(alias = "startTime")]
    pub start_time: Option<String>,
    #[serde(alias = "endTime")]
    pub end_time: Option<String>,
    #[serde(alias = "totalBytesProcessed")]
    pub total_bytes_processed: Option<String>,
    // "completionRatio": number,
    // "quotaDeferments": [
    //   string
    // ],
    pub query: Option<JobStatistics2>,
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
    #[serde(alias = "numChildJobs")]
    pub num_child_jobs: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobStatus {
    #[serde(alias = "errorResult")]
    pub error_result: Option<ErrorProto>,
    pub errors: Option<Vec<ErrorProto>>,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobCreationReason {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub kind: Option<String>,
    pub etag: Option<String>,
    pub id: Option<String>,
    #[serde(alias = "selfLink")]
    pub self_link: Option<String>,
    pub user_email: Option<String>,
    pub configuration: Option<JobConfiguration>,
    #[serde(alias = "jobReference")]
    pub job_reference: Option<JobReference>,
    pub statistics: Option<JobStatistics>,
    pub status: Option<JobStatus>,
    pub principal_subject: Option<String>,
    //jobCreationReason
    #[serde(alias = "jobCreationReason")]
    pub job_creation_reason: Option<JobCreationReason>,
}

impl Job {
    // https://cloud.google.com/bigquery/docs/reference/rest/v2/Job#jobstatistics2
    pub(crate) fn is_query_script(&self) -> bool {
        if let Some(statement_type) = self.get_statement_type() {
            return statement_type == "SCRIPT";
        }

        false
    }

    pub(crate) fn is_unsupported_type(&self) -> bool {
        !(self.is_query_script() || self.is_query_select() || self.is_dml_statement())
    }

    // https://cloud.google.com/bigquery/docs/reference/rest/v2/Job#jobstatistics2
    pub(crate) fn is_query_select(&self) -> bool {
        if let Some(statement_type) = self.get_statement_type() {
            return statement_type == "SELECT";
        }

        false
    }

    // https://cloud.google.com/bigquery/docs/reference/rest/v2/Job#jobstatistics2
    pub(crate) fn is_dml_statement(&self) -> bool {
        if let Some(statement_type) = self.get_statement_type() {
            return ["INSERT", "UPDATE", "DELETE", "MERGE"]
                .iter()
                .any(|c| c == &statement_type);
        }

        false
    }

    pub(crate) fn is_multi_query_job(&self) -> bool {
        if let Some(stats) = self.statistics.as_ref() {
            if let Some(num_child_jobs) = parse_to_usize(stats.num_child_jobs.clone()) {
                return num_child_jobs > 1;
            }
        }

        false
    }
    pub(crate) fn has_error(&self) -> bool {
        if let Some(status) = self.status.as_ref() {
            if let Some(error_result) = status.error_result.as_ref() {
                return error_result.message.is_some();
            }
        }

        false
    }
    pub(crate) fn get_statement_type(&self) -> Option<String> {
        if let Some(statistics) = &self.statistics {
            if let Some(query) = &statistics.query {
                return Some(query.statement_type.clone());
            }
        }
        None
    }

    pub(crate) fn get_dml_stats(&self) -> Option<DmlStats> {
        if let Some(statistics) = &self.statistics {
            if let Some(query) = &statistics.query {
                return query.dml_stats.clone();
            }
        }
        None
    }

    pub(crate) fn is_complete(&self) -> bool {
        if let Some(status) = &self.status {
            // Valid states include 'PENDING', 'RUNNING', and 'DONE'.
            return status.state == "DONE";
        }

        true
    }

    pub(crate) fn is_query_select_and_complete(&self) -> bool {
        self.is_query_select() && self.is_complete()
    }
}

#[derive(Debug, Serialize)]
pub struct QueryRequest {
    pub query: String,
    // #[serde(alias = "maxResults")]
    // pub max_results: Option<u64>,
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
    pub cache_hit: Option<bool>,
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

#[derive(Debug)]
pub struct GetJobRequest {
    pub project_id: String,
    pub job_id: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorProto {
    pub reason: Option<String>,
    pub location: Option<String>,
    #[serde(alias = "debugInfo")]
    pub debug_info: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobReference {
    #[serde(alias = "projectId")]
    pub project_id: String,
    #[serde(alias = "jobId")]
    pub job_id: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueryResultsResponse {
    pub kind: String,
    pub etag: String,
    pub schema: Option<TableSchema>,
    #[serde(alias = "jobReference")]
    pub job_reference: JobReference,
    #[serde(alias = "totalRows")]
    pub total_rows: Option<String>,
    #[serde(alias = "pageToken")]
    pub page_token: Option<String>,
    pub rows: Option<Vec<serde_json::Value>>,
    #[serde(alias = "totalBytesProcessed")]
    pub total_bytes_processed: String,
    #[serde(alias = "jobComplete")]
    pub job_complete: bool,
    pub errors: Option<Vec<ErrorProto>>,
    #[serde(alias = "cacheHit")]
    pub cache_hit: Option<bool>,
    #[serde(alias = "numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) enum Projection {
    MINIMAL,
    FULL,
}

#[derive(Debug, Serialize)]
pub struct GetListRequest {
    #[serde(alias = "projectId")]
    pub project_id: String,
    pub max_results: Option<usize>,
    pub projection: Option<Projection>,
    #[serde(alias = "parentJobId")]
    pub parent_job_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetListResponse {
    pub kind: String,
    pub etag: String,
    #[serde(alias = "nextPageToken")]
    pub next_page_token: Option<String>,
    pub jobs: Option<Vec<Job>>,
    pub unreachable: Option<Vec<String>>,
}

impl Jobs {
    pub fn new(token: &str) -> Jobs {
        Jobs {
            token: String::from(token),
        }
    }

    // POST functions need CORS requests. :/

    // /* https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/insert
    //  */
    // pub async fn insert(self: &Self, project_id: &str, request: Job) -> Option<Job> {
    //     // headers
    //     let headers = web_sys::Headers::new().unwrap();
    //     //Access-Control-Allow-Origin
    //     headers.set("Access-Control-Request-Headers", "Authorization").unwrap();
    //     headers.set("Accept", "application/json").unwrap();
    //     headers
    //         .set("Content-Type", "application/json; charset=utf-8")
    //         .unwrap();
    //     headers
    //         .set("Authorization", &format!("Bearer {}", &self.token))
    //         .unwrap();

    //     // console::log_1(&JsValue::from_str(&headers.get("Accept").unwrap().unwrap()));
    //     // console::log_1(&JsValue::from_str(&headers.get("Content-Type").unwrap().unwrap()));
    //     // console::log_1(&JsValue::from_str(&headers.get("Authorization").unwrap().unwrap()));

    //     //body
    //     let body = serde_wasm_bindgen::to_value(&request).unwrap();

    //     //method, mode and headers
    //     let mut opts = web_sys::RequestInit::new();
    //     opts.method("POST")
    //         // .mode(web_sys::RequestMode::Cors)
    //         .headers(&headers)
    //         .body(Some(&body));

    //     let url = format!(
    //         "https://content-bigquery.googleapis.com/bigquery/v2/projects/{}/jobs",
    //         project_id
    //     );

    //     let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();

    //     let window = web_sys::window().unwrap();
    //     let resp_value = JsFuture::from(window.fetch_with_request(&request))
    //         .await
    //         .unwrap();

    //     assert!(wasm_bindgen::JsCast::is_instance_of::<web_sys::Response>(
    //         &resp_value
    //     ));
    //     let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

    //     if resp.status() == 200 {
    //         let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    //         // Use serde to parse the JSON into a struct.
    //         let bq_response = serde_wasm_bindgen::from_value::<Job>(json);

    //         if bq_response.is_err() {
    //             console::log_1(&JsValue::from_str(&format!(
    //                 "error: {:?}",
    //                 bq_response.err().unwrap().to_string()
    //             )));
    //         } else {
    //             return Some(bq_response.unwrap());
    //         }
    //     }
    //     // else {
    //     //     console::log_1(&JsValue::from_str(&format!(
    //     //         "response: {:?}",
    //     //         wasm_bindgen::JsValue::from(&resp).as_string()
    //     //     )));
    //     // }

    //     None
    // }

    // /* https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query
    //  */
    // pub async fn query(
    //     self: &Self,
    //     project_id: &str,
    //     request: QueryRequest,
    // ) -> Option<QueryResponse> {
    //     let mut opts = web_sys::RequestInit::new();
    //     opts.method("POST");
    //     opts.mode(web_sys::RequestMode::NoCors);
    //     let headers = web_sys::Headers::new().unwrap();
    //     // headers.set("Accept", "application/json").unwrap();
    //     headers.set("Content-Type", "application/json").unwrap();
    //     headers
    //         .set("Authorization", &format!("Bearer {}", &self.token))
    //         .unwrap();
    //     opts.headers(&headers);
    //     let body = serde_wasm_bindgen::to_value(&request).unwrap();
    //     opts.body(Some(&body));

    //     let url = format!(
    //         "https://bigquery.googleapis.com/bigquery/v2/projects/{}/queries",
    //         project_id
    //     );

    //     let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();

    //     let window = web_sys::window().unwrap();
    //     let resp_value = JsFuture::from(window.fetch_with_request(&request))
    //         .await
    //         .unwrap();

    //     assert!(wasm_bindgen::JsCast::is_instance_of::<web_sys::Response>(
    //         &resp_value
    //     ));
    //     let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

    //     if resp.status() == 200 {
    //         let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    //         // Use serde to parse the JSON into a struct.
    //         let bq_response = serde_wasm_bindgen::from_value::<QueryResponse>(json);

    //         if bq_response.is_err() {
    //             console::log_1(&JsValue::from_str(&format!(
    //                 "error: {:?}",
    //                 bq_response.err().unwrap().to_string()
    //             )));
    //         } else {
    //             return Some(bq_response.unwrap());
    //         }
    //     }

    //     None
    // }

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
        url = format!("{}&formatOptions.useInt64Timestamp=false", url);

        // console::log_1(&JsValue::from_str(&url));

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

        // console::log_1(resp_value.as_ref());

        let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

        if resp.status() == 200 {
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            // console::log_1(json.as_ref());
            // console::log_1(&JsValue::from_str(&format!(
            //     "json: {:?}",
            //     json
            // )));

            // Use serde to parse the JSON into a struct.
            let bq_response = serde_wasm_bindgen::from_value::<GetQueryResultsResponse>(json);

            // console::log_1(&JsValue::from_str(&format!(
            //     "bq_response: {:?}",
            //     bq_response
            // )));

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
    https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/list
    */
    pub async fn get(self: &Self, request: GetJobRequest) -> Option<Job> {
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
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/jobs/{}",
            request.project_id, request.job_id
        );

        if request.location.is_some() {
            url = format!("{}?location={}", url, request.location.as_ref().unwrap());
        }

        console::log_1(&JsValue::from_str(&url));

        let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        console::log_1(resp_value.as_ref());

        assert!(wasm_bindgen::JsCast::is_instance_of::<web_sys::Response>(
            &resp_value
        ));

        let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

        if resp.status() == 200 {
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            // console::log_1(json.as_ref());

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

    /*
    https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/list
    */
    pub async fn get_list(self: &Self, request: GetListRequest) -> Option<GetListResponse> {
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
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/jobs",
            request.project_id
        );

        // if (request.location) { url.searchParams.append("location", request.location); }
        if request.max_results.is_some() {
            url = format!("{}?maxResults={}", url, request.max_results.unwrap());
        } else {
            url = format!("{}?maxResults=500", url);
        }
        if request.parent_job_id.is_some() {
            url = format!("{}&parentJobId={}", url, request.parent_job_id.unwrap());
        }
        if request.projection.is_some() {
            url = format!(
                "{}&projection={}",
                url,
                match request.projection.unwrap() {
                    Projection::MINIMAL => "minimal",
                    Projection::FULL => "full",
                }
            );
        }

        //AIzaSyAa8yy0GdcGPHdtD083HiGGx_S0vMPScDM
        // url = format!("{}&key=AIzaSyAa8yy0GdcGPHdtD083HiGGx_S0vMPScDM", url);

        console::log_1(&JsValue::from_str(&url));

        let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        // assert!(wasm_bindgen::JsCast::is_instance_of::<web_sys::Response>(
        //     &resp_value
        // ));

        let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

        if resp.status() == 200 {
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            // console::log_1(json.as_ref());

            // Use serde to parse the JSON into a struct.
            let bq_response = serde_wasm_bindgen::from_value::<GetListResponse>(json);

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

#[cfg(test)]
mod tests {

    use js_sys::JSON;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    //* try out some stuff */
    #[wasm_bindgen_test]
    pub fn deserialize_json_to_job() {
        let json_job_response = r#"
            {
                "kind": "bigquery#job",
                "etag": "i5chOreAELANLzXLPFIRZA==",
                "id": "xxx-project-1:EU.0db7c357-9d1b-xxxx-a641-ca0f009342df",
                "selfLink": "https://bigquery.googleapis.com/bigquery/v2/projects/damiao-project-1/jobs/0db7c357-9d1b-48c9-a641-ca0f009342df?location=EU",
                "user_email": "xxxx@gmail.com",
                "configuration": {
                    "query": {
                        "query": "WITH tBase AS (\nSELECT \n    pimExportDate, \n    Combi_number,\n    Width_accessoires,\n    Lining,    \n    Width_accessoires,\n    Additional_info,\n    Sleeve_Length,\n    Pim_Value,\n    Colour_PDP,\n    Not_searchable,\n    ROW_NUMBER() OVER(ORDER BY Combi_number ASC) AS row_number\nFROM `damiao-project-1.PvhTest.PimExport` pim\nWHERE \n    pimExportDate <= \"2022-03-23\"\n)\n\n-- projects/damiao-project-1/topics/test_topic_no_schema\n\nSELECT \n    (\n        SELECT AS STRUCT\n            CAST(row_number AS STRING) AS row_number,\n            \"dsdfdsd\" AS data_type\n    ) AS attributes,\n    TO_JSON(tBase) AS data,\n\nFROM tBase\nLIMIT 10;",
                        "destinationTable": {
                            "projectId": "xxx-project-1",
                            "datasetId": "_9b1179fxxxxad7e9c3af3ff30",
                            "tableId": "anondc2ef39266xxx4b35af6572d83c79e4c84f33e2cc31c8a0a0941"
                        },
                        "writeDisposition": "WRITE_TRUNCATE",
                        "priority": "INTERACTIVE",
                        "useQueryCache": true,
                        "useLegacySql": false
                    },
                    "jobType": "QUERY"
                },
                "jobReference": {
                    "projectId": "xxxx-project-1",
                    "jobId": "0db7c357-9d1b-xxxx-a641-ca0f009342df",
                    "location": "EU"
                },
                "statistics": {
                    "creationTime": "1700418877577",
                    "startTime": "1700418877849",
                    "endTime": "1700418877968",
                    "totalBytesProcessed": "0",
                    "query": {
                        "totalBytesProcessed": "0",
                        "totalBytesBilled": "0",
                        "cacheHit": true,
                        "statementType": "SELECT"
                    }
                },
                "status": {
                    "state": "DONE"
                },
                "principal_subject": "user:xxxx@gmail.com",
                "jobCreationReason": {
                    "code": "REQUESTED"
                }
            }
        "#;

        let job_response = JSON::parse(json_job_response).unwrap();
        let parse_event = &serde_wasm_bindgen::from_value::<super::Job>(job_response).unwrap();

        assert_eq!(parse_event.kind.as_ref().unwrap(), "bigquery#job");
        assert_eq!(
            parse_event.etag.as_ref().unwrap(),
            "i5chOreAELANLzXLPFIRZA=="
        );
        assert_eq!(
            parse_event.principal_subject,
            Some("user:xxxx@gmail.com".to_owned())
        );
        assert_eq!(
            parse_event.job_reference.as_ref().unwrap().job_id,
            "0db7c357-9d1b-xxxx-a641-ca0f009342df".to_owned()
        );
        assert_eq!(parse_event.configuration.as_ref().unwrap().query.as_ref().unwrap().query, "WITH tBase AS (\nSELECT \n    pimExportDate, \n    Combi_number,\n    Width_accessoires,\n    Lining,    \n    Width_accessoires,\n    Additional_info,\n    Sleeve_Length,\n    Pim_Value,\n    Colour_PDP,\n    Not_searchable,\n    ROW_NUMBER() OVER(ORDER BY Combi_number ASC) AS row_number\nFROM `damiao-project-1.PvhTest.PimExport` pim\nWHERE \n    pimExportDate <= \"2022-03-23\"\n)\n\n-- projects/damiao-project-1/topics/test_topic_no_schema\n\nSELECT \n    (\n        SELECT AS STRUCT\n            CAST(row_number AS STRING) AS row_number,\n            \"dsdfdsd\" AS data_type\n    ) AS attributes,\n    TO_JSON(tBase) AS data,\n\nFROM tBase\nLIMIT 10;".to_owned());
        assert_eq!(parse_event.configuration.as_ref().unwrap().dry_run, None);
        assert_eq!(parse_event.status.as_ref().unwrap().state, "DONE");
        assert!(parse_event.statistics.is_some());
        assert_eq!(
            parse_event.job_creation_reason.as_ref().unwrap().code,
            "REQUESTED"
        );
    }
}
