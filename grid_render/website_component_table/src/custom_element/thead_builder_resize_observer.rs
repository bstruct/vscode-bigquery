use std::cell::RefCell;
use website_base::{
    wasm_bindgen::{JsCast, JsValue, prelude::Closure},
    web_sys::{ResizeObserver, ResizeObserverEntry},
};

// Using thread_local persist a ResizeObserver
thread_local! {
    pub(crate) static RESIZE_OBSERVER: RefCell<ResizeObserver>  = RefCell::new(create_resize_observer());
}

fn create_resize_observer() -> ResizeObserver {
    let callback = Closure::wrap(Box::new(on_resize) as Box<dyn Fn(JsValue)>);
    let observer = ResizeObserver::new(callback.as_ref().unchecked_ref())
        .expect("Failed to create ResizeObserver");
    callback.forget();
    observer
}

fn on_resize(entries: JsValue) {
    if let Some(entries) = entries.dyn_ref::<website_base::web_sys::js_sys::Array>() {
        entries
            .iter()
            .map(|i| i.dyn_into::<ResizeObserverEntry>())
            .filter_map(Result::ok)
            .for_each(update_width_variable);
    }
}

fn update_width_variable(entry: ResizeObserverEntry) {
    if let Some(target) = entry.target().dyn_ref::<website_base::web_sys::Element>()
        && let Ok(Some(table)) = target.closest("table")
            && let Some(table) = table.dyn_ref::<website_base::web_sys::HtmlElement>() {

                let resize_info = get_resize_info(target);
                let content_rect = entry.content_rect();
                let new_width = content_rect.width().ceil() as u32 + resize_info.padding_px * 2;

                let _ = table.style().set_property(
                    &format!("--c{}", resize_info.index),
                    &format!("{}px", new_width),
                );
            }
}

struct ResizeInfo {
    index: usize,
    padding_px: u32,
}

fn get_resize_info(element: &website_base::web_sys::Element) -> ResizeInfo {
    let index = element
        .get_attribute("index")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);
    let padding_px = element
        .get_attribute("pad")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);
    ResizeInfo { index, padding_px }
}

pub(crate) fn observe_element(element: &website_base::web_sys::Element) {
    RESIZE_OBSERVER.with(|observer| {
        observer.borrow().observe(element);
    });
}

pub(crate) fn unobserve_element(element: &website_base::web_sys::Element) {
    RESIZE_OBSERVER.with(|observer| {
        observer.borrow().unobserve(element);
    });
}
