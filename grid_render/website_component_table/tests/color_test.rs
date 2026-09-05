use wasm_bindgen_test::*;
use website_base::document::{
    create_element_with_children, create_element_with_text, initial_setup, InitialSetup,
};

#[allow(dead_code)]
#[wasm_bindgen_test]
fn apply_color_scheme() {
    let text_content = include_str!("main.css");

    let main_body = vec![create_element_with_children("div", &vec![])
        .unwrap()
        .set_attribute("class", "page")
        .unwrap()];

    // not supported in safari
    // bstruct_browser_base::navigation::set_onnavigate_event();

    let initial_setup_result = initial_setup(&InitialSetup {
        title: String::from("bstruct"),
        head_nodes: vec![create_element_with_text("style", Some(text_content)).unwrap()],
        body_nodes: main_body,
    });

    assert!(initial_setup_result.is_ok());
}
