use wasm_bindgen::JsValue;
use website_component_table::{
    TableBuilder, TableColumn, TableColumnDefinition, TableRow, TableStyle, TableValue,
};

use super::{
    bq_query_custom_element::BigqueryQueryCustomElement,
    bq_table_custom_element::BigqueryTableCustomElement,
};
use crate::{
    bigquery::{
        base::{TableFieldSchema, TableSchema},
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
        let header = self.get_header();
        let number_columns = header.len();
        let page_start_index = bq_query_requested.get_page_start_index();

        let (rows_in_page, rows) = self.get_rows(number_columns, page_start_index);
        let rows_total = self.get_rows_total();

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "rows_total: {:?}, rows_in_page: {}, rows len: {}",
            rows_total,
            rows_in_page,
            rows.len()
        )));

        let table_builder = TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: header
                .iter()
                .map(|h| {
                    TableColumnDefinition::Column(TableColumn {
                        name: h.clone(),
                        text: h.clone(),
                        width_px: 200,
                    })
                })
                .collect(),
            rows,
        };

        bq_query_requested.with_table_info(Some(rows_in_page), rows_total, Some(table_builder))
    }

    fn get_header(&self) -> Vec<String> {
        assert!(self.schema.is_some(), "unexpected empty schema");

        let schema = self.schema.as_ref().unwrap();
        get_bq_table_header(&schema)
    }

    fn get_rows(&self, _number_columns: usize, page_start_index: usize) -> (usize, Vec<TableRow>) {
        assert!(self.schema.is_some(), "unexpected empty schema");
        let schema = self.schema.as_ref().unwrap();

        let rows_in_page = if self.rows.is_some() {
            self.rows.as_ref().unwrap().len()
        } else {
            0
        };

        // let number_rows = calculate_number_rows(&self.rows);

        let mut rows: Vec<TableRow> = vec![];

        place_bq_table_rows(
            &mut rows,
            &schema.fields,
            &self.rows,
            0,
            0,
            true,
            page_start_index,
        );

        (rows_in_page, rows.to_owned())
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
        let header = self.get_header();
        let number_columns = header.len();
        let page_start_index = bq_table_element.get_page_start_index();

        let (rows_in_page, rows) = self.get_rows(number_columns, page_start_index, response_rows);
        let rows_total = self.get_rows_total();

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "rows_total: {}, rows_in_page: {}, rows len: {}",
            rows_total,
            rows_in_page,
            rows.len()
        )));

        let table_builder = TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: header
                .iter()
                .map(|h| {
                    TableColumnDefinition::Column(TableColumn {
                        name: h.clone(),
                        text: h.clone(),
                        width_px: 200,
                    })
                })
                .collect(),
            rows,
        };

        bq_table_element.with_table_info(Some(rows_in_page), Some(rows_total), Some(table_builder))
    }

    fn get_rows(
        &self,
        _number_columns: usize,
        page_start_index: usize,
        response_rows: &Option<TableDataListResponse>,
    ) -> (usize, Vec<TableRow>) {
        assert!(self.schema.is_some(), "unexpected empty schema");
        let schema = self.schema.as_ref().unwrap();

        let origin_rows = &response_rows.as_ref().expect("rows not found").rows;
        if let Some(origin_rows) = origin_rows {
            let rows_in_page = origin_rows.len();

            // let number_rows = calculate_number_rows(&Some(origin_rows.to_owned()));
            let mut rows: Vec<TableRow> = vec![];

            place_bq_table_rows(
                &mut rows,
                &schema.fields,
                &Some(origin_rows.to_owned()),
                0,
                0,
                true,
                page_start_index,
            );

            (rows_in_page, rows.to_owned())
        } else {
            (0, vec![])
        }
    }

    fn get_header(&self) -> Vec<String> {
        assert!(self.schema.is_some(), "unexpected empty schema");

        let schema = self.schema.as_ref().unwrap();
        get_bq_table_header(&schema)
    }

    fn get_rows_total(&self) -> usize {
        parse_to_usize(Some(self.num_rows.clone().unwrap_or(String::from("0")))).unwrap_or(0)
    }
}

pub fn from_schema_value(
    field_schema: &TableFieldSchema,
    value: &Option<serde_json::Value>,
) -> TableValue {
    match field_schema.r#type.as_str() {
        "TIMESTAMP" => TableValue::String(timestamp_to_value(value).unwrap_or_default()),
        _ => TableValue::String(
            value
                .as_ref()
                .unwrap_or(&serde_json::Value::Null)
                .to_string(),
        ),
    }
}

fn get_bq_table_header(schema: &TableSchema) -> Vec<String> {
    let mut header_columns = Vec::new();
    header_columns.push(String::from("#"));

    append_bq_table_header(&mut header_columns, &schema.fields, &None);

    header_columns
}

fn append_bq_table_header(
    header_columns: &mut Vec<String>,
    fields: &Vec<TableFieldSchema>,
    parent_name: &Option<String>,
) {
    for field in fields {
        if field.is_array() {
            if parent_name.as_ref().is_some() {
                header_columns.push(format!(
                    "{}.{}.#",
                    parent_name.as_ref().unwrap(),
                    &field.name
                ));
            } else {
                header_columns.push(format!("{}.#", &field.name));
            }
        }
        let inner_name = match parent_name.as_ref() {
            Some(n) => format!("{}.{}", n, field.name),
            None => String::from(field.name.clone()),
        };
        if field.is_complex_object() {
            if field.fields.is_some() {
                append_bq_table_header(
                    header_columns,
                    field.fields.as_ref().unwrap(),
                    &Some(inner_name),
                );
            }
        } else {
            header_columns.push(inner_name);
        }
    }
}

fn place_bq_table_rows(
    rows: &mut Vec<TableRow>,
    schema_fields: &Vec<TableFieldSchema>,
    data_rows: &Option<Vec<serde_json::Value>>,
    array_row_index: usize,
    array_col_index: usize,
    include_index_column: bool,
    page_start_index: usize,
) -> (usize, usize) {
    // 2 sets of variables are in use.
    // "data_..." to control the position of the data
    // "array_.." to control the position and increments of the TableItem array

    // control the vertical position of the array
    let mut array_row_increment = 0;

    // variable to move horizontally the placement in the TableItem array
    let mut array_col_increment = 0;

    if data_rows.is_some() {
        let data_rows_len = data_rows.as_ref().unwrap().len();
        let data_rows = data_rows.as_ref().unwrap();

        //
        for data_row_index in 0..data_rows_len {
            let data_row = &data_rows[data_row_index];

            //when the data row has inner arrays, the max size of the inner array(s) is controlled here
            let mut array_max_inner_row_increment = 0;

            //reset the variable of horizontal movement in a new data row
            array_col_increment = 0;

            //index column
            if include_index_column {
                rows[array_row_index + array_row_increment].cells
                    [array_col_index + array_col_increment] =
                    TableValue::Index(data_row_index + 1 + page_start_index);
                array_col_increment += 1;
            }

            // go through the schema of the data
            for col_index in 0..schema_fields.len() {
                let field_schema = &schema_fields[col_index];
                let value = data_row.pointer(&format!("/f/{}/v", col_index));

                let value = match value {
                    Some(v) => Some(v.clone()),
                    None => None,
                };

                if field_schema.is_array() && field_schema.is_complex_object() {
                    if value.as_ref().is_some()
                        && value.as_ref().unwrap().is_array()
                        && value.as_ref().unwrap().as_array().unwrap().len() > 0
                    {
                        let inner_schema_fields = &field_schema.fields.clone().unwrap();
                        let inner_data_rows = value
                            .as_ref()
                            .unwrap()
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|i| i.pointer("/v").unwrap().clone())
                            .collect::<Vec<serde_json::Value>>();

                        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                        //         "field_schema: {:?}, is_array {}, is_complex_object: {}, value: {:?}, inner_data_rows: {:?}",
                        //         field_schema,
                        //         field_schema.is_array(),
                        //         field_schema.is_complex_object(),
                        //         value,
                        //         inner_data_rows
                        //     )));

                        let positions = place_bq_table_rows(
                            rows,
                            inner_schema_fields,
                            &Some(inner_data_rows.to_owned()),
                            array_row_index + array_row_increment,
                            array_col_index + array_col_increment,
                            true,
                            0,
                        );
                        //establish the max rows to progress
                        array_max_inner_row_increment =
                            match array_max_inner_row_increment > positions.0 {
                                true => array_max_inner_row_increment,
                                false => positions.0,
                            };
                        //move the col index further
                        array_col_increment += positions.1;
                    } else {
                        let count_inner_columns = field_schema.calc_number_cols();
                        array_col_increment += count_inner_columns;
                    }
                } else {
                    if field_schema.is_complex_object() && value.as_ref().is_some() {
                        let inner_schema_fields = &field_schema.fields.clone().unwrap();
                        let inner_data_rows = &Some(vec![value.unwrap().clone()]);

                        let positions = place_bq_table_rows(
                            rows,
                            inner_schema_fields,
                            inner_data_rows,
                            array_row_index + array_row_increment,
                            array_col_index + array_col_increment,
                            false,
                            0,
                        );

                        array_col_increment += positions.1;
                    } else {
                        if field_schema.is_array() && !field_schema.is_complex_object() {
                            if value.is_some() {
                                let inner_data_rows =
                                    &Some(value.unwrap().as_array().unwrap().to_owned());

                                if let Some(inner_data_rows) = inner_data_rows {
                                    // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                                    //     "field_schema: {:?}, is_array {}, is_complex_object: {}, inner_data_rows: {:?}",
                                    //     field_schema,
                                    //     field_schema.is_array(),
                                    //     field_schema.is_complex_object(),
                                    //     inner_data_rows
                                    // )));

                                    let mut row_index = 0;
                                    for row in inner_data_rows {
                                        let value = row.pointer("/v").unwrap().clone();

                                        rows[array_row_index + array_row_increment + row_index]
                                            .cells[array_col_index + array_col_increment] =
                                            TableValue::Index(row_index + 1);

                                        rows[array_row_index + array_row_increment + row_index]
                                            .cells[array_col_index + array_col_increment + 1] =
                                            from_schema_value(field_schema, &Some(value));

                                        row_index += 1;
                                    }

                                    array_max_inner_row_increment =
                                        match array_max_inner_row_increment > row_index {
                                            true => array_max_inner_row_increment,
                                            false => row_index,
                                        };
                                }
                            }

                            //move the col index further
                            array_col_increment += 2;
                        } else {
                            if value.as_ref().is_some()
                                && value.as_ref().unwrap().is_array()
                                && value.as_ref().unwrap().as_array().unwrap().len() == 0
                            {
                                array_col_increment += 2;
                            } else {
                                rows[array_row_index + array_row_increment].cells
                                    [array_col_index + array_col_increment] =
                                    from_schema_value(field_schema, &value);
                                array_col_increment += 1;
                            }
                        }
                    }
                }
            }

            //
            if array_max_inner_row_increment > 0 {
                array_row_increment += array_max_inner_row_increment;
            } else {
                array_row_increment += 1;
            }
        }
    }

    (array_row_increment, array_col_increment)
}

// fn calculate_number_rows(data_rows: &Option<Vec<serde_json::Value>>) -> usize {
//     let mut count: usize = 0;

//     if data_rows.is_some() {
//         let data_rows = data_rows.as_ref().unwrap();

//         for data_row in data_rows {
//             let mut col_index: usize = 0;
//             let mut increment = 1;
//             let mut value = data_row.pointer(&format!("/f/{}/v", col_index));
//             while value.is_some() {
//                 if value.unwrap().is_array() {
//                     let inner_data_rows = &value
//                         .unwrap()
//                         .as_array()
//                         .unwrap()
//                         .iter()
//                         .map(|i| i.pointer("/v").unwrap().clone())
//                         .collect::<Vec<serde_json::Value>>();

//                     let x = calculate_number_rows(&Some(inner_data_rows.to_owned()));
//                     increment = match increment >= x {
//                         true => increment,
//                         false => x,
//                     };
//                 }

//                 col_index += 1;
//                 value = data_row.pointer(&format!("/f/{}/v", col_index));
//             }

//             count += increment;
//         }
//     }

//     count
// }

fn timestamp_to_value(bq_timestamp: &Option<serde_json::Value>) -> Option<String> {
    if let Some(bq_timestamp) = bq_timestamp {
        if !bq_timestamp.is_null() {
            let timestamp: f64 = bq_timestamp
                .as_str()
                .unwrap_or_default()
                .parse()
                .expect("timestamp not valid");

            let js_date = js_sys::Date::new(&JsValue::from(timestamp * 1000.0));
            let str = js_date.to_iso_string().as_string().unwrap();

            return Some(str);
        }
    }

    None
}

impl Job {
    pub(crate) fn to_error_table(&self) -> TableBuilder {
        let columns = [
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

        // DataTable::new("e1", &Some(header), &Some(rows))

        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns: columns,
            rows: rows,
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
        let columns = [
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

        TableBuilder {
            style: TableStyle::default(),
            dynamic_table_render: false,
            columns,
            rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_bindgen_test::*;
    use website_component_table::{TableRow, TableValue};

    use super::timestamp_to_value;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    pub fn get_bq_table_header_test_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());

        assert_eq!(header.len(), 62);
        assert_eq!(header[0], "#");

        let field_names = [
            "Pim_Value",
            "AttributeValueCategory",
            "Colour_PDP",
            "Width_accessoires",
            "Height_accessoires",
            "Lining",
            "Shop_by_Sport",
            "Not_searchable",
            "Exclusive_Access",
            "Promo_Activity",
            "Additional_info",
            "Lifestyle",
            "Ranking",
            "Product_GBPC",
            "TradebyteActive_Combi",
            "MainColorPDP",
            "Sleeve_Length",
            "Padding",
            "Soldout",
            "Neck_Line",
            "Key_Looks",
            "pimExportDate",
            "ProductGroupCategory",
            "Heel_Height",
            "USP_flag",
            "Material_2",
            "Combi_number",
            "Flavour_Copy",
            "Delete_Flag",
            "New_Arrivals",
            "Combi_Reference",
            "RISE",
            "CTP_date",
            "STYLE",
            "Proper_style_name",
            "StyleLength",
            "Actual_Online_Date",
            "Structure_assignments",
            "Functionality",
            "Promo_Flag",
            "Fit_for_bottoms",
            "Sustainable",
            "Material_3",
            "Fit_for_tops",
            "Material",
            "Program",
            "ImageCount",
            "Collection",
            "Occasion",
            "Shop_by_Activity",
            "Brand",
            "Length_accessoires",
            "Backfill_AboutYou",
            "DETAIL",
            "row_number",
        ];

        for i in 0..28 {
            let name = &field_names[i];
            assert_eq!(&header[i + 1], name)
        }

        assert_eq!(&header[29], &"Delete_Flag.#");
        assert_eq!(&header[30], &"Delete_Flag.value");
        assert_eq!(&header[31], &"Delete_Flag.level");

        for i in 29..37 {
            let name = field_names[i];
            assert_eq!(&header[i + 3], name);
        }

        assert_eq!(&header[40], "Structure_assignments.#");
        assert_eq!(&header[41], "Structure_assignments.assignment");
        assert_eq!(&header[42], "Structure_assignments.structure_system");

        assert_eq!(&header[43], "Functionality");
        assert_eq!(&header[44], "Promo_Flag.#");
        assert_eq!(&header[45], "Promo_Flag.value");
        assert_eq!(&header[46], "Promo_Flag.country");

        for i in 40..55 {
            let name = field_names[i];
            assert_eq!(&header[i + 7], name);
        }

        assert_eq!(&header[61], "row_number");
    }

    #[test]
    pub fn get_bq_table_header_test_2() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test2.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());

        assert_eq!(header.len(), 9);
        assert_eq!(header[0], "#");
        assert_eq!(&header[1], &"Pim_Value");
        assert_eq!(&header[2], &"AttributeValueCategory");
        assert_eq!(&header[3], &"Colour_PDP");
        assert_eq!(&header[4], &"Width_accessoires");
        assert_eq!(&header[5], &"Height_accessoires");
        assert_eq!(&header[6], &"Delete_Flag.#");
        assert_eq!(&header[7], &"Delete_Flag.value");
        assert_eq!(&header[8], &"Delete_Flag.level");
    }

    #[test]
    pub fn get_bq_table_header_test_3() {
        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());

        assert_eq!(header.len(), 4);
        assert_eq!(header[0], "#");
        assert_eq!(&header[1], &"attributes.row_number");
        assert_eq!(&header[2], &"attributes.data_type");
        assert_eq!(&header[3], &"data");
    }

    #[test]
    fn place_bq_table_rows_test_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        // let header =
        //     &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        // let number_columns = header.len();
        // let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<TableRow> = vec![];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,
            0,
            true,
            0,
        );

        assert_eq!(rows.len(), 1796);
        assert_eq!(rows[0].cells.len(), 62);

        let v = rows[0].cells[0].clone();
        assert!(match v {
            TableValue::Index(v) => v == 1,
            _ => false,
        });
        let v = rows[0].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "",
            _ => false,
        });
        let v = rows[0].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "SKIRTS",
            _ => false,
        });

        let v = rows[0].cells[8].clone();
        assert!(match v {
            TableValue::Null => true,
            _ => false,
        });

        let v = rows[0].cells[27].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "J20J215714BEH",
            _ => false,
        });

        let v = rows[0].cells[28].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "• stretch jersey and mesh<CRLF>• double layered design<CRLF>• pleated waist<CRLF>• midi length<CRLF>• pulls on<CRLF>• Calvin Klein broad logo elastic waistband<CRLF><CRLF>Our model is 1.80m (5ft 11in) and is wearing a size S.<CRLF><CRLF>84% polyester 16% elastane <CRLF>delicate machine wash<CRLF>do not tumble dry<CRLF>fits true to size",
            _ => false,
        });

        let v = rows[0].cells[29].clone();
        assert!(match v {
            TableValue::Index(i) => i == 1,
            _ => false,
        });

        let v = rows[0].cells[30].clone();
        assert!(match v {
            TableValue::Null => true,
            _ => false,
        });

        let v = rows[0].cells[31].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "Style",
            _ => false,
        });

        let v = rows[0].cells[32].clone();
        assert!(match v {
            TableValue::Boolean(b) => b == false,
            _ => false,
        });

        let v = rows[1].cells[29].clone();
        assert!(match v {
            TableValue::Index(i) => i == 2,
            _ => false,
        });

        let v = rows[1].cells[30].clone();
        assert!(match v {
            TableValue::Null => true,
            _ => false,
        });

        let v = rows[1].cells[31].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "Combi",
            _ => false,
        });

        let v = rows[0].cells[61].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "452216",
            _ => false,
        });

        let v = rows[1].cells[0].clone();
        assert!(match v {
            TableValue::Null => true,
            _ => false,
        });

        let v = rows[27].cells[0].clone().clone();
        assert!(match v {
            TableValue::Index(i) => i == 2,
            _ => false,
        });

        let v = rows[1762].cells[0].clone().clone();
        assert!(match v {
            TableValue::Index(i) => i == 50,
            _ => false,
        });
    }

    #[test]
    fn place_bq_table_rows_test_2() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test2.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        // let header =
        //     &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        // let number_columns = header.len();
        // let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<TableRow> = vec![];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,
            0,
            true,
            0,
        );

        assert_eq!(rows.len(), 16);
        assert_eq!(rows[0].cells.len(), 9);

        let v = rows[0].cells[0].clone();
        assert!(match v {
            TableValue::Index(v) => v == 1,
            _ => false,
        });
        let v = rows[0].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "",
            _ => false,
        });
        let v = rows[0].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "TRAINERS",
            _ => false,
        });
        let v = rows[0].cells[3].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "BlackMono",
            _ => false,
        });
    }

    #[test]
    fn place_bq_table_rows_test() {
        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        // let header =
        //     &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        // let number_columns = header.len();
        // let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<TableRow> = vec![];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,
            0,
            true,
            0,
        );

        assert_eq!(rows.len(), 50);
        assert_eq!(rows[0].cells.len(), 4);

        let v = rows[0].cells[0].clone();
        assert!(match v {
            TableValue::Index(v) => v == 1,
            _ => false,
        });

        let v = rows[0].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "163517",
            _ => false,
        });
        let v = rows[0].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "dsdfdsd",
            _ => false,
        });
        let v = rows[0].cells[3].clone();
        assert!(match v {
            TableValue::String(s) => s.len() > 100,
            _ => false,
        });

        let v = rows[1].cells[0].clone().clone();
        assert!(match v {
            TableValue::Index(v) => v == 2,
            _ => false,
        });
        let v = rows[1].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "163518",
            _ => false,
        });
        let v = rows[1].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "dsdfdsd",
            _ => false,
        });
        let v = rows[1].cells[3].clone();
        assert!(match v {
            TableValue::String(s) => s.len() > 100,
            _ => false,
        });

        let v = rows[49].cells[0].clone();
        assert!(match v {
            TableValue::Index(v) => v == 50,
            _ => false,
        });
        let v = rows[49].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "163566",
            _ => false,
        });
        let v = rows[49].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "dsdfdsd",
            _ => false,
        });
        let v = rows[49].cells[3].clone();
        assert!(match v {
            TableValue::String(s) => s.len() > 100,
            _ => false,
        });
    }

    #[test]
    fn place_bq_all_types_test_1() {
        let complex_object_array_test = include_str!("test_resources/all_types_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        // let header =
        //     &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        // let number_columns = header.len();
        // let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<TableRow> = vec![];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,
            0,
            true,
            0,
        );

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].cells.len(), 21);

        let v = rows[0].cells[0].clone();
        assert!(match v {
            TableValue::Index(v) => v == 1,
            _ => false,
        });

        let v = rows[0].cells[1].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "1",
            _ => false,
        });

        let v = rows[0].cells[2].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "1",
            _ => false,
        });

        let v = rows[0].cells[3].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "1-6 15 0:0:0",
            _ => false,
        });
        let v = rows[0].cells[4].clone();
        assert!(match v {
            TableValue::Null => true,
            _ => false,
        });

        let v = rows[0].cells[5].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "",
            _ => false,
        });

        let v = rows[0].cells[6].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "12345",
            _ => false,
        });

        let v = rows[0].cells[7].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "123.45",
            _ => false,
        });

        let v = rows[0].cells[8].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "1",
            _ => false,
        });

        let v = rows[0].cells[9].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "{\"coordinates\":[10,20],\"id\":1}",
            _ => false,
        });

        let v = rows[0].cells[10].clone();
        assert!(match v {
            TableValue::Boolean(s) => s == false,
            _ => false,
        });

        let v = rows[0].cells[11].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "POINT(-50 90)",
            _ => false,
        });

        let v = rows[0].cells[12].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "fzury",
            _ => false,
        });
        let v = rows[0].cells[13].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "LINESTRING(1 2, 3 4)",
            _ => false,
        });

        let v = rows[0].cells[14].clone();
        assert!(match v {
            TableValue::String(s) =>
                s.clone() == "POLYGON((-125 48, -124 46, -117 46, -117 49, -125 48))",
            _ => false,
        });

        let v = rows[0].cells[15].clone();
        assert!(match v {
            TableValue::String(s) => s.clone() == "1.703357814940265E9",
            _ => false,
        });
    }

    #[test]
    fn get_rows_test_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();
        let number_columns = 62;

        let rows = complex_object_array_test.get_rows(number_columns, 0);

        assert_eq!(rows.1.len(), 1796);
    }

    #[test]
    fn get_simple_array_test_1() {
        let complex_object_array_test = include_str!("test_resources/simple_array.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();
        let number_columns = 4;

        let rows = complex_object_array_test.get_rows(number_columns, 0);

        assert_eq!(rows.1.len(), 150);

        let rows1 = &rows.1;
        let row1 = &rows1.first().expect("must have row 1").cells;
        assert_eq!(row1.len(), 4);

        let rc = &row1[0];
        assert!(match rc {
            TableValue::Index(s) => s.clone() == 1,
            _ => false,
        });

        let rc = &row1[1];
        assert!(match rc {
            TableValue::String(s) => s.clone() == "a",
            _ => false,
        });

        let rc = &row1[2];
        assert!(match rc {
            TableValue::Index(s) => s.clone() == 1,
            _ => false,
        });
    }

    #[test]
    fn get_simple_array_test_4() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test4.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header = complex_object_array_test.get_header();
        // println!("{:?}", header);
        let number_columns = header.len();

        let rows = complex_object_array_test.get_rows(number_columns, 0);

        assert_eq!(rows.1.len(), 3502);

        let rows1 = &rows.1;
        let row1: &TableRow = &rows1[0];
        //assert_eq!(row1.len(), 4);

        let c_index: TableValue = row1.cells[0].clone();
        assert!(match c_index {
            TableValue::Index(s) => s.clone() == 1,
            _ => false,
        });

        //"selling_channels.#"
        let rc = row1.cells[63].clone();
        println!("header: {:?}", header[63]);
        println!("rc: {:?}", rc);
        assert!(match rc {
            TableValue::Null => true,
            _ => false,
        });
    }

    #[wasm_bindgen_test]
    fn timestamp_to_value_test_1() {
        let value: serde_json::Value =
            serde_json::Value::String(String::from("1.703357814940265E9"));
        let result = timestamp_to_value(&Some(value));

        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str(), "2023-12-23T18:56:54.940Z");
    }
}
