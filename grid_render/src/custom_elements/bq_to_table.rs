use crate::bigquery::jobs::TableFieldSchema;

use super::table_plot::{render_table, TableItem};

impl crate::bigquery::jobs::GetQueryResultsResponse {
    pub fn plot_table(&self, element: &web_sys::HtmlElement) {
        let header = &get_bq_table_header(&self.schema);
        let rows = &get_bq_table_rows(&self.schema, &self.rows);
        // let total_rows = parse_to_usize(Some(self.total_rows));

        render_table(element, header, rows);
    }
}

fn get_bq_table_header(schema: &Option<crate::bigquery::jobs::TableSchema>) -> Vec<String> {
    let mut header_columns = Vec::new();
    header_columns.push(String::from("#"));

    if let Some(table_schema) = &schema {
        append_bq_table_header(&mut header_columns, &table_schema.fields, &None);
    }

    header_columns
}

fn append_bq_table_header(
    header_columns: &mut Vec<String>,
    fields: &Vec<TableFieldSchema>,
    parent_name: &Option<String>,
) {
    for field in fields {
        let is_array = field.mode.as_ref().is_some() && field.mode.as_ref().unwrap() == "REPEATED";
        let complex_object = field.r#type == "RECORD";

        if is_array {
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
        if complex_object { 
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

fn get_bq_table_rows(
    schema: &Option<crate::bigquery::jobs::TableSchema>,
    rows: &Vec<serde_json::Value>,
) -> Vec<Vec<TableItem>> {
    todo!()
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn get_bq_table_header_1() {
        let complex_object_array_test = include_str!("complex_object_array_test.json");
        let complex_object_array_test = &serde_json::from_str::<
            crate::bigquery::jobs::GetQueryResultsResponse,
        >(complex_object_array_test)
        .unwrap();

        let header = &super::get_bq_table_header(&complex_object_array_test.schema);

        // assert_eq!(header.len(), 30);
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
            assert_eq!(&header[i+1], name)
        }

        assert_eq!(&header[29], &"Delete_Flag.#");
        assert_eq!(&header[30], &"Delete_Flag.value");
        assert_eq!(&header[31], &"Delete_Flag.level");

        for i in 29..37 {
            let name = field_names[i];
            assert_eq!(&header[i+3], name);
        }

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Structure_assignments.#");

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Structure_assignments.assignment");

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(
        //         thead_td.inner_html(),
        //         "Structure_assignments.structure_system"
        //     );

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Functionality");

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Promo_Flag.#");

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Promo_Flag.value");

        //     thead_td = thead_td.next_element_sibling().unwrap();
        //     assert_eq!(thead_td.tag_name(), "TH");
        //     assert_eq!(thead_td.inner_html(), "Promo_Flag.country");

        //     for i in 40..55 {
        //         let name = field_names[i];
        //         // console_log!("i: {}, name: {}", i, name);
        //         thead_td = thead_td.next_element_sibling().unwrap();
        //         assert_eq!(thead_td.tag_name(), "TH");
        //         assert_eq!(thead_td.inner_html(), name);
        //     }
    }

    // #[wasm_bindgen_test]
    // fn complex_object_array_test_1() {
    //     let complex_object_array_test = include_str!("complex_object_array_test.json");
    //     let complex_object_array_test = js_sys::JSON::parse(complex_object_array_test).unwrap();
    //     let complex_object_array_test = &serde_wasm_bindgen::from_value::<
    //         crate::bigquery::jobs::GetQueryResultsResponse,
    //     >(complex_object_array_test)
    //     .unwrap();

    //     let element = &createElement("div");
    //     complex_object_array_test.plot_table(element);

    //     let shadow = element.shadow_root().unwrap();
    //     let child = shadow.first_element_child().unwrap();
    //     assert_eq!(child.tag_name(), "STYLE");

    //     let child = child.next_element_sibling().unwrap();
    //     assert_eq!(child.tag_name(), "DIV");
    //     assert_eq!(child.id(), "controls-background");

    //     let child = child.next_element_sibling().unwrap();
    //     assert_eq!(child.tag_name(), "DIV");
    //     assert_eq!(child.id(), "controls");

    //     let table = child.next_element_sibling().unwrap();
    //     assert_eq!(table.tag_name(), "TABLE");

    //     let mut thead_td = table
    //         .first_element_child()
    //         .unwrap()
    //         .first_element_child()
    //         .unwrap()
    //         .first_element_child()
    //         .unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "#");

    //     let field_names = [
    //         "Pim_Value",
    //         "AttributeValueCategory",
    //         "Colour_PDP",
    //         "Width_accessoires",
    //         "Height_accessoires",
    //         "Lining",
    //         "Shop_by_Sport",
    //         "Not_searchable",
    //         "Exclusive_Access",
    //         "Promo_Activity",
    //         "Additional_info",
    //         "Lifestyle",
    //         "Ranking",
    //         "Product_GBPC",
    //         "TradebyteActive_Combi",
    //         "MainColorPDP",
    //         "Sleeve_Length",
    //         "Padding",
    //         "Soldout",
    //         "Neck_Line",
    //         "Key_Looks",
    //         "pimExportDate",
    //         "ProductGroupCategory",
    //         "Heel_Height",
    //         "USP_flag",
    //         "Material_2",
    //         "Combi_number",
    //         "Flavour_Copy",
    //         "Delete_Flag",
    //         "New_Arrivals",
    //         "Combi_Reference",
    //         "RISE",
    //         "CTP_date",
    //         "STYLE",
    //         "Proper_style_name",
    //         "StyleLength",
    //         "Actual_Online_Date",
    //         "Structure_assignments",
    //         "Functionality",
    //         "Promo_Flag",
    //         "Fit_for_bottoms",
    //         "Sustainable",
    //         "Material_3",
    //         "Fit_for_tops",
    //         "Material",
    //         "Program",
    //         "ImageCount",
    //         "Collection",
    //         "Occasion",
    //         "Shop_by_Activity",
    //         "Brand",
    //         "Length_accessoires",
    //         "Backfill_AboutYou",
    //         "DETAIL",
    //         "row_number",
    //     ];

    //     for i in 0..28 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.value");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Delete_Flag.level");

    //     for i in 29..37 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Structure_assignments.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Structure_assignments.assignment");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(
    //         thead_td.inner_html(),
    //         "Structure_assignments.structure_system"
    //     );

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Functionality");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.#");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.value");

    //     thead_td = thead_td.next_element_sibling().unwrap();
    //     assert_eq!(thead_td.tag_name(), "TH");
    //     assert_eq!(thead_td.inner_html(), "Promo_Flag.country");

    //     for i in 40..55 {
    //         let name = field_names[i];
    //         // console_log!("i: {}, name: {}", i, name);
    //         thead_td = thead_td.next_element_sibling().unwrap();
    //         assert_eq!(thead_td.tag_name(), "TH");
    //         assert_eq!(thead_td.inner_html(), name);
    //     }

    //     // values

    //     let tbody = table
    //         .first_element_child()
    //         .unwrap()
    //         .next_element_sibling()
    //         .unwrap();
    //     assert_eq!(tbody.tag_name(), "TBODY");

    //     // assert_eq!(thead_td.inner_html(), "Row");
    // }
}
