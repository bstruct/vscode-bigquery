use serde::{Deserialize, Serialize};
use wasm_bindgen_test::*;
use website_base::{
    document::{HtmlNode, create_element_with_children, create_element_with_text},
    struct_node::HtmlNodeRender,
    web_sys,
};
use website_component_table::{
    InnerTableBuilder, TableBuilder, TableColumn, TableColumnDefinition, TableColumnGroup, TableRow, TableStyle,
    TableValue,
};

#[derive(Debug, Serialize, Deserialize)]
struct StackOverflowPost {
    id: String,
    title: Option<String>,
    body: Option<String>,
    accepted_answer_id: Option<String>,
    answer_count: Option<String>,
    comment_count: Option<String>,
    community_owned_date: Option<String>,
    creation_date: Option<String>,
    favorite_count: Option<String>,
    last_activity_date: Option<String>,
    last_edit_date: Option<String>,
    last_editor_display_name: Option<String>,
    last_editor_user_id: Option<String>,
    owner_display_name: Option<String>,
    owner_user_id: Option<String>,
    parent_id: Option<String>,
    post_type_id: Option<String>,
    score: Option<String>,
    view_count: Option<String>,
    tags: Option<Vec<String>>,
}

/// Test loading large JSON data file and rendering it
#[wasm_bindgen_test]
fn test_stackoverflow_json_data() {
    // Load the JSON data
    let json_str = include_str!("test_table_1.json");
    let posts: Vec<StackOverflowPost> = serde_json::from_str(json_str).unwrap();

    web_sys::console::log_1(&format!("Loaded {} Stack Overflow posts", posts.len()).into());

    // Take first 100 posts for the test to keep it manageable
    let sample_posts: Vec<&StackOverflowPost> = posts.iter().take(100).collect();

    // Build the table
    let table = TableBuilder {
        style: TableStyle::solarized_light(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "index".to_string(),
                text: "#".to_string(),
                width_px: 60,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "id".to_string(),
                text: "Post ID".to_string(),
                width_px: 100,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "title".to_string(),
                text: "Title".to_string(),
                width_px: 300,
            }),
            TableColumnDefinition::Group(TableColumnGroup {
                name: "stats".to_string(),
                text: "Statistics".to_string(),
                columns: vec![
                    TableColumnDefinition::Column(TableColumn {
                        name: "score".to_string(),
                        text: "Score".to_string(),
                        width_px: 80,
                    }),
                    TableColumnDefinition::Column(TableColumn {
                        name: "view_count".to_string(),
                        text: "Views".to_string(),
                        width_px: 80,
                    }),
                    TableColumnDefinition::Column(TableColumn {
                        name: "answer_count".to_string(),
                        text: "Answers".to_string(),
                        width_px: 80,
                    }),
                ],
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "tags".to_string(),
                text: "Tags".to_string(),
                width_px: 200,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "creation_date".to_string(),
                text: "Created".to_string(),
                width_px: 180,
            }),
        ],
        rows: sample_posts
            .iter()
            .enumerate()
            .map(|(idx, post)| TableRow {
                cells: vec![
                    TableValue::Index(idx + 1),
                    TableValue::String(post.id.clone()),
                    TableValue::String(
                        post.title
                            .clone()
                            .unwrap_or_else(|| "(no title)".to_string()),
                    ),
                    TableValue::Int(
                        post.score
                            .clone()
                            .unwrap_or_else(|| "0".to_string())
                            .parse()
                            .unwrap_or(0),
                    ),
                    TableValue::Int(
                        post.view_count
                            .clone()
                            .unwrap_or_else(|| "0".to_string())
                            .parse()
                            .unwrap_or(0),
                    ),
                    TableValue::Int(
                        post.answer_count
                            .clone()
                            .unwrap_or_else(|| "0".to_string())
                            .parse()
                            .unwrap_or(0),
                    ),
                    TableValue::Array(get_posts_tags_array(&post, 6)),
                    TableValue::String(
                        post.creation_date
                            .clone()
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                ],
            })
            .collect(),
    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text("h2", Some("Stack Overflow Posts (100 of 1000)")).unwrap(),
                create_element_with_text(
                    "p",
                    Some("📊 Large dataset test with real Stack Overflow data"),
                )
                .unwrap(),
            ],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &rendered)
            .unwrap()
            .set_attribute("style", "width: 100%; height: 400px; overflow: auto;")
            .unwrap(),
    );
}

fn get_posts_tags_array(post: &StackOverflowPost, start_col_index: usize) -> InnerTableBuilder {
    let rows = post
        .tags
        .as_ref()
        .map(|tags| {
            tags.iter()
                .map(|tag| TableRow {
                    cells: vec![TableValue::String(tag.clone())],
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);

    InnerTableBuilder {
        style: TableStyle::default_light(),
        col_span: 1,
        start_col_index,
        rows,
    }
}

/// Test with full dataset for stress testing
#[wasm_bindgen_test]
fn test_full_stackoverflow_dataset() {
    let json_str = include_str!("test_table_1.json");
    let posts: Vec<StackOverflowPost> = serde_json::from_str(json_str).unwrap();

    web_sys::console::log_1(
        &format!(
            "Loaded {} Stack Overflow posts for full dataset test",
            posts.len()
        )
        .into(),
    );

    // Build table with ALL posts for stress testing
    let table = TableBuilder {
        style: TableStyle::dracula(),
        dynamic_table_render: false,
        columns: vec![
            TableColumnDefinition::Column(TableColumn {
                name: "index".to_string(),
                text: "#".to_string(),
                width_px: 60,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "id".to_string(),
                text: "ID".to_string(),
                width_px: 80,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "title".to_string(),
                text: "Title".to_string(),
                width_px: 400,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "score".to_string(),
                text: "Score".to_string(),
                width_px: 70,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "views".to_string(),
                text: "Views".to_string(),
                width_px: 80,
            }),
            TableColumnDefinition::Column(TableColumn {
                name: "tags".to_string(),
                text: "Tags".to_string(),
                width_px: 200,
            }),
        ],
        rows: posts
            .iter()
            .enumerate()
            .map(|(idx, post)| TableRow {
                cells: vec![
                    TableValue::Index(idx + 1),
                    TableValue::String(post.id.clone()),
                    TableValue::String(
                        post.title
                            .clone()
                            .unwrap_or_else(|| "(no title)".to_string()),
                    ),
                    TableValue::Int(
                        post.score
                            .clone()
                            .unwrap_or_else(|| "0".to_string())
                            .parse()
                            .unwrap_or(0),
                    ),
                    TableValue::Int(
                        post.view_count
                            .clone()
                            .unwrap_or_else(|| "0".to_string())
                            .parse()
                            .unwrap_or(0),
                    ),
                    TableValue::Array(get_posts_tags_array(&post, 5)),
                ],
            })
            .collect(),
    };

    let rendered = table.render().unwrap();

    append_to_body(
        &create_element_with_children(
            "div",
            &vec![
                create_element_with_text(
                    "h2",
                    Some(&format!(
                        "Full Stack Overflow Dataset ({} posts)",
                        posts.len()
                    )),
                )
                .unwrap(),
                create_element_with_text(
                    "p",
                    Some("Virtual scrolling should handle this large dataset efficiently!"),
                )
                .unwrap(),
            ],
        )
        .unwrap(),
    );

    append_to_body(
        &create_element_with_children("div", &rendered)
            .unwrap()
            .set_attribute("style", "width: 100%; height: 400px; overflow: auto;")
            .unwrap(),
    );
}

fn append_to_body(node: &HtmlNode) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&node.to_node().unwrap()).unwrap();
}
