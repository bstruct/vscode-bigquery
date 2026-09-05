use crate::{InnerTableBuilder, TableRow, TableValue};
use website_base::{
    base_result::BaseResult,
    document::{HtmlNode, create_element_with_children, create_element_with_text},
    struct_node::HtmlNodeRender,
};

impl HtmlNodeRender for InnerTableBuilder {
    fn render(&self) -> BaseResult<Vec<HtmlNode>> {
        let bstruct_table = create_element_with_text("bstruct-table", None)?;

        bstruct_table
            .attach_shadow(true)?
            .append_children(&vec![self.get_style_element()?, self.get_table_element()?])?;

        Ok(vec![bstruct_table])
    }
}

impl InnerTableBuilder {
    fn get_table_element(&self) -> BaseResult<HtmlNode> {
        let rows = self
            .rows
            .iter()
            .map(|row| self.render_row(row))
            .collect::<BaseResult<Vec<HtmlNode>>>()?;

        create_element_with_children(
            "table",
            &vec![create_element_with_children("tbody", &rows)?],
        )
    }

    /// Render one inner row.
    ///
    /// Every cell receives an explicit `min-width`/`max-width` derived from the
    /// parent table's `--c<N>` column variables, so inner cells line up with the
    /// header columns of the outermost table. A cell that spans several columns
    /// (a nested repeated record rendered as another inner table) gets the sum of
    /// the widths of all the columns it spans. Positional `nth-child` rules cannot
    /// express that, which is why the widths are set per cell here.
    fn render_row(&self, row: &TableRow) -> BaseResult<HtmlNode> {
        let mut col_index = self.start_col_index;
        let mut cells: Vec<HtmlNode> = Vec::with_capacity(row.cells.len());

        for (position, cell) in row.cells.iter().enumerate() {
            let span = cell_col_span(cell);
            let is_array = matches!(cell, TableValue::Array(_));

            for node in cell.render()? {
                node.set_attribute(
                    "style",
                    &self.cell_width_style(position, col_index, span, is_array),
                )?;
                cells.push(node);
            }

            col_index += span;
        }

        create_element_with_children("tr", &cells)
    }

    /// Inline width style for the cell at `position` in its row, occupying `span`
    /// columns starting at flat column `col_index`.
    ///
    /// `min-width`/`max-width` apply to the content box, so the cell's own
    /// horizontal padding and collapsed borders are subtracted from the column
    /// widths. Array cells are rendered with `padding: 0` (`td.array` in every
    /// built-in style), so only the borders are accounted for there.
    fn cell_width_style(
        &self,
        position: usize,
        col_index: usize,
        span: usize,
        is_array: bool,
    ) -> String {
        let border_px = if position == 0 {
            self.style.margin_px
        } else {
            self.style.margin_px * 2
        } as isize;

        let adjust_px: isize = if is_array {
            border_px
        } else {
            border_px - (self.style.padding_px * 2) as isize
        };

        let columns_sum = (col_index..col_index + span)
            .map(|i| format!("var(--c{})", i))
            .collect::<Vec<String>>()
            .join(" + ");

        let width = if adjust_px >= 0 {
            format!("calc({} + {}px)", columns_sum, adjust_px)
        } else {
            format!("calc({} - {}px)", columns_sum, -adjust_px)
        };

        format!("min-width:{};max-width:{};", width, width)
    }
}

/// Number of flat leaf columns a cell occupies in its row.
pub(crate) fn cell_col_span(cell: &TableValue) -> usize {
    match cell {
        TableValue::Array(inner) => inner.col_span.max(1),
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TableStyle;

    fn builder(start_col_index: usize) -> InnerTableBuilder {
        InnerTableBuilder {
            col_span: 3,
            start_col_index,
            style: TableStyle {
                margin_px: 1,
                padding_px: 6,
                css_entries: vec![],
            },
            rows: vec![],
        }
    }

    #[test]
    fn single_column_cell_uses_its_own_column_variable() {
        let b = builder(4);
        assert_eq!(
            b.cell_width_style(0, 4, 1, false),
            "min-width:calc(var(--c4) - 11px);max-width:calc(var(--c4) - 11px);"
        );
        assert_eq!(
            b.cell_width_style(1, 5, 1, false),
            "min-width:calc(var(--c5) - 10px);max-width:calc(var(--c5) - 10px);"
        );
    }

    #[test]
    fn spanning_cell_sums_all_spanned_columns() {
        let b = builder(2);
        assert_eq!(
            b.cell_width_style(1, 3, 2, true),
            "min-width:calc(var(--c3) + var(--c4) + 2px);max-width:calc(var(--c3) + var(--c4) + 2px);"
        );
        assert_eq!(
            b.cell_width_style(0, 3, 1, true),
            "min-width:calc(var(--c3) + 1px);max-width:calc(var(--c3) + 1px);"
        );
    }

    #[test]
    fn array_cell_spans_its_col_span() {
        let inner = TableValue::Array(builder(7));
        assert_eq!(cell_col_span(&inner), 3);
        assert_eq!(cell_col_span(&TableValue::Null), 1);
        assert_eq!(cell_col_span(&TableValue::String("x".into())), 1);
    }
}
