use std::cell::RefCell;
use std::collections::HashMap;

use website_base::{
    base_result::BaseResult,
    custom_element,
    document::{HtmlNode, create_element, create_element_with_children},
    struct_node::HtmlNodeRender,
    web_sys::{Element, Event},
};

use crate::TableBuilder;

// Using thread_local to store dynamic table state
// only used if dynamic_table_render 
thread_local! {
    pub(crate) static TABLE_BUILDER_PERSISTED: RefCell<HashMap<String, TableBuilder>> = RefCell::new(HashMap::new());
}

impl HtmlNodeRender for TableBuilder {
    fn render(&self) -> BaseResult<Vec<HtmlNode>> {
        let table_id = self.persist_state_if_needed();

        let css = self.get_style_elements()?;

        //custom element - `bstruct-table`
        let main_element = self.create_custom_element()?;

        let shadow = main_element.attach_shadow(true)?;
        let content = &self.get_content()?;
        shadow.append_children(&css)?.append_children(content)?;

        // set table id attribute if persisted
        if let Some(table_id) = table_id
            && let Some(element) = content.first() {
                main_element.set_attribute("tid", &table_id)?;
                element.set_attribute("tid", &table_id)?;
            }

        Ok(vec![main_element])
    }
}

impl TableBuilder {
    fn persist_state_if_needed(&self) -> Option<String> {
        if self.dynamic_table_render {
            let table_id = get_or_create_table_id();
            TABLE_BUILDER_PERSISTED.with(|cell| {
                let mut borrowed = cell.borrow_mut();
                borrowed.insert(table_id.clone(), self.clone());
            });

            return Some(table_id);
        }

        None
    }

    fn create_custom_element(&self) -> BaseResult<HtmlNode> {
        let custom_element = &custom_element::CustomElementDefinition {
            tag_name: "bstruct-table".to_string(),
        }
        .check_and_define()?;

        let element = create_element(&custom_element.tag_name)?
            .set_attribute("style", self.get_custom_element_style())?;

        if self.dynamic_table_render {
            element
                .set_event_listener("connected", on_connected)?
                .set_event_listener("disconnected", on_disconnected)?;
        }

        Ok(element)
    }

    fn get_custom_element_style(&self) -> &str {
        if self.dynamic_table_render {
            "overflow:clip;height:100%;width:100%;display:block;"
        } else {
            ""
        }
    }

    fn get_content(&self) -> BaseResult<Vec<HtmlNode>> {
        match self.dynamic_table_render {
            true => self.get_dynamic_base_setup(),
            false => self.get_table(),
        }
    }

    fn get_table(&self) -> BaseResult<Vec<HtmlNode>> {
        let table = create_element_with_children(
            "table",
            &vec![
                crate::custom_element::thead_builder::get_thead(&self.columns, &self.style)?,
                crate::custom_element::tbody_builder::get_tbody(&self.rows)?,
            ],
        )?;
        Ok(vec![table])
    }

    fn get_dynamic_base_setup(&self) -> BaseResult<Vec<HtmlNode>> {
        let elements = vec![
            create_element("div")?.set_attribute("id", "margin_top")?,
            create_element("div")?.set_attribute("id", "margin_left")?,
            create_element("div")?.set_attribute("id", "display")?,
            create_element("div")?.set_attribute("id", "margin_right")?,
            create_element("div")?.set_attribute("id", "margin_bottom")?,
        ];

        let viewport_div = create_element_with_children("div", &elements)?
            .set_attribute("id", "viewport")?
            .set_event_listener(
                "scroll",
                crate::custom_element::table_builder_dynamic_scroll::on_scroll,
            )?
            .set_event_listener(
                "connected",
                crate::custom_element::table_builder_dynamic_scroll::on_connected,
            )?;

        Ok(vec![viewport_div])
    }
}

fn get_or_create_table_id() -> String {
    // Generate a unique ID based on the table's memory address or use a UUID
    // For now, using a simple counter-based approach
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    format!(
        "t{}",
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    )
}

fn on_connected(event: &website_base::web_sys::Event) {
    event.stop_propagation();
    event.prevent_default();

    if let Some(viewport_div) = find_div_viewport(event)
        && let Ok(event) = Event::new("connected") {
            let _ = viewport_div.dispatch_event(&event);
        }
}

fn on_disconnected(event: &website_base::web_sys::Event) {
    event.stop_propagation();
    event.prevent_default();

    if let Some(custom_component) = crate::common::get_target_element(event)
        && let Some(table_id) = custom_component.get_attribute("tid") {
            TABLE_BUILDER_PERSISTED.with(|cell| {
                let mut borrowed = cell.borrow_mut();
                borrowed.remove(&table_id);
            });
        }
}

fn find_div_viewport(event: &website_base::web_sys::Event) -> Option<Element> {
    if let Some(element) = crate::common::get_target_element(event)
        && let Some(shadow) = element.shadow_root()
            && let Ok(viewport_div) = shadow.query_selector("div#viewport") {
                return viewport_div;
            }
    None
}
