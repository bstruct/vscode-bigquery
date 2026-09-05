use website_base::{
    base_result::{BaseResult, ToBaseResult},
    document::{HtmlNode, create_element_with_children},
    struct_node::HtmlNodeRender,
    web_sys::Element,
};

use crate::TableRow;

pub(crate) fn get_tbody(rows: &[TableRow]) -> BaseResult<HtmlNode> {
    create_element_with_children("tbody", &get_rows(rows)?)
}

fn get_rows(rows: &[TableRow]) -> BaseResult<Vec<HtmlNode>> {
    let nodes = rows
        .iter()
        .map(|r| r.render())
        .collect::<BaseResult<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(nodes)
}

pub(crate) fn add_row_cells_to_tbody(
    display_div: &Element,
    rows: &[TableRow],
    prepend_or_append: crate::common::PrependOrAppend,
) -> BaseResult<()> {
    if let Ok(tbody) = display_div.query_selector("table > tbody")
        && let Some(tbody) = tbody {
            for (row_index, row) in rows.iter().enumerate() {
                if let Some(row_element) = tbody.children().get_with_index(row_index as u32) {
                    let cells: Vec<&crate::TableValue> = match prepend_or_append {
                        crate::common::PrependOrAppend::Prepend => row.cells.iter().rev().collect(),
                        crate::common::PrependOrAppend::Append => row.cells.iter().collect(),
                    };

                    for cell in cells {
                        if let Some(cell_node) = cell.render()?.first() {
                            let cell_node = cell_node.to_element_node()?;

                            match prepend_or_append {
                                crate::common::PrependOrAppend::Prepend => row_element
                                    .prepend_with_node_1(&cell_node)
                                    .to_base_result()?,
                                crate::common::PrependOrAppend::Append => row_element
                                    .append_with_node_1(&cell_node)
                                    .to_base_result()?,
                            }
                        }
                    }
                }
            }
        }

    Ok(())
}
