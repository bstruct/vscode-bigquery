use web_sys::Element;

pub(crate) fn get_attribute(element: &Element, attribute_name: &str) -> String {
    let att = element.get_attribute(attribute_name);
    assert!(att.is_some(), "attribute not found: {}", attribute_name);
    att.unwrap()
}

pub(crate) fn set_attribute(element: &web_sys::Element, attribute_name: &str, value: &str) {
    element.set_attribute(attribute_name, value).unwrap();
}

pub(crate) fn remove_attribute(element: &web_sys::Element, attribute_name: &str) {
    element.remove_attribute(attribute_name).unwrap();
}
