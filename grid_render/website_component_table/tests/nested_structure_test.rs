use wasm_bindgen_test::*;
use website_base::{
    document::{create_element_with_children, create_element_with_text, HtmlNode},
    struct_node::HtmlNodeRender,
    web_sys,
};
use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup, TableRow, TableStyle, TableValue
};

/// Test nested column groups with hierarchical headers
#[wasm_bindgen_test]
fn test_nested_column_groups() {
    let table = TableBuilder {
        style: TableStyle::dracula(),
        dynamic_table_render: false,
        columns: vec![
            // Simple column
            TableColumnDefinition::Column(TableColumn {
                name: "id".to_string(),
                text: "ID".to_string(),
                width_px: 80,
            }),
            // Nested group with subcolumns
            TableColumnDefinition::Group(TableColumnGroup {
                name: "user_info".to_string(),
                text: "User Information".to_string(),
                columns: vec![
                    TableColumnDefinition::Column(TableColumn {
                        name: "first_name".to_string(),
                        text: "First Name".to_string(),
                        width_px: 120,
                    }),
                    TableColumnDefinition::Column(TableColumn {
                        name: "last_name".to_string(),
                        text: "Last Name".to_string(),
                        width_px: 120,
                    }),
                ],
            }),
            // Another nested group with deeper nesting
            TableColumnDefinition::Group(TableColumnGroup {
                name: "contact".to_string(),
                text: "Contact Details".to_string(),
                columns: vec![
                    TableColumnDefinition::Column(TableColumn {
                        name: "email".to_string(),
                        text: "Email".to_string(),
                        width_px: 200,
                    }),
                    TableColumnDefinition::Group(TableColumnGroup {
                        name: "phone".to_string(),
                        text: "Phone".to_string(),
                        columns: vec![
                            TableColumnDefinition::Column(TableColumn {
                                name: "home".to_string(),
                                text: "Home".to_string(),
                                width_px: 120,
                            }),
                            TableColumnDefinition::Group(TableColumnGroup {
                                name: "mobile".to_string(),
                                text: "Mobile".to_string(),
                                columns: vec![
                                    TableColumnDefinition::Column(TableColumn {
                                        name: "personal".to_string(),
                                        text: "Personal".to_string(),
                                        width_px: 120,
                                    }),
                                    TableColumnDefinition::Column(TableColumn {
                                        name: "work".to_string(),
                                        text: "Work".to_string(),
                                        width_px: 120,
                                    }),
                                ],
                            }),
                        ],
                    }),
                ],
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableValue::Index(1),
                    TableValue::String("John".to_string()),
                    TableValue::String("Doe".to_string()),
                    TableValue::String("john.doe@example.com".to_string()),
                    TableValue::String("555-0100".to_string()),
                    TableValue::String("555-0101-personal".to_string()),
                    TableValue::Null,
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(2),
                    TableValue::String("Jane".to_string()),
                    TableValue::String("Smith".to_string()),
                    TableValue::String("jane.smith@example.com".to_string()),
                    TableValue::String("555-0200".to_string()),
                    TableValue::String("555-0202-personal".to_string()),
                    TableValue::String("555-0203-work".to_string()),
                ],
            },
        ],
    };

    let rendered = table.render().unwrap();

    // Append to body for visual verification in browser tests
    append_to_body(
        &create_element_with_children(
            "div",
            &vec![create_element_with_text("h2", Some("Nested Column Groups Test")).unwrap()],
        )
        .unwrap(),
    );

    rendered.iter().for_each(|i| append_to_body(i));
}

/// Test simple array values
#[wasm_bindgen_test]
fn test_simple_arrays() {


    let t1 = InnerTableBuilder {
        style: TableStyle::gruvbox_dark(),
        col_span: 1,
        start_col_index: 1,
        rows: vec![
            TableRow {
                cells: vec![TableValue::String("tag1".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("tag2".to_string())],
            },
            TableRow {
                cells: vec![TableValue::String("tag3".to_string())],
            },
        ],
    };

    let t2 = InnerTableBuilder {
        style: TableStyle::gruvbox_dark(),
        col_span: 2,
        start_col_index: 2,
        rows: vec![
            TableRow {
                cells: vec![TableValue::Int(1), TableValue::Int(2)],
            },
            TableRow {
                cells: vec![TableValue::Int(3), TableValue::Int(4)],
            },
        ],
    };


    let table = TableBuilder {
        style: TableStyle::gruvbox_dark(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "id".to_string(),
                text: "ID".to_string(),
                width_px: 80,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "tags".to_string(),
                text: "Tags".to_string(),
                width_px: 200,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "matrix".to_string(),
                text: "Matrix".to_string(),
                width_px: 200,
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableValue::Index(1),
                    TableValue::Array(t1),
                    TableValue::Array(t2),
                ],
            },
        ],

    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("h2", Some("Simple Arrays Test")).unwrap(),
            ],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &rendered)
            .unwrap()
            .set_attribute("style", "width: 100%; overflow: auto;")
            .unwrap(),
    );
}

fn append_to_body(node: &HtmlNode) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();
}
