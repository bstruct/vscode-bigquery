use serde::{Deserialize, Serialize};

use super::base::{TableSchema, TableReference};

pub struct Tables {
    token: String,
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