use web_sys::Node;
use super::base_element::BaseElement;

pub trait BaseElementTrait<T> {
    fn new(id: &str, value: &Option<T>) -> Self;
    fn render(&self, parent_node: &Node) -> BaseElement;
}
