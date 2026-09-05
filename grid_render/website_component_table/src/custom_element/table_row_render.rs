use website_base::{
    base_result::BaseResult,
    document::{create_element_with_children, HtmlNode},
    struct_node::HtmlNodeRender,
};

use crate::TableRow;

impl HtmlNodeRender for TableRow {
    fn render(&self) -> BaseResult<Vec<HtmlNode>> {
        let children: Vec<HtmlNode> = self
            .cells
            .iter()
            .map(|c| c.render())
            .collect::<BaseResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(vec![create_element_with_children("tr", &children)?])
    }
}
