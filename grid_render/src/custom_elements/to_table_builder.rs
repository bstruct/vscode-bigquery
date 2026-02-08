use website_component_table::{
    TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup, TableRow, TableStyle,
    TableValue,
};

use crate::bigquery::{base::TableFieldSchema, jobs::GetQueryResultsResponse};

impl GetQueryResultsResponse {
    pub(crate) fn to_table_builder(&self) -> TableBuilder {
        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: self.get_columns(),
            rows: self.get_rows_new(),
        }
    }

    fn get_columns(&self) -> Vec<TableColumnDefinition> {
        if let Some(schema) = &self.schema {
            schema
                .fields
                .iter()
                .map(|field| field.to_table_column_definition())
                .collect()
        } else {
            vec![]
        }
    }

    fn get_rows_new(&self) -> Vec<TableRow> {
        if let Some(rows) = &self.rows {
            rows.iter().map(|row| json_value_to_row(row)).collect()
        } else {
            vec![]
        }
    }
}

fn json_value_to_row(value: &serde_json::Value) -> TableRow {
    let cells = if let Some(array) = value.as_array() {
        array
            .iter()
            .map(|cell| json_value_to_table_value(cell))
            .collect()
    } else {
        vec![]
    };

    TableRow { cells }
}

fn json_value_to_table_value(value: &serde_json::Value) -> TableValue {
    // value.as_str().unwrap_or_default().to_string()
    TableValue::String(value.as_str().unwrap_or_default().to_string())
}

impl TableFieldSchema {
    pub(crate) fn to_table_column_definition(&self) -> TableColumnDefinition {
        if let Some(fields) = &self.fields {
            TableColumnDefinition::Group(TableColumnGroup {
                text: self.name.clone(),
                name: self.name.clone(),
                columns: fields
                    .iter()
                    .map(|field| field.to_table_column_definition())
                    .collect(),
            })
        } else {
            TableColumnDefinition::Column(TableColumn {
                text: self.name.clone(),
                name: self.name.clone(),
                width_px: 100,
            })
        }
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn place_bq_table_rows_test_3() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test3.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let table_builder = complex_object_array_test.to_table_builder();

        assert_eq!(table_builder.rows.len(), 1);
        // assert_eq!(table_builder.columns.len(), 67);

        // // check if for every column that contains "#", the row has the number "1" for that same index.
        // let mut index = 0;
        // for h in header {
        //     if h.contains("#") {
        //         let value = rows[0].cells[index].clone();
        //         assert!(match value {
        //             TableValue::Index(v) => v == 1,
        //             _ => false,
        //         });
        //     }
        //     index += 1;
        // }
    }

}