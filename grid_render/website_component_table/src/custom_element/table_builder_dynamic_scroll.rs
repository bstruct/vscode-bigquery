use crate::{
    common::get_target_element,
    custom_element::{
        row_column_range::RowsColumnsRange, table_builder_dynamic_render::TableBuilderViewPort,
        table_builder_render::TABLE_BUILDER_PERSISTED,
    },
};
use website_base::{
    base_result::{BaseResult, ToBaseResult},
    wasm_bindgen::{JsCast, JsValue},
};

pub(crate) fn on_connected(event: &website_base::web_sys::Event) {
    on_event(event);
}

pub(crate) fn on_scroll(event: &website_base::web_sys::Event) {
    on_event(event);
}

fn on_event(event: &website_base::web_sys::Event) {
    event.stop_propagation();
    event.prevent_default();

    if let Some(viewport_div) = get_target_element(event)
        && let Ok(viewport_div) = Viewport::from_viewport_div(&viewport_div) {
            let _ = viewport_div.set_css_variables();
            let _ = viewport_div.create_or_move_table();
        }
}

pub(crate) struct Viewport {
    viewport_div: website_base::web_sys::Element,
    display_div: website_base::web_sys::Element,
    table_id: String,
    scroll_top: i32,
    scroll_left: i32,
    height: i32,
    width: i32,
}

impl Viewport {
    fn from_viewport_div(viewport_div: &website_base::web_sys::Element) -> BaseResult<Viewport> {
        if viewport_div.id() != "viewport" {
            return Err("not a viewport div".to_string().into());
        }

        if let Some(table_id) = viewport_div.get_attribute("tid")
            && let Some(display_div) = find_display_div(viewport_div) {
                let scroll_top = viewport_div.scroll_top();
                let scroll_left = viewport_div.scroll_left();
                let height = viewport_div.client_height();
                let width = viewport_div.client_width();

                let viewport_div = Viewport {
                    viewport_div: viewport_div.clone(),
                    display_div,
                    table_id,
                    scroll_top,
                    scroll_left,
                    height,
                    width,
                };

                return Ok(viewport_div);
            }

        Err("unexpected error".into())
    }

    fn set_css_variables(&self) -> BaseResult<()> {
        let style = format!(
            "--scroll_top:{}px;--scroll_left:{}px;--view_height:{}px;--view_width:{}px;",
            self.scroll_top, self.scroll_left, self.height, self.width
        );

        self.viewport_div
            .set_attribute("style", &style)
            .to_base_result()?;

        Ok(())
    }

    fn create_or_move_table(&self) -> BaseResult<()> {
        if let Some(table) = find_table_element(&self.viewport_div) {
            let columns_range = RowsColumnsRange::from_display_div(&self.display_div)?;

            self.set_displace_left(&columns_range)?;
            self.trigger_render_if_needed(&columns_range, &table)?;
        } else {
            self.create_table_first_time()?;
        }

        Ok(())
    }

    fn create_table_first_time(&self) -> BaseResult<()> {
        TABLE_BUILDER_PERSISTED.with(|cell| {
            if let Some(table_builder) = cell.borrow().get(&self.table_id).as_ref() {
                let table_viewport = TableBuilderViewPort {
                    viewport_div: &self.viewport_div,
                    display_div: &self.display_div,
                    table_builder,
                    scroll_top: self.scroll_top,
                    // scroll_left: self.scroll_left,
                    height: self.height,
                    width: self.width,
                    // table_element: &None,
                };

                if let Ok(row_column_range) = table_viewport.create_table_first_time() {
                    let _ = row_column_range.set_properties(&self.display_div);
                }
            }
        });

        Ok(())
    }

    fn set_displace_left(&self, columns_range: &RowsColumnsRange) -> BaseResult<()> {
        let invisible_content_left_width = columns_range.invisible_content_left_width;
        let displace_left = invisible_content_left_width - self.viewport_div.scroll_left();

        self.display_div
            .set_attribute(
                "style",
                format!("--displace-left:{}px;", displace_left).as_str(),
            )
            .to_base_result()?;

        Ok(())
    }

    fn trigger_render_if_needed(
        &self,
        columns_range: &RowsColumnsRange,
        table: &website_base::web_sys::Element,
    ) -> BaseResult<()> {
        let invisible_content_left_width = columns_range.invisible_content_left_width;
        let displace_left = invisible_content_left_width - self.viewport_div.scroll_left();

        let table_width = table.client_width();
        let needs_render = displace_left > 0 || table_width + displace_left < self.width;

        if needs_render {
            self.create_render_timeout(displace_left);
            // website_base::web_sys::console::log_1(&JsValue::from_str("create_render_timeout"));
        } else {
            // website_base::web_sys::console::log_1(&JsValue::from_str("no need to render"));
        }

        Ok(())
    }

    fn create_render_timeout(&self, displace_left: i32) {
        let closure = website_base::wasm_bindgen::prelude::Closure::wrap(
            Box::new(timeout_callback) as Box<dyn FnMut(JsValue, JsValue)>,
        );

        let arg1 = JsValue::from(&self.viewport_div);
        let arg2 = JsValue::from(displace_left);

        let _ = website_base::web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_2(
                closure.as_ref().unchecked_ref(),
                20,
                &arg1,
                &arg2,
            );

        closure.forget();
    }

    fn is_locked(&self) -> bool {
        self.viewport_div.has_attribute("lock")
    }

    fn lock(&self) -> BaseResult<()> {
        self.viewport_div
            .set_attribute("lock", "true")
            .to_base_result()
    }

    fn unlock(&self) -> BaseResult<()> {
        self.viewport_div.remove_attribute("lock").to_base_result()
    }

    fn re_render_table(&self) -> BaseResult<()> {
        let current_rows_columns_range = RowsColumnsRange::from_display_div(&self.display_div)?;

        TABLE_BUILDER_PERSISTED.with(|cell| {
            if let Some(table_builder) = cell.borrow().get(&self.table_id).as_ref() {
                let table_viewport = TableBuilderViewPort {
                    viewport_div: &self.viewport_div,
                    display_div: &self.display_div,
                    table_builder,
                    scroll_top: self.scroll_top,
                    // scroll_left: self.scroll_left,
                    height: self.height,
                    width: self.width,
                    // table_element: &find_table_element(&self.viewport_div),
                };

                if let Ok(new_rows_columns_range) =
                    table_viewport.re_render_table(&current_rows_columns_range)
                {
                    let _ = new_rows_columns_range.set_properties(&self.display_div);
                    let _ = self.set_displace_left(&new_rows_columns_range);
                }
            }
        });

        Ok(())
    }
}

fn timeout_callback(arg1: JsValue, arg2: JsValue) {
    if let Ok(viewport_div) = arg1.dyn_into::<website_base::web_sys::Element>() {
        let displace_left = arg2.as_f64().unwrap_or(0.0) as i32;

        if let Ok(viewport) = Viewport::from_viewport_div(&viewport_div)
            && let Ok(columns_range) = RowsColumnsRange::from_display_div(&viewport.display_div) {
                let invisible_content_left_width = columns_range.invisible_content_left_width;
                let recent_displace_left =
                    invisible_content_left_width - viewport_div.scroll_left();

                if recent_displace_left == displace_left
                    && !viewport.is_locked() {
                        let _ = viewport.lock();
                        let _ = viewport.re_render_table();
                        let _ = viewport.unlock();
                    }
            }
    }
}

fn find_display_div(
    viewport_div: &website_base::web_sys::Element,
) -> Option<website_base::web_sys::Element> {
    viewport_div.query_selector("div#display").unwrap_or_default()
}

fn find_table_element(
    viewport_div: &website_base::web_sys::Element,
) -> Option<website_base::web_sys::Element> {
    viewport_div.query_selector("table").unwrap_or_default()
}
