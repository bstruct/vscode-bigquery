use crate::bigquery::jobs::TableFieldSchema;

use super::table_plot::{render_table, TableItem};

impl crate::bigquery::jobs::GetQueryResultsResponse {
    pub fn plot_table(&self, element: &web_sys::HtmlElement) {
        if self.schema.is_some() {
            let schema = self.schema.as_ref().unwrap();

            let header = &get_bq_table_header(&schema);
            let number_columns = header.len();
            let number_rows = calculate_number_rows(&self.rows);
            let mut rows: Vec<Vec<Option<TableItem>>> =
                vec![vec![None; number_columns]; number_rows];

            place_bq_table_rows(&mut rows, &schema.fields, &self.rows,0,0);

            render_table(element, header, &rows);
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
    rows: &mut Vec<Vec<Option<TableItem>>>,
    schema_fields: &Vec<crate::bigquery::jobs::TableFieldSchema>,
    data_rows: &Vec<serde_json::Value>,
    start_row_index: usize,
    start_col_index: usize,
) {
    let mut data_row_index = start_row_index;
    for data_row in data_rows {

        rows[data_row_index][start_col_index] = Some(TableItem::new_main_index(data_row_index + 1));

        for col_index in 0..schema_fields.len() {
            let field = &schema_fields[col_index];
            let value = data_row.pointer(&format!("/f/{}/v", col_index));

            if field.is_array()
                && field.is_complex_object()
                && value.is_some()
                && value.unwrap().is_array()
            {

                let inner_schema_fields = &field.fields.clone().unwrap();
                let inner_data_rows = value.unwrap().as_array().unwrap();

                place_bq_table_rows(rows, inner_schema_fields, inner_data_rows, data_row_index,  col_index);
                // let value_array = value.unwrap().as_array().unwrap();
                // let inner_rows = get_bq_table_rows(&field.fields.as_ref().unwrap(), value_array);
                // consolidate_rows(&mut output_rows, &mut output_row, &inner_rows);
            } else {
                rows[data_row_index][col_index + 1] = Some(TableItem::from_value(&value));
            }
        }

        data_row_index += 1;
    }
}

fn calculate_number_rows(data_rows: &Vec<serde_json::Value>) -> usize {
    let mut count: usize = 0;

    for data_row in data_rows {
        let mut col_index: usize = 0;
        let mut increment = 1;
        let mut value = data_row.pointer(&format!("/f/{}/v", col_index));
        while value.is_some() {
            if value.unwrap().is_array() {
                let x = calculate_number_rows(&value.unwrap().as_array().unwrap());
                increment = match increment >= x - 1 {
                    true => increment,
                    false => x - 1,
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
    use crate::custom_elements::table_plot::TableItem;

    #[test]
    pub fn calculate_number_rows_test_1() {
        let complex_object_array_test = include_str!("complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        assert_eq!(number_rows, 1746);
    }

    #[test]
    pub fn get_bq_table_header_test_1() {
        let complex_object_array_test = include_str!("complex_object_array_test.json");
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
    fn place_bq_table_rows_test_1() {
        let complex_object_array_test = include_str!("complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header =
            &super::get_bq_table_header(&complex_object_array_test.schema.as_ref().unwrap());
        let number_columns = header.len();
        let number_rows = super::calculate_number_rows(&complex_object_array_test.rows);
        let mut rows: Vec<Vec<Option<TableItem>>> = vec![vec![None; number_columns]; number_rows];

        super::place_bq_table_rows(
            &mut rows,
            &complex_object_array_test.schema.as_ref().unwrap().fields,
            &complex_object_array_test.rows,
            0,0
        );

        assert_eq!(rows.len(), 1746);
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

        let v = rows[0][61].clone();
        assert!(v.is_some());
        assert_eq!(v.unwrap().value.unwrap(), "452216");
    }
}
