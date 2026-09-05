use wasm_bindgen_test::*;
use website_base::{
    document::{create_element_with_children, create_element_with_text, HtmlNode},
    struct_node::HtmlNodeRender, web_sys,
};
use website_component_table::{TableBuilder, TableColumn, TableColumnDefinition, TableRow, TableValue};

/// Helper function to append test output to the document body
fn append_to_body(node: &HtmlNode) {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_element_node().unwrap())
        .unwrap();
}

/// Helper function to create a text element node
fn text_node(text: &str) -> HtmlNode {
    create_element_with_text("p", Some(text)).unwrap()
}

/// Helper function to create a heading node
fn heading_node(text: &str) -> HtmlNode {
    create_element_with_text("h2", Some(text)).unwrap()
}

/// Helper function to create an hr node
fn hr_node() -> HtmlNode {
    create_element_with_text("hr", None).unwrap()
}

#[wasm_bindgen_test]
fn test_large_text_truncation() {
    // Create a very large string (1000 characters)
    let large_text = "A".repeat(1000);
    
    // Create a moderately large string (300 characters - should not be truncated)
    let medium_text = "B".repeat(300);
    
    // Create a string right at the limit (500 characters - should not be truncated)
    let limit_text = "C".repeat(500);
    
    // Create a string just over the limit (501 characters - should be truncated)
    let over_limit_text = "D".repeat(501);

    let table = TableBuilder {
        style: website_component_table::TableStyle::solarized_light(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "index".to_string(),
                text: "#".to_string(),
                width_px: 80,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "description".to_string(),
                text: "Description".to_string(),
                width_px: 200,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "content".to_string(),
                text: "Content".to_string(),
                width_px: 400,
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableValue::Index(1),
                    TableValue::String("Very large text (1000 chars)".to_string()),
                    TableValue::String(large_text.clone()),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(2),
                    TableValue::String("Medium text (300 chars)".to_string()),
                    TableValue::String(medium_text.clone()),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(3),
                    TableValue::String("At limit (500 chars)".to_string()),
                    TableValue::String(limit_text.clone()),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(4),
                    TableValue::String("Over limit (501 chars)".to_string()),
                    TableValue::String(over_limit_text.clone()),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(5),
                    TableValue::String("Normal text".to_string()),
                    TableValue::String("This is a normal length string".to_string()),
                ],
            },
        ],

    };

    let rendered = table.render().unwrap();

    // Append to body for visual inspection in browser
    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("p", None).unwrap(),
                heading_node("Large Text Content Test"),
                text_node("This test demonstrates text truncation for content exceeding 500 characters."),
                text_node("• Row 1: 1000 chars - should show truncation message"),
                text_node("• Row 2: 300 chars - should display fully with ellipsis via CSS"),
                text_node("• Row 3: 500 chars - should display fully (at limit)"),
                text_node("• Row 4: 501 chars - should show truncation message"),
                text_node("• Row 5: Normal text - should display normally"),
            ],
        )
        .unwrap(),
    );

    rendered.iter().for_each(|i| append_to_body(i));
    
    append_to_body(&hr_node());
}

#[wasm_bindgen_test]
fn test_text_overflow_ellipsis() {
    // Test that CSS ellipsis works for text that fits in the display length but exceeds column width
    let repeated_text = "This is a long sentence that should overflow the column width. ".repeat(3);

    let table = TableBuilder {
        style: website_component_table::TableStyle::default_dark(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "index".to_string(),
                text: "#".to_string(),
                width_px: 60,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "narrow_col".to_string(),
                text: "Narrow Column".to_string(),
                width_px: 150,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "wide_col".to_string(),
                text: "Wide Column".to_string(),
                width_px: 400,
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableValue::Index(1),
                    TableValue::String(repeated_text.clone()),
                    TableValue::String(repeated_text.clone()),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(2),
                    TableValue::String("Short".to_string()),
                    TableValue::String("Short text".to_string()),
                ],
            },
        ],

    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("p", None).unwrap(),
                heading_node("Text Overflow with Ellipsis Test"),
                text_node("This test shows CSS ellipsis for text that exceeds column width."),
            ],
        )
        .unwrap(),
    );

    rendered.iter().for_each(|i| append_to_body(i));

    append_to_body(&hr_node());
}

#[wasm_bindgen_test]
fn test_mixed_data_types_with_large_content() {
    // Test various data types including large strings
    let large_json = r#"{"key1": "value1", "key2": "value2", "key3": "value3"}"#.repeat(20);

    let table = TableBuilder {
        style: website_component_table::TableStyle::solarized_dark(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "index".to_string(),
                text: "#".to_string(),
                width_px: 60,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "type".to_string(),
                text: "Type".to_string(),
                width_px: 120,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "value".to_string(),
                text: "Value".to_string(),
                width_px: 350,
            }),
        ],
        rows: vec![
            TableRow {
                cells: vec![
                    TableValue::Index(1),
                    TableValue::String("Large String".to_string()),
                    TableValue::String(large_json),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(2),
                    TableValue::String("Number".to_string()),
                    TableValue::Int(42),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(3),
                    TableValue::String("Float".to_string()),
                    TableValue::Float(3.14159),
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(4),
                    TableValue::String("Null".to_string()),
                    TableValue::Null,
                ],
            },
            TableRow {
                cells: vec![
                    TableValue::Index(5),
                    TableValue::String("Normal String".to_string()),
                    TableValue::String("Just a regular string".to_string()),
                ],
            },
        ],

    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("p", None).unwrap(),
                heading_node("Mixed Data Types with Large Content Test"),
                text_node("This test shows different data types including a large JSON string."),
            ],
        )
        .unwrap(),
    );

    rendered.iter().for_each(|i| append_to_body(i));

    append_to_body(&hr_node());
}
