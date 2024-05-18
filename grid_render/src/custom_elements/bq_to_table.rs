use std::ops::Deref;

use wasm_bindgen::JsValue;

use super::{
    bq_query_custom_element::BigqueryQueryCustomElement,
    bq_table_custom_element::BigqueryTableCustomElement,
    data_table_element::{DataTable, DataTableItem},
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
    pub(crate) fn to_bq_table(
        &self,
        bq_table_requested: &BigqueryTableCustomElement,
    ) -> BigqueryTableCustomElement {
        let header = self.get_header();
        let number_columns = header.len();
        let page_start_index = bq_table_requested.get_page_start_index();

        let (rows_in_page, rows) = self.get_rows(number_columns, page_start_index);
        let rows_total = self.get_rows_total();

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "rows_total: {}, rows_in_page: {}, rows len: {}",
            rows_total,
            rows_in_page,
            rows.len()
        )));

        bq_table_requested.with_table_info(
            Some(rows_in_page),
            Some(rows_total),
            Some(header),
            Some(rows),
        )
    }

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
            "rows_total: {}, rows_in_page: {}, rows len: {}",
            rows_total,
            rows_in_page,
            rows.len()
        )));

        bq_query_requested.with_table_info(
            Some(rows_in_page),
            Some(rows_total),
            Some(header),
            Some(rows),
        )
    }

    fn get_header(&self) -> Vec<String> {
        assert!(self.schema.is_some(), "unexpected empty schema");

        let schema = self.schema.as_ref().unwrap();
        get_bq_table_header(&schema)
    }

    fn get_rows(
        &self,
        number_columns: usize,
        page_start_index: usize,
    ) -> (usize, Vec<Vec<Option<DataTableItem>>>) {
        assert!(self.schema.is_some(), "unexpected empty schema");
        let schema = self.schema.as_ref().unwrap();

        let rows_in_page = if self.rows.is_some() {
            self.rows.as_ref().unwrap().len()
        } else {
            0
        };

        let number_rows = calculate_number_rows(&self.rows);

        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

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

    fn get_rows_total(&self) -> usize {
        parse_to_usize(Some(self.total_rows.to_string())).unwrap_or(0)
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

        bq_table_element.with_table_info(
            Some(rows_in_page),
            Some(rows_total),
            Some(header),
            Some(rows),
        )
    }

    fn get_rows(
        &self,
        number_columns: usize,
        page_start_index: usize,
        response_rows: &Option<TableDataListResponse>,
    ) -> (usize, Vec<Vec<Option<DataTableItem>>>) {
        assert!(self.schema.is_some(), "unexpected empty schema");
        let schema = self.schema.as_ref().unwrap();

        let origin_rows = &response_rows.as_ref().expect("rows not found").rows;

        let rows_in_page = origin_rows.len();

        let number_rows = calculate_number_rows(&Some(origin_rows.to_owned()));
        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

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

impl DataTableItem {
    pub fn from_schema_value(
        field_schema: &TableFieldSchema,
        value: &Option<serde_json::Value>,
    ) -> DataTableItem {
        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     "value: {:?}",
        //     value
        // )));

        match field_schema.r#type.as_str() {
            "TIMESTAMP" => DataTableItem::from_value(&timestamp_to_value(value)),
            _ => DataTableItem::from_value(value),
        }
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
    rows: &mut Vec<Vec<Option<DataTableItem>>>,
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
                rows[array_row_index + array_row_increment]
                    [array_col_index + array_col_increment] = Some(DataTableItem::new_main_index(
                    data_row_index + 1 + page_start_index,
                ));
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

                if field_schema.is_array()
                    && field_schema.is_complex_object()
                    && value.as_ref().is_some()
                    && value.as_ref().unwrap().is_array()
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
                        rows[array_row_index + array_row_increment]
                            [array_col_index + array_col_increment] =
                            Some(DataTableItem::from_schema_value(field_schema, &value));
                        array_col_increment += 1;
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

fn calculate_number_rows(data_rows: &Option<Vec<serde_json::Value>>) -> usize {
    let mut count: usize = 0;

    if data_rows.is_some() {
        let data_rows = data_rows.as_ref().unwrap();

        for data_row in data_rows {
            let mut col_index: usize = 0;
            let mut increment = 1;
            let mut value = data_row.pointer(&format!("/f/{}/v", col_index));
            while value.is_some() {
                if value.unwrap().is_array() {
                    let inner_data_rows = &value
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|i| i.pointer("/v").unwrap().clone())
                        .collect::<Vec<serde_json::Value>>();

                    let x = calculate_number_rows(&Some(inner_data_rows.to_owned()));
                    increment = match increment >= x {
                        true => increment,
                        false => x,
                    };
                }

                col_index += 1;
                value = data_row.pointer(&format!("/f/{}/v", col_index));
            }

            count += increment;
        }
    }

    count
}

fn timestamp_to_value(bq_timestamp: &Option<serde_json::Value>) -> Option<serde_json::Value> {
    if let Some(bq_timestamp) = bq_timestamp {
        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        //     "bq_timestamp: {:?}",
        //     bq_timestamp
        // )));

        let timestamp: f64 = bq_timestamp
            .as_str()
            .unwrap_or_default()
            .parse()
            .expect("timestamp not valid");

        let js_date = js_sys::Date::new(&JsValue::from(timestamp * 1000.0));
        let str = js_date.to_iso_string().as_string().unwrap();

        return Some(serde_json::to_value(str).unwrap());
    }

    None
}

impl Job {
    pub(crate) fn to_error_table(&self) -> DataTable {
        let header = ["message".to_string(), "reason".to_string()].to_vec();

        // let rows_default = [[
        //     Some(DataTableItem::from_string(&"--".to_string())),
        //     Some(DataTableItem::from_string(&"--".to_string())),
        // ]
        // .to_vec()]
        // .to_vec();

        let rows: Vec<Vec<Option<DataTableItem>>> = match self.status.as_ref() {
            Some(status) => match &status.error_result {
                Some(error_result) => [[
                    Some(DataTableItem::from_string(
                        error_result.message.as_ref().unwrap_or(&"".to_string()),
                    )),
                    Some(DataTableItem::from_string(
                        error_result.reason.as_ref().unwrap_or(&"".to_string()),
                    )),
                ]
                .to_vec()]
                .to_vec(),
                None => Self::get_errors_rows_default(),
            },
            None => Self::get_errors_rows_default(),
        };

        DataTable::new("e1", &Some(header), &Some(rows))
    }

    fn get_errors_rows_default() -> Vec<Vec<Option<DataTableItem>>> {
        [[
            Some(DataTableItem::from_string(&"--".to_string())),
            Some(DataTableItem::from_string(&"--".to_string())),
        ]
        .to_vec()]
        .to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::custom_elements::{
        base_element_trait::BaseElementTrait, bq_table_custom_element::BigqueryTableCustomElement,
        data_table_element::DataTableItem,
    };
    use wasm_bindgen_test::*;

    use super::timestamp_to_value;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    pub fn calculate_number_rows_test_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        assert_eq!(number_rows, 1796);
    }

    #[test]
    pub fn calculate_number_rows_test_2() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test2.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        assert_eq!(number_rows, 16);
    }

    #[test]
    pub fn calculate_number_rows_test_3() {
        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        assert_eq!(number_rows, 50);
    }

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

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        let number_columns = header.len();
        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,
            0,
            true,
            0,
        );

        // println!("row 0: \n{:?}", rows[0]);
        // println!("row 28: \n{:?}", rows[28]);
        assert_eq!(rows.len(), 1796);
        assert_eq!(rows[0].len(), 62);

        let v = rows[0][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "1");
        let v = rows[0][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "");
        let v = rows[0][2].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "SKIRTS");

        let v = rows[0][8].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(v.is_null);
        assert!(v.value.is_none());

        let v = rows[0][27].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "J20J215714BEH");

        let v = rows[0][28].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "• stretch jersey and mesh<CRLF>• double layered design<CRLF>• pleated waist<CRLF>• midi length<CRLF>• pulls on<CRLF>• Calvin Klein broad logo elastic waistband<CRLF><CRLF>Our model is 1.80m (5ft 11in) and is wearing a size S.<CRLF><CRLF>84% polyester 16% elastane <CRLF>delicate machine wash<CRLF>do not tumble dry<CRLF>fits true to size");

        let v = rows[0][29].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "1");

        let v = rows[0][30].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(v.is_null);
        assert!(v.value.is_none());

        let v = rows[0][31].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "Style");

        let v = rows[0][32].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "false");

        let v = rows[1][29].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "2");

        let v = rows[1][30].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "true");

        let v = rows[1][31].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "Combi");

        let v = rows[0][61].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "452216");

        let v = rows[1][0].clone();
        assert!(v.is_none());

        let v = rows[27][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "2");

        let v = rows[1762][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "50");
    }

    #[test]
    fn place_bq_table_rows_test_2() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test2.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        let number_columns = header.len();
        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

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
        assert_eq!(rows[0].len(), 9);

        let v = rows[0][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "1");
        let v = rows[0][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "");
        let v = rows[0][2].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "TRAINERS");
        let v = rows[0][3].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "BlackMono");
    }

    #[test]
    fn place_bq_table_rows_test_3() {
        let complex_object_array_test = include_str!("test_resources/struct_json_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        let number_columns = header.len();
        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

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
        assert_eq!(rows[0].len(), 4);

        let v = rows[0][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "1");
        let v = rows[0][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "163517");
        let v = rows[0][2].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "dsdfdsd");
        let v = rows[0][3].clone();
        assert!(v.is_some());
        let v = &v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert!(v.value.as_ref().unwrap().len() > 100);

        let v = rows[1][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "2");
        let v = rows[1][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "163518");
        let v = rows[1][2].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "dsdfdsd");
        let v = rows[1][3].clone();
        assert!(v.is_some());
        let v = &v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert!(v.value.as_ref().unwrap().len() > 100);

        let v = rows[49][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "50");
        let v = rows[49][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "163566");
        let v = rows[49][2].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "dsdfdsd");
        let v = rows[49][3].clone();
        assert!(v.is_some());
        let v = &v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert!(v.value.as_ref().unwrap().len() > 100);
    }

    #[test]
    fn place_bq_table_rows_test_4() {
        let complex_object_array_test = include_str!("test_resources/all_types_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        let number_columns = header.len();
        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<Vec<Option<DataTableItem>>> =
            vec![vec![None; number_columns]; number_rows];

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
        assert_eq!(rows[0].len(), 21);

        let v = rows[0][0].clone().unwrap();
        assert!(v.is_index);
        assert_eq!(v.value.unwrap(), "1");

        let v = rows[0][1].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "1");

        let v = rows[0][2].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "1");

        let v = rows[0][3].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "1-6 15 0:0:0");

        let v = rows[0][4].clone();
        assert!(v.is_some());
        assert!(v.unwrap().is_null);

        let v = rows[0][5].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "");

        let v = rows[0][6].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "12345");

        let v = rows[0][7].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "123.45");

        let v = rows[0][8].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "1");

        let v = rows[0][9].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "{\"coordinates\":[10,20],\"id\":1}");

        let v = rows[0][10].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "false");

        let v = rows[0][11].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "POINT(-50 90)");

        let v = rows[0][12].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "fzury");

        let v = rows[0][13].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(v.value.unwrap(), "LINESTRING(1 2, 3 4)");

        let v = rows[0][14].clone();
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(!v.is_index);
        assert!(!v.is_null);
        assert_eq!(
            v.value.unwrap(),
            "POLYGON((-125 48, -124 46, -117 46, -117 49, -125 48))"
        );

        let v = rows[0][15].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "1.703357814940265E9");

        // let v = rows[1][0].clone();
        // assert!(v.is_none());

        // let v = rows[27][0].clone().unwrap();
        // assert!(v.is_index);
        // assert_eq!(v.value.unwrap(), "2");

        // let v = rows[1762][0].clone().unwrap();
        // assert!(v.is_index);
        // assert_eq!(v.value.unwrap(), "50");
    }

    #[wasm_bindgen_test]
    fn to_bq_table_test_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let bq_table = &BigqueryTableCustomElement::base_new(
            "element_id".to_string(),
            "jobId".to_string(),
            "projectId".to_string(),
            "location".to_string(),
            "token".to_string(),
        );

        let bq_table_2 = complex_object_array_test.to_bq_table(bq_table);
        let parent_node = &crate::createElement("div");
        bq_table_2.to_data_table("element_id").render(parent_node);

        let inner_html = &parent_node.inner_html();
        assert!(inner_html.contains("Included_BE+ Sales Catalog"));
        assert!(inner_html.contains("Pim_Value"));
        assert!(inner_html.contains("J20J215714BEH"));
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

    #[wasm_bindgen_test]
    fn timestamp_to_value_test_1() {
        let value: serde_json::Value =
            serde_json::Value::String(String::from("1.703357814940265E9"));
        let result = timestamp_to_value(&Some(value));

        assert!(result.is_some());
        assert_eq!(
            result.unwrap().as_str().unwrap_or_default(),
            "2023-12-23T18:56:54.940Z"
        );
    }
}
