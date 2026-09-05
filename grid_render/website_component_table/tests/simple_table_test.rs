use wasm_bindgen_test::*;
use website_base::{
    document::{create_element_with_children, create_element_with_text, HtmlNode},
    struct_node::HtmlNodeRender,
    web_sys,
};
use website_component_table::{
    TableBuilder, TableColumn, TableColumnDefinition, TableRow, TableStyle,
    TableValue,
};

#[allow(dead_code)]
#[wasm_bindgen_test]
fn simple_table_1() {
    let table = TableBuilder {
        style: TableStyle::one_dark(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "main_index".to_string(),
                text: "#".to_string(),
                width_px: 100,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "c2".to_string(),
                text: "c2".to_string(),
                width_px: 100,
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![TableValue::Index(1), TableValue::Null],
            },
            TableRow {
                cells: vec![TableValue::Index(2), TableValue::String("123".to_string())],
            },
            TableRow {
                cells: vec![TableValue::Index(3), TableValue::String("456".to_string())],
            },
        ],
    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("p", None).unwrap(),
                create_element_with_text("h2", Some("simple_table_1")).unwrap(),
                create_element_with_text("p", None).unwrap(),
            ],
        )
        .unwrap(),
    );

    rendered.iter().for_each(|i| append_to_body(i));
}

// #[allow(dead_code)]
// #[wasm_bindgen_test]
// fn simple_table_2_large() {
//     // Generate 50 columns
//     let mut columns = vec![TableColumnDefinition::Column(TableColumn {
//         name: "index".to_string(),
//         text: "#".to_string(),
//         data_type: TableColumnDataType::Index,
//         width_px: 60,
//     })];

//     for i in 1..50 {
//         let data_type = match i % 5 {
//             0 => TableColumnDataType::Number,
//             1 => TableColumnDataType::Text,
//             2 => TableColumnDataType::Boolean,
//             3 => TableColumnDataType::Date,
//             _ => TableColumnDataType::Text,
//         };

//         columns.push(TableColumnDefinition::Column(TableColumn {
//             name: format!("col_{}", i),
//             text: format!("Column {}", i),
//             data_type,
//             width_px: 120,
//         }));
//     }

//     // Generate 50 rows
//     let mut rows = Vec::new();
//     for row_num in 0..50 {
//         let mut cells = vec![TableValue::Index(row_num + 1)];

//         for col_num in 1..50 {
//             let value = match col_num % 5 {
//                 0 => TableValue::Int((row_num * col_num) as i128),
//                 1 => TableValue::String(format!("Text_{}_{}", row_num, col_num)),
//                 2 => TableValue::Float((row_num as f64 + col_num as f64) / 2.0),
//                 3 => {
//                     if row_num % 3 == 0 {
//                         TableValue::Null
//                     } else {
//                         TableValue::String(format!(
//                             "2025-{:02}-{:02}",
//                             (col_num % 12) + 1,
//                             (row_num % 28) + 1
//                         ))
//                     }
//                 }
//                 _ => TableValue::String(format!("Data_{}", row_num * col_num)),
//             };
//             cells.push(value);
//         }

//         rows.push(TableRow { cells });
//     }

//     let table = TableBuilder {
//         dynamic_table_render: true,
//         style: Default::default(),
//         columns,
//         rows,
//     };

//     let rendered = table.render().unwrap();

//     append_to_body(
//         &create_element_with_children(
//             "div",
//             &vec![
//                 create_element_with_text("p", None).unwrap(),
//                 create_element_with_text("h2", Some("simple_table_2_large (50x50)")).unwrap(),
//                 create_element_with_text("p", Some("This table has 50 columns and 50 rows for testing scrolling and viewport behavior.")).unwrap(),
//             ],
//         )
//         .unwrap(),
//     );

//     append_to_body(
//         &create_element_with_children("div", &rendered)
//             .unwrap()
//             .set_attribute("style", "width: 100%; height: 400px; overflow: auto;")
//             .unwrap(),
//     );
// }

// #[allow(dead_code)]
// #[wasm_bindgen_test]
// fn simple_table_3_virtual_scroll() {
//     // Generate 20 columns
//     let mut columns = vec![
//         TableColumnDefinition::Column(TableColumn {
//             name: "index".to_string(),
//             text: "#".to_string(),
//             data_type: TableColumnDataType::Index,
//             width_px: 60,
//         })
//     ];

//     for i in 1..20 {
//         let data_type = match i % 4 {
//             0 => TableColumnDataType::Number,
//             1 => TableColumnDataType::Text,
//             2 => TableColumnDataType::Date,
//             _ => TableColumnDataType::Text,
//         };

//         columns.push(TableColumnDefinition::Column(TableColumn {
//             name: format!("col_{}", i),
//             text: format!("C{}", i),
//             data_type,
//             width_px: 100,
//         }));
//     }

//     // Generate 1000 rows for virtual scrolling test!
//     let mut rows = Vec::new();
//     for row_num in 0..1000 {
//         let mut cells = vec![TableValue::Index(row_num + 1)];

//         for col_num in 1..20 {
//             let value = match col_num % 4 {
//                 0 => TableValue::Int((row_num * col_num) as i128),
//                 1 => TableValue::String(format!("Row{}_Col{}", row_num, col_num)),
//                 2 => {
//                     if row_num % 7 == 0 {
//                         TableValue::Null
//                     } else {
//                         TableValue::String(format!("2025-{:02}-{:02}", (col_num % 12) + 1, (row_num % 28) + 1))
//                     }
//                 },
//                 _ => TableValue::Float((row_num as f64 + col_num as f64) / 3.14159),
//             };
//             cells.push(value);
//         }

//         rows.push(TableRow { cells });
//     }

//     let table = TableBuilder {
//         dynamic_table_render: true,
//         style: Default::default(),
//         columns,
//         rows,
//     };

//     let rendered = table.render().unwrap();

//     append_to_body(
//         &create_element_with_children(
//             "div",
//             &vec![
//                 create_element_with_text("p", None).unwrap(),
//                 create_element_with_text("h2", Some("simple_table_3_virtual_scroll (1000x20)")).unwrap(),
//                 create_element_with_text("p", Some("🚀 This table has 1,000 rows and 20 columns (20,000 cells!) with VIRTUAL SCROLLING. Only visible rows are rendered!")).unwrap(),
//             ],
//         )
//         .unwrap(),
//     );

//     append_to_body(
//         &create_element_with_children("div", &rendered)
//             .unwrap()
//             .set_attribute("style", "width: 100%; height: 400px; overflow: auto;")
//             .unwrap(),
//     );
// }

fn append_to_body(node: &HtmlNode) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();
}
