use wasm_bindgen_test::*;
use website_base::{
    document::{create_element_with_children, create_element_with_text},
    struct_node::HtmlNodeRender,
    web_sys,
};
use website_component_table::*;

#[wasm_bindgen_test]
fn test_expandable_arrays() {
    // Create a table with array data
    let columns = vec![
        TableColumn {
            name: "id".to_string(),
            text: "ID".to_string(),
            width_px: 60,
        },
        TableColumn {
            name: "name".to_string(),
            text: "Name".to_string(),
            width_px: 150,
        },
        TableColumn {
            name: "tags".to_string(),
            text: "Tags".to_string(),
            width_px: 200,
        },
        TableColumn {
            name: "scores".to_string(),
            text: "Scores".to_string(),
            width_px: 150,
        },
    ];

    let t1 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 2,
        rows: vec![
            TableRow {
                cells: vec![TableValue::String("rust".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("wasm".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("web".to_string())],
            },
        ],
    };

    let t2 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 3,
        rows: vec![
            TableRow {
                cells: vec![TableValue::Int(85)],
            },
            TableRow {
                cells: vec![TableValue::Int(90)],
            },
            TableRow {
                cells: vec![TableValue::Int(78)],
            },
        ],
    };

    let t3 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 2,
        rows: vec![
            TableRow {
                cells: vec![TableValue::String("javascript".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("typescript".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("react".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("node".to_string())],
            },
        ],
    };
    let t4 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 3,
        rows: vec![
            TableRow {
                cells: vec![TableValue::Int(88)],
            },
            TableRow {
                cells: vec![TableValue::Int(91)],
            },
            TableRow {
                cells: vec![TableValue::Int(85)],
            },
            TableRow {
                cells: vec![TableValue::Int(90)],
            },
        ],
    };

    let t5 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 2,
        rows: vec![
            TableRow {
                cells: vec![TableValue::String("python".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("django".to_string())],
            },
        ],
    };

    let t6 = InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index: 3,
        rows: vec![
            TableRow {
                cells: vec![TableValue::Int(92)],
            },
            TableRow {
                cells: vec![TableValue::Int(87)],
            },
        ],
    };

    let rows = vec![
        TableRow {
            cells: vec![
                TableValue::Index(0),
                TableValue::String("Alice".to_string()),
                TableValue::Array(t1),
                TableValue::Array(t2),
            ],
        },
        TableRow {
            cells: vec![
                TableValue::Index(1),
                TableValue::String("Bob".to_string()),
                TableValue::Array(t3),
                TableValue::Array(t4),
            ],
        },
        TableRow {
            cells: vec![
                TableValue::Index(2),
                TableValue::String("Charlie".to_string()),
                TableValue::Array(t5),
                TableValue::Array(t6),
            ],
        },
    ];

    let table = TableBuilder {
        style: TableStyle::default_dark(),
        dynamic_table_render: false,
        columns: columns
            .into_iter()
            .map(TableColumnDefinition::Column)
            .collect(),
        rows,
    };

    // Render the table
    let result = table.render();
    assert!(result.is_ok(), "Table rendering failed: {:?}", result.err());

    let html_elements = result.unwrap();
    assert_eq!(html_elements.len(), 1);

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![create_element_with_text("h2", Some("Expandable Array Test")).unwrap()],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &html_elements)
            .unwrap()
            .set_attribute("style", "width: 100%; overflow: auto;")
            .unwrap(),
    );
}

fn append_to_body(node: &website_base::document::HtmlNode) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();
}

#[wasm_bindgen_test]
fn test_large_nested_structures() {
    // Create a table with large nested arrays and matrices
    let columns = vec![
        TableColumnDefinition::Column(TableColumn {
            name: "id".to_string(),
            text: "ID".to_string(),
            width_px: 60,
        }),
        TableColumnDefinition::Column(TableColumn {
            name: "name".to_string(),
            text: "Name".to_string(),
            width_px: 150,
        }),
        TableColumnDefinition::Column(TableColumn {
            name: "large_array".to_string(),
            text: "Large Array (50 items)".to_string(),
            width_px: 250,
        }),
        TableColumnDefinition::Group(TableColumnGroup {
            text: "Matrix (100x10)".to_string(),
            name: "matrix".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "c1".to_string(),
                    text: "c1".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c2".to_string(),
                    text: "c2".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c3".to_string(),
                    text: "c3".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c4".to_string(),
                    text: "c4".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c5".to_string(),
                    text: "c5".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c6".to_string(),
                    text: "c6".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c7".to_string(),
                    text: "c7".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c8".to_string(),
                    text: "c8".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c9".to_string(),
                    text: "c9".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "c10".to_string(),
                    text: "c10".to_string(),
                    width_px: 100,
                }),
            ],
        }),
        TableColumnDefinition::Column(TableColumn {
            name: "last column".to_string(),
            text: "last column".to_string(),
            width_px: 100,
        })
    ];

    let mut rows = vec![];

    // Create 5 rows with large nested data
    for i in 0..5 {
        // Create a 50-element array
        let large_array: Vec<Vec<TableValue>> = (0..50)
            .map(|n| vec![TableValue::String(format!("Item_{}", n))])
            .collect();

        let large_array_table = InnerTableBuilder {
            style: TableStyle::default_light(),
            col_span: 1,
            start_col_index: 2,
            rows: large_array
                .iter()
                .map(|row_values| TableRow {
                    cells: row_values.clone(),
                })
                .collect(),
        };

        // Create a 100×10 matrix
        let matrix: Vec<Vec<TableValue>> = (0..100)
            .map(|row_num| {
                (0..10)
                    .map(|col_num| TableValue::Int((row_num * 10 + col_num) as i128))
                    .collect()
            })
            .collect();
        let matrix_table = InnerTableBuilder {
            style: TableStyle::default_light(),
            col_span: 10,
            start_col_index: 3,
            rows: matrix
                .iter()
                .map(|row_values| TableRow {
                    cells: row_values.clone(),
                })
                .collect(),
        };

        rows.push(TableRow {
            cells: vec![
                TableValue::Index(i),
                TableValue::String(format!("Row_{}", i)),
                TableValue::Array(large_array_table),
                TableValue::Array(matrix_table),
                TableValue::String(format!("last_{}", i)),
            ],
        });
    }

    let table = TableBuilder {
        style: TableStyle::monokai(),
        dynamic_table_render: false,
        columns: columns,
        rows,
    };

    // Render the table
    let result = table.render();
    assert!(result.is_ok(), "Table rendering failed: {:?}", result.err());

    let html_elements = result.unwrap();
    assert_eq!(html_elements.len(), 1);

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("h2", Some("Large Nested Structures Test")).unwrap(),
                create_element_with_text("p", Some("This table contains 50-element arrays and 100x10 matrices. Click the expand buttons to view the content.")).unwrap(),
            ],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &html_elements)
            .unwrap()
            .set_attribute("style", "width: 100%; overflow: auto;")
            .unwrap(),
    );
}
