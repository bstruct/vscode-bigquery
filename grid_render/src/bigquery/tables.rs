use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

use super::base::{TableSchema, TableReference};

pub struct Tables {
    token: String,
}

impl Tables {
    pub fn new(token: &str) -> Tables {
        Tables {
            token: String::from(token),
        }
    }
    pub async fn get(
        self: &Self,
        request: TableReference,
    ) -> Option<Table> {
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
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets/{}/tables/{}",
            request.project_id.unwrap(), 
            request.dataset_id.unwrap(),
            request.table_id.unwrap(),
        );

        // if (request.location) { url.searchParams.append("location", request.location); }
        // if request.max_results.is_some() {
        //     url = format!("{}?maxResults={}", url, request.max_results.unwrap());
        // } else {
        //     url = format!("{}?maxResults=50", url);
        // }
        // if request.start_index.is_some() {
        //     url = format!("{}&startIndex={}", url, request.start_index.unwrap());
        // }

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

        let resp: web_sys::Response = wasm_bindgen::JsCast::dyn_into(resp_value).unwrap();

        if resp.status() == 200 {
            let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

            web_sys::console::log_1(json.as_ref());

            // Use serde to parse the JSON into a struct.
            let bq_response = serde_wasm_bindgen::from_value::<Table>(json);

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

// https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#resource:-table
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub kind: Option<String>,
    pub etag: Option<String>,
    pub id: Option<String>,
    #[serde(alias = "selfLink")]
    pub self_link: Option<String>,
    #[serde(alias = "tableReference")]
    pub table_reference: Option<TableReference>,
    #[serde(alias = "friendlyName")]
    pub friendly_name: Option<String>,
    pub description: Option<String>,
    // "labels": {
    //     string: string,
    //     ...
    //   },
    pub schema: Option<TableSchema>,
    // "timePartitioning": {
    //     object (TimePartitioning)
    //   },
    //   "rangePartitioning": {
    //     object (RangePartitioning)
    //   },
    //   "clustering": {
    //     object (Clustering)
    //   },
    #[serde(alias = "requirePartitionFilter")]
    pub require_partition_filter: Option<bool>,
    #[serde(alias = "numBytes")]
    pub num_bytes: Option<String>,
    #[serde(alias = "numLongTermBytes")]
    pub num_long_term_bytes: Option<String>,
    #[serde(alias = "numRows")]
    pub num_rows: Option<String>,
    #[serde(alias = "creationTime")]
    pub creation_time: Option<String>,
    #[serde(alias = "expirationTime")]
    pub expiration_time: Option<String>,
    #[serde(alias = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    pub r#type: Option<String>,

    // "view": {
    //     object (ViewDefinition)
    //   },
    //   "materializedView": {
    //     object (MaterializedViewDefinition)
    //   },
    //   "materializedViewStatus": {
    //     object (MaterializedViewStatus)
    //   },
    //   "externalDataConfiguration": {
    //     object (ExternalDataConfiguration)
    //   },
    pub location: Option<String>,
    //   "streamingBuffer": {
    //     object (Streamingbuffer)
    //   },
    //   "encryptionConfiguration": {
    //     object (EncryptionConfiguration)
    //   },
    //   "snapshotDefinition": {
    //     object (SnapshotDefinition)
    //   },
    //   "defaultCollation": string,
    #[serde(alias = "defaultCollation")]
    pub default_collation: Option<String>,
    //   "defaultRoundingMode": enum (RoundingMode),
    //   "cloneDefinition": {
    //     object (CloneDefinition)
    //   },
    #[serde(alias = "maxStaleness")]
    pub max_staleness: Option<String>,
    //   "tableConstraints": {
    //     object (TableConstraints)
    //   },
    //   "resourceTags": {
    //     string: string,
    //     ...
    //   }
}
