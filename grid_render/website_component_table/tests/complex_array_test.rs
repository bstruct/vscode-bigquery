use wasm_bindgen_test::*;
use std::collections::HashMap;
use website_base::{
    document::{HtmlNode, create_element_with_children, create_element_with_text},
    struct_node::HtmlNodeRender,
    web_sys,
};
use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup, TableRow,
    TableStyle, TableValue,
};

fn build_delete_flag_table(items: &[serde_json::Value], start_col_index: usize) -> InnerTableBuilder {
    let rows = items
        .iter()
        .map(|item| {
            let value = item
                .get("value")
                .map(|v| {
                    if v.is_null() {
                        "null".to_string()
                    } else {
                        v.as_str().unwrap_or_default().to_string()
                    }
                })
                .unwrap_or_default();
            let level = item
                .get("level")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            TableRow {
                cells: vec![TableValue::String(value), TableValue::String(level)],
            }
        })
        .collect();

    InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 2,
        start_col_index,
        rows,
    }
}

fn build_structure_assignments_table(items: &[serde_json::Value], start_col_index: usize) -> InnerTableBuilder {
    let rows = items
        .iter()
        .map(|item| {
            let assignment = item
                .get("assignment")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let structure_system = item
                .get("structure_system")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            TableRow {
                cells: vec![
                    TableValue::String(assignment),
                    TableValue::String(structure_system),
                ],
            }
        })
        .collect();

    InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 2,
        start_col_index,
        rows,
    }
}

fn build_promo_flag_table(items: &[serde_json::Value], start_col_index: usize) -> InnerTableBuilder {
    let rows = items
        .iter()
        .map(|item| {
            let country = item
                .get("country")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let value = item
                .get("value")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            TableRow {
                cells: vec![TableValue::String(country), TableValue::String(value)],
            }
        })
        .collect();

    InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 2,
        start_col_index,
        rows,
    }
}

fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(v) => v.to_string(),
        serde_json::Value::Number(v) => v.to_string(),
        serde_json::Value::String(v) => v.clone(),
        serde_json::Value::Array(v) => format!("[{} items]", v.len()),
        serde_json::Value::Object(_) => "{...}".to_string(),
    }
}

fn append_to_body(node: &HtmlNode) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();
}

#[wasm_bindgen_test]
fn test_complex_array_table_builder() {
    let json_str = include_str!("complex_array.json");
    let records: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap();

    let first_record = records
        .first()
        .and_then(|v| v.as_object())
        .expect("complex_array.json should contain at least one object row");

    let keys: Vec<String> = first_record.keys().cloned().collect();
    let mut nested_start_col_index: HashMap<String, usize> = HashMap::new();

    let mut columns = vec![TableColumnDefinition::Column(TableColumn {
        name: "index".to_string(),
        text: "#".to_string(),
        width_px: 50,
    })];

    let mut flat_col_index = 1;
    for key in &keys {
        match key.as_str() {
            "Delete_Flag" => {
                nested_start_col_index.insert(key.clone(), flat_col_index);
                columns.push(TableColumnDefinition::Group(TableColumnGroup {
                    name: key.clone(),
                    text: key.clone(),
                    columns: vec![
                        TableColumnDefinition::Column(TableColumn {
                            name: "value".to_string(),
                            text: "value".to_string(),
                            width_px: 90,
                        }),
                        TableColumnDefinition::Column(TableColumn {
                            name: "level".to_string(),
                            text: "level".to_string(),
                            width_px: 90,
                        }),
                    ],
                }));
                flat_col_index += 2;
            }
            "Structure_assignments" => {
                nested_start_col_index.insert(key.clone(), flat_col_index);
                columns.push(TableColumnDefinition::Group(TableColumnGroup {
                    name: key.clone(),
                    text: key.clone(),
                    columns: vec![
                        TableColumnDefinition::Column(TableColumn {
                            name: "assignment".to_string(),
                            text: "assignment".to_string(),
                            width_px: 220,
                        }),
                        TableColumnDefinition::Column(TableColumn {
                            name: "structure_system".to_string(),
                            text: "structure_system".to_string(),
                            width_px: 200,
                        }),
                    ],
                }));
                flat_col_index += 2;
            }
            "Promo_Flag" => {
                nested_start_col_index.insert(key.clone(), flat_col_index);
                columns.push(TableColumnDefinition::Group(TableColumnGroup {
                    name: key.clone(),
                    text: key.clone(),
                    columns: vec![
                        TableColumnDefinition::Column(TableColumn {
                            name: "country".to_string(),
                            text: "country".to_string(),
                            width_px: 160,
                        }),
                        TableColumnDefinition::Column(TableColumn {
                            name: "value".to_string(),
                            text: "value".to_string(),
                            width_px: 90,
                        }),
                    ],
                }));
                flat_col_index += 2;
            }
            _ => {
                columns.push(TableColumnDefinition::Column(TableColumn {
                    name: key.clone(),
                    text: key.clone(),
                    width_px: 170,
                }));
                flat_col_index += 1;
            }
        }
    }

    let table = TableBuilder {
        style: TableStyle::solarized_light(),
        dynamic_table_render: false,
        columns,
        rows: records
            .iter()
            .enumerate()
            .map(|(idx, record)| {
                let mut cells = vec![TableValue::Index(idx + 1)];

                for key in &keys {
                    match key.as_str() {
                        "Delete_Flag" => {
                            let start_col_index = *nested_start_col_index
                                .get("Delete_Flag")
                                .expect("Delete_Flag start index should exist");
                            let items = record
                                .get("Delete_Flag")
                                .and_then(|v| v.as_array())
                                .map(|v| v.as_slice())
                                .unwrap_or(&[]);
                            cells.push(TableValue::Array(build_delete_flag_table(items, start_col_index)));
                        }
                        "Structure_assignments" => {
                            let start_col_index = *nested_start_col_index
                                .get("Structure_assignments")
                                .expect("Structure_assignments start index should exist");
                            let items = record
                                .get("Structure_assignments")
                                .and_then(|v| v.as_array())
                                .map(|v| v.as_slice())
                                .unwrap_or(&[]);
                            cells.push(TableValue::Array(build_structure_assignments_table(
                                items,
                                start_col_index,
                            )));
                        }
                        "Promo_Flag" => {
                            let start_col_index = *nested_start_col_index
                                .get("Promo_Flag")
                                .expect("Promo_Flag start index should exist");
                            let items = record
                                .get("Promo_Flag")
                                .and_then(|v| v.as_array())
                                .map(|v| v.as_slice())
                                .unwrap_or(&[]);
                            cells.push(TableValue::Array(build_promo_flag_table(items, start_col_index)));
                        }
                        _ => {
                            let value = record
                                .get(key)
                                .map(json_value_to_string)
                                .unwrap_or_default();
                            cells.push(TableValue::String(value));
                        }
                    }
                }

                TableRow {
                    cells,
                }
            })
            .collect(),
    };

    let rendered = table.render().unwrap();
    assert_eq!(rendered.len(), 1);

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("h2", Some("Complex Array TableBuilder Test")).unwrap(),
                create_element_with_text(
                    "p",
                    Some("Rendered all top-level columns from tests/complex_array.json with expandable nested arrays."),
                )
                .unwrap(),
            ],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &rendered)
            .unwrap()
            .set_attribute("style", "width: 100%; height: 500px; overflow: auto;")
            .unwrap(),
    );
}