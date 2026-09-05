//! Hover toolbar for grid cells whose text does not fit in the column.
//!
//! When the pointer is over a clipped text cell, a small toolbar appears at the
//! cell's top-right corner with two actions:
//!
//! * **Copy** — writes the full cell value to the clipboard (falls back to asking
//!   the extension host to do it when the browser clipboard API is unavailable).
//! * **Open** — posts the full value to the extension host, which opens it in a
//!   new editor tab.
//!
//! One toolbar is installed per `bq-query` / `bq-table` host element, inside its
//! shadow root. Cell detection is done through `Event::composed_path()`, so cells
//! of nested inner tables (arrays of structs, arrays inside arrays) work too.

use std::cell::RefCell;

use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{Element, Event, EventTarget};

pub(crate) const TOOLS_BE_ID: &str = "cell-tools";
const INSTALLED_ATT: &str = "cell_tools";
const BTN_COPY: &str = "btn_copy_cell";
const BTN_OPEN: &str = "btn_open_cell";
const VISIBLE_CLASS: &str = "visible";
const COPIED_FEEDBACK_MS: i32 = 1500;

/// Message commands understood by the extension host (see `resultsGridRender.ts`
/// and `bqnbController.ts`).
pub(crate) const COMMAND_COPY_CELL_VALUE: &str = "copy_cell_value";
pub(crate) const COMMAND_OPEN_CELL_VALUE: &str = "open_cell_value";

thread_local! {
    /// The cell the toolbar is currently shown for. Only one toolbar is visible
    /// at a time, whichever grid on the page it belongs to.
    static ACTIVE_CELL: RefCell<Option<Element>> = const { RefCell::new(None) };
    static ACTIVE_TOOLS: RefCell<Option<Element>> = const { RefCell::new(None) };
}

/// Install the toolbar into `host`'s shadow root. Safe to call on every render:
/// the work is done only once per host element.
pub(crate) fn install(host: &Element) {
    if host.has_attribute(INSTALLED_ATT) {
        return;
    }

    let shadow = match host.shadow_root() {
        Some(s) => s,
        None => return,
    };

    let tools = match build_toolbar() {
        Some(t) => t,
        None => return,
    };

    // Insert as the *second* child, right after the `<style>` element:
    // - `BaseElement::append_child` (used for the style) is positional and reuses
    //   whatever the first child is, so the toolbar must not be first;
    // - pagination replaces the *last* child of the shadow root with a loading
    //   placeholder, so the toolbar must not be last either.
    let shadow_node: &web_sys::Node = &shadow;
    let anchor = shadow_node.first_child().and_then(|first| first.next_sibling());
    if shadow_node.insert_before(&tools, anchor.as_ref()).is_err() {
        return;
    }

    if host.set_attribute(INSTALLED_ATT, "1").is_err() {
        return;
    }

    // `mousemove` rather than `mouseover`: mouseover/mouseout are retargeted (and
    // dropped) when the pointer moves between nodes of a nested shadow tree, so a
    // host-level listener would never see cells of inner tables. mousemove has no
    // relatedTarget and always carries the full composed path.
    add_listener(host, "mousemove", on_pointer_move);
    add_listener(host, "mouseleave", |_| hide());
    add_listener(&tools, "click", on_toolbar_click);
}

fn build_toolbar() -> Option<Element> {
    let document = web_sys::window()?.document()?;
    let tools = document.create_element("div").ok()?;
    tools.set_attribute("be_id", TOOLS_BE_ID).ok()?;
    tools.set_inner_html(&format!(
        concat!(
            r#"<button be_id="{copy}" title="Copy the full cell value">"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 16 16" fill="currentColor">"#,
            r#"<path d="M4 4V2.5A1.5 1.5 0 0 1 5.5 1h8A1.5 1.5 0 0 1 15 2.5v8a1.5 1.5 0 0 1-1.5 1.5H12v-1.5h1.5v-8h-8V4H4z"/>"#,
            r#"<path d="M1 5.5A1.5 1.5 0 0 1 2.5 4h8A1.5 1.5 0 0 1 12 5.5v8a1.5 1.5 0 0 1-1.5 1.5h-8A1.5 1.5 0 0 1 1 13.5v-8zm1.5 0v8h8v-8h-8z"/>"#,
            r#"</svg><span>Copy</span></button>"#,
            r#"<button be_id="{open}" title="Open the full cell value in a new editor tab">"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 16 16" fill="currentColor">"#,
            r#"<path d="M9 1h6v6h-1.5V3.56L7.03 10.03 5.97 8.97 12.44 2.5H9V1z"/>"#,
            r#"<path d="M2 3.5A1.5 1.5 0 0 1 3.5 2H7v1.5H3.5v9h9V9H14v3.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 12.5v-9z"/>"#,
            r#"</svg><span>Open</span></button>"#,
        ),
        copy = BTN_COPY,
        open = BTN_OPEN,
    ));
    Some(tools)
}

fn add_listener<F>(target: &EventTarget, event_type: &str, f: F)
where
    F: Fn(&Event) + 'static,
{
    let closure = Closure::wrap(Box::new(f) as Box<dyn Fn(&Event)>);
    if let Err(e) =
        target.add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
    {
        web_sys::console::error_1(&JsValue::from_str(&format!(
            "cell_tools: failed to add '{}' listener: {:?}",
            event_type, e
        )));
    }
    closure.forget();
}

/// Show the toolbar when the pointer is over a clipped text cell; hide it when
/// the pointer is over anything else (except the toolbar itself).
fn on_pointer_move(event: &Event) {
    let mut cell: Option<Element> = None;

    for item in event.composed_path().iter() {
        let element = match item.dyn_into::<Element>() {
            Ok(e) => e,
            Err(_) => continue,
        };

        if element.get_attribute("be_id").as_deref() == Some(TOOLS_BE_ID) {
            // Pointer is on the toolbar: keep it open.
            return;
        }

        if cell.is_none() && is_text_cell(&element) {
            cell = Some(element);
        }
    }

    // Same cell as before: nothing to do (mousemove fires continuously).
    let unchanged = ACTIVE_CELL.with(|active| match (active.borrow().as_ref(), cell.as_ref()) {
        (Some(active), Some(td)) => active.is_same_node(Some(td)),
        (None, None) => true,
        _ => false,
    });
    if unchanged {
        return;
    }

    let tools = event
        .current_target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .and_then(|host| host.shadow_root())
        .and_then(|shadow| {
            shadow
                .query_selector(&format!("[be_id='{}']", TOOLS_BE_ID))
                .ok()
                .flatten()
        });

    match (cell, tools) {
        (Some(td), Some(tools)) if is_clipped(&td) => show(&tools, &td),
        _ => hide(),
    }
}

fn is_text_cell(element: &Element) -> bool {
    element.tag_name().eq_ignore_ascii_case("td")
        && element.class_name().split_whitespace().any(|c| c == "text")
}

/// True when the cell's content is wider than the visible cell box.
fn is_clipped(td: &Element) -> bool {
    td.scroll_width() > td.client_width()
}

fn show(tools: &Element, td: &Element) {
    let previous = ACTIVE_TOOLS.with(|t| t.borrow_mut().replace(tools.clone()));
    if let Some(previous) = previous {
        if !previous.is_same_node(Some(tools)) {
            let _ = previous.remove_attribute("class");
        }
    }
    // Make it visible first so it has a layout box to measure.
    let _ = tools.set_attribute("class", VISIBLE_CLASS);

    // Position relative to the host element (`:host { position: relative }`),
    // right-aligned with the cell's top-right corner. A cell narrower than the
    // toolbar gets the toolbar anchored to its left edge instead, so it spills
    // over the next cell rather than hiding the start of the value.
    let host_rect = tools
        .parent_node()
        .and_then(|n| n.dyn_into::<web_sys::ShadowRoot>().ok())
        .map(|s| s.host().get_bounding_client_rect());
    let (offset_left, offset_top) = match host_rect {
        Some(r) => (r.left(), r.top()),
        None => (0.0, 0.0),
    };

    let cell_rect = td.get_bounding_client_rect();
    let tools_width = tools.get_bounding_client_rect().width();
    let left = (cell_rect.right() - tools_width).max(cell_rect.left());

    let _ = tools.set_attribute(
        "style",
        &format!(
            "top:{}px;left:{}px;",
            cell_rect.top() - offset_top,
            left - offset_left
        ),
    );

    ACTIVE_CELL.with(|c| *c.borrow_mut() = Some(td.clone()));
}

fn hide() {
    if let Some(tools) = ACTIVE_TOOLS.with(|t| t.borrow_mut().take()) {
        let _ = tools.remove_attribute("class");
    }
    ACTIVE_CELL.with(|c| *c.borrow_mut() = None);
}

fn active_cell_text() -> Option<String> {
    ACTIVE_CELL.with(|c| c.borrow().as_ref().and_then(|td| td.text_content()))
}

fn on_toolbar_click(event: &Event) {
    event.stop_propagation();

    let button = event
        .target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .and_then(|el| el.closest("button").ok().flatten());

    let button = match button {
        Some(b) => b,
        None => return,
    };

    let text = match active_cell_text() {
        Some(t) => t,
        None => return,
    };

    match button.get_attribute("be_id").as_deref() {
        Some(BTN_COPY) => {
            copy_to_clipboard(&text);
            flash_button_label(&button, "Copied");
        }
        Some(BTN_OPEN) => post_to_host(COMMAND_OPEN_CELL_VALUE, &text),
        _ => {}
    }
}

/// Write `text` to the clipboard with `navigator.clipboard.writeText`. If the
/// API is missing or the write is rejected, ask the extension host to copy it.
fn copy_to_clipboard(text: &str) {
    match clipboard_write_text(text) {
        Ok(promise) => {
            let fallback_text = text.to_string();
            let on_reject = Closure::wrap(Box::new(move |_reason: JsValue| {
                post_to_host(COMMAND_COPY_CELL_VALUE, &fallback_text);
            }) as Box<dyn FnMut(JsValue)>);
            let _ = promise.catch(&on_reject);
            on_reject.forget();
        }
        Err(_) => post_to_host(COMMAND_COPY_CELL_VALUE, text),
    }
}

fn clipboard_write_text(text: &str) -> Result<js_sys::Promise, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let navigator = window.navigator();
    let clipboard = js_sys::Reflect::get(&navigator, &JsValue::from_str("clipboard"))?;
    if clipboard.is_undefined() || clipboard.is_null() {
        return Err(JsValue::from_str("navigator.clipboard unavailable"));
    }
    let write_text = js_sys::Reflect::get(&clipboard, &JsValue::from_str("writeText"))?
        .dyn_into::<js_sys::Function>()?;
    let result = write_text.call1(&clipboard, &JsValue::from_str(text))?;
    Ok(js_sys::Promise::from(result))
}

/// Post `{ command, text }` to the extension host through the `vscode` API
/// object that the webview (or the notebook renderer shim) exposes.
pub(crate) fn post_to_host(command: &str, text: &str) {
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("command"),
        &JsValue::from_str(command),
    );
    let _ = js_sys::Reflect::set(&message, &JsValue::from_str("text"), &JsValue::from_str(text));

    // `vscode` is a global lexical binding (`const vscode = acquireVsCodeApi()`),
    // not a `window` property, so it must be resolved from a script scope.
    let post = js_sys::Function::new_with_args("message", "vscode.postMessage(message);");
    if let Err(e) = post.call1(&JsValue::NULL, &message) {
        web_sys::console::error_1(&JsValue::from_str(&format!(
            "cell_tools: could not post '{}' to the extension host: {:?}",
            command, e
        )));
    }
}

/// Temporarily replace the button's label (e.g. "Copy" → "Copied").
fn flash_button_label(button: &Element, label: &str) {
    let span = match button.query_selector("span").ok().flatten() {
        Some(s) => s,
        None => return,
    };
    let original = span.text_content().unwrap_or_default();
    span.set_text_content(Some(label));
    let _ = button.set_attribute("state", "done");

    let button = button.clone();
    let restore = Closure::once_into_js(move || {
        span.set_text_content(Some(&original));
        let _ = button.remove_attribute("state");
    });

    if let Some(window) = web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            restore.unchecked_ref(),
            COPIED_FEEDBACK_MS,
        );
    }
}
