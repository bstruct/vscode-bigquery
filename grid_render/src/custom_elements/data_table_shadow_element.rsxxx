use web_sys::{Element, ShadowRoot};

pub(crate) struct DataTableShadow;

impl DataTableShadow {
    pub(crate) fn init_shadow(element: &Element) -> ShadowRoot {
        match element.shadow_root() {
            Some(root) => {
                while root.child_element_count() > 1 {
                    root.remove_child(&root.last_child().unwrap()).unwrap();
                }
                root
            }
            None => init(element),
        }
    }
}

fn init(element: &Element) -> ShadowRoot {
    let shadow_init = web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open);
    let shadow = element.attach_shadow(&shadow_init).unwrap();

    let shadow_style = crate::createElement("style");
    let css_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/grid.css"));
    shadow_style.set_inner_html(css_content);
    shadow.append_child(&shadow_style).unwrap();
    shadow
}
