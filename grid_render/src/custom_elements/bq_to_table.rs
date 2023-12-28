use super::{
    data_table_controls_element::DataTableControls,
    data_table_element::{DataTable, DataTableItem},
    data_table_shadow_element::DataTableShadow,
};
use crate::{
    bigquery::jobs::{GetQueryResultsResponse, TableFieldSchema},
    parse_to_usize,
};

impl GetQueryResultsResponse {
    pub fn plot_table(&self, element: &web_sys::Element) {
        if self.schema.is_some() {
            let schema = self.schema.as_ref().unwrap();

            let header = &get_bq_table_header(&schema);
            let number_columns = header.len();
            let number_rows = calculate_number_rows(&self.rows);
            let start_index = 0;

            let mut rows: Vec<Vec<Option<DataTableItem>>> =
                vec![vec![None; number_columns]; number_rows];

            place_bq_table_rows(&mut rows, &schema.fields, &self.rows, 0, 0, true);

            let number_of_rows_total = parse_to_usize(Some(self.total_rows.clone())).unwrap();

            let shadow_root = &DataTableShadow::init_shadow(element);

            DataTableControls::render_control(
                shadow_root,
                number_rows,
                number_of_rows_total,
                start_index,
            );

            DataTable::render_table(shadow_root, header, &rows);
            // console::log_1(&JsValue::from_str(&"4 - zzzzzzz"));
        }
    }
}

fn get_bq_table_header(schema: &crate::bigquery::jobs::TableSchema) -> Vec<String> {
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
    schema_fields: &Vec<crate::bigquery::jobs::TableFieldSchema>,
    data_rows: &Vec<serde_json::Value>,
    array_row_index: usize,
    array_col_index: usize,
    include_index_column: bool,
) -> (usize, usize) {
    // 2 sets of variables are in use.
    // "data_..." to control the position of the data
    // "array_.." to control the position and increments of the TableItem array

    // control the vertical position of the array
    let mut array_row_increment = 0;

    // variable to move horizontally the placement in the TableItem array
    let mut array_col_increment = 0;

    //
    for data_row_index in 0..data_rows.len() {
        let data_row = &data_rows[data_row_index];

        //when the data row has inner arrays, the max size of the inner array(s) is controlled here
        let mut array_max_inner_row_increment = 0;

        //reset the variable of horizontal movement in a new data row
        array_col_increment = 0;

        //index column
        if include_index_column {
            rows[array_row_index + array_row_increment][array_col_index + array_col_increment] =
                Some(DataTableItem::new_main_index(data_row_index + 1));
            array_col_increment += 1;
        }

        // go through the schema of the data
        for col_index in 0..schema_fields.len() {
            let field = &schema_fields[col_index];
            let value = data_row.pointer(&format!("/f/{}/v", col_index));

            if field.is_array()
                && field.is_complex_object()
                && value.is_some()
                && value.unwrap().is_array()
            {
                let inner_schema_fields = &field.fields.clone().unwrap();
                let inner_data_rows = &value
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|i| i.pointer("/v").unwrap().clone())
                    .collect::<Vec<serde_json::Value>>();

                let positions = place_bq_table_rows(
                    rows,
                    inner_schema_fields,
                    inner_data_rows,
                    array_row_index + array_row_increment,
                    array_col_index + array_col_increment,
                    true,
                );
                //establish the max rows to progress
                array_max_inner_row_increment = match array_max_inner_row_increment > positions.0 {
                    true => array_max_inner_row_increment,
                    false => positions.0,
                };
                //move the col index further
                array_col_increment += positions.1;
            } else {
                if field.is_complex_object() && value.is_some() {
                    let inner_schema_fields = &field.fields.clone().unwrap();
                    let inner_data_rows = &vec![value.unwrap().clone()];

                    let positions = place_bq_table_rows(
                        rows,
                        inner_schema_fields,
                        inner_data_rows,
                        array_row_index + array_row_increment,
                        array_col_index + array_col_increment,
                        false,
                    );

                    array_col_increment += positions.1;
                } else {
                    rows[array_row_index + array_row_increment]
                        [array_col_index + array_col_increment] =
                        Some(DataTableItem::from_value(&value));
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

    (array_row_increment, array_col_increment)
}

fn calculate_number_rows(data_rows: &Vec<serde_json::Value>) -> usize {
    let mut count: usize = 0;

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

                let x = calculate_number_rows(inner_data_rows);
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

    count
}

#[cfg(test)]
mod tests {
    use crate::custom_elements::data_table_element::DataTableItem;
    use wasm_bindgen_test::*;

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

    #[wasm_bindgen_test]
    fn plot_table_1() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let element = &crate::createElement("div");

        complex_object_array_test.plot_table(element);

        assert!(element.shadow_root().is_some());
        assert!(element.shadow_root().unwrap().has_child_nodes());

        let table = element.shadow_root().unwrap().last_child();
        assert!(table.is_some());
        let table = table.unwrap();
        assert_eq!(table.node_name(), "TABLE");
    }

    #[wasm_bindgen_test]
    fn plot_table_twice() {
        let complex_object_array_test =
            include_str!("test_resources/complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let element = &crate::createElement("div");

        complex_object_array_test.plot_table(element);

        assert!(element.shadow_root().is_some());
        assert!(element.shadow_root().unwrap().has_child_nodes());
        assert_eq!(element.shadow_root().unwrap().child_element_count(), 4);

        let table = element.shadow_root().unwrap().last_child();
        assert!(table.is_some());
        let table = table.unwrap();
        assert_eq!(table.node_name(), "TABLE");

        //second plot
        complex_object_array_test.plot_table(element);

        assert!(element.shadow_root().is_some());
        assert!(element.shadow_root().unwrap().has_child_nodes());
        assert_eq!(element.shadow_root().unwrap().child_element_count(), 4);

        let table = element.shadow_root().unwrap().last_child();
        assert!(table.is_some());
        let table = table.unwrap();
        assert_eq!(table.node_name(), "TABLE");
    }
}
