use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub fields: Vec<TableFieldSchema>,
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

impl TableFieldSchema {
    pub fn is_array(&self) -> bool {
        self.mode.as_ref().is_some() && self.mode.as_ref().unwrap() == "REPEATED"
    }
    pub fn is_complex_object(&self) -> bool {
        self.r#type == "RECORD"
    }

    pub(crate) fn calc_number_cols(&self) -> usize {
        if self.is_array() && self.is_complex_object() {

            // start with 1 for the index column (#)
            let mut count = 1;

            for c in self.fields.as_ref().unwrap() {
                count += c.calc_number_cols();
            }

            count
        } else {
            if self.is_complex_object() {
                2
            } else {
                1
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableReference {
    #[serde(alias = "projectId")]
    pub project_id: String,
    #[serde(alias = "datasetId")]
    pub dataset_id: String,
    #[serde(alias = "tableId")]
    pub table_id: String,
}
