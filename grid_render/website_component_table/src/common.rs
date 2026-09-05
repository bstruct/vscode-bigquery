use website_base::wasm_bindgen::JsCast;

pub(crate) fn get_target_element(
    event: &website_base::web_sys::Event,
) -> Option<website_base::web_sys::Element> {
    if let Some(target) = event.target()
        && let Ok(base_element) = target.dyn_into::<website_base::web_sys::Element>() {
            return Some(base_element);
        }
    None
}

#[allow(dead_code)]
pub(crate) fn measure_performance<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let performance = website_base::web_sys::window()
        .and_then(|w| w.performance())
        .expect("performance should be available");

    let start = performance.now();
    let result = f();
    let end = performance.now();

    website_base::web_sys::console::log_1(&format!("{}: {:.2}ms", name, end - start).into());

    result
}

pub(crate) enum PrependOrAppend {
    Prepend,
    Append,
}
