use web_sys::Node;
use super::base_element::BaseElement;

pub trait BaseElementTrait {
    fn get_element_id(&self) -> &str;
    fn render(&self, parent_node: &Node) -> BaseElement;
}
