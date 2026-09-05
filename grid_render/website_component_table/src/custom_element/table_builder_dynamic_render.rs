use website_base::{
    base_result::{BaseResult, ToBaseResult},
    document::create_element_with_children,
    wasm_bindgen::JsCast,
};

use crate::{
    TableRow,
    common::PrependOrAppend,
    custom_element::{
        row_column_range::RowsColumnsRange,
        tbody_builder, thead_builder,
        visible_column_range_calculator::{CalculatedVisibleColumnRange, VisibleColumnRange},
    },
};

pub(crate) struct TableBuilderViewPort<'a> {
    pub(crate) viewport_div: &'a website_base::web_sys::Element,
    pub(crate) display_div: &'a website_base::web_sys::Element,
    // pub(crate) table_element: &'a Option<website_base::web_sys::Element>,
    pub(crate) table_builder: &'a crate::TableBuilder,

    pub(crate) scroll_top: i32,
    // pub(crate) scroll_left: i32,
    pub(crate) height: i32,
    pub(crate) width: i32,
}

impl TableBuilderViewPort<'_> {
    fn calculate_visible_column_range(&self) -> CalculatedVisibleColumnRange {
        VisibleColumnRange {
            scroll_left: self.viewport_div.scroll_left(),
            border_width: 1,
            visible_width: self.width as usize,
            columns: &self.table_builder.columns,
        }
        .calculate()
    }

    fn calculate_number_of_visible_row_range(&self) -> (usize, usize) {
        let row_height = 20usize; // Assume each row is 20px height
        let start_row = (self.scroll_top as usize) / row_height;
        let end_row = ((self.scroll_top + self.height) as usize) / row_height;
        (start_row, end_row)
    }

    fn get_max_header_levels(&self) -> usize {
        self.table_builder
            .columns
            .iter()
            .map(thead_builder::get_levels_count)
            .max()
            .unwrap_or(1)
    }

    pub(crate) fn create_table_first_time(&self) -> BaseResult<RowsColumnsRange> {
        let col_range = self.calculate_visible_column_range();
        let row_range = self.calculate_number_of_visible_row_range();
        let header_max_rows = self.get_max_header_levels();
        //
        // Build table with only visible rows and columns
        let columns =
            &self.table_builder.columns[col_range.start_column..col_range.end_column + 1].to_vec();
        let rows = &self.table_builder.rows[row_range.0..row_range.1]
            .iter()
            .map(|r| TableRow {
                cells: r.cells[col_range.start_column..col_range.end_column + 1].to_vec(),
            })
            .collect::<Vec<TableRow>>();

        let table = create_element_with_children(
            "table",
            &vec![
                thead_builder::get_thead(columns, &self.table_builder.style).unwrap(),
                tbody_builder::get_tbody(rows).unwrap(),
            ],
        )?;

        let _ = website_base::document::HtmlNode::ElementNode(self.display_div.clone())
            .append_children(&vec![table])?;

        Ok(RowsColumnsRange {
            header_max_rows,
            invisible_content_left_width: 0,
            start_column: col_range.start_column,
            end_column: col_range.end_column,
            start_row: row_range.0,
            end_row: row_range.1,
        })
    }

    pub(crate) fn re_render_table(
        &self,
        current_rows_columns_range: &RowsColumnsRange,
    ) -> BaseResult<RowsColumnsRange> {
        let col_range = self.calculate_visible_column_range();
        let row_range = self.calculate_number_of_visible_row_range();

        let count_cols_to_remove_left =
            col_range.start_column.saturating_sub(current_rows_columns_range.start_column);
        let count_cols_to_remove_right =
            current_rows_columns_range.end_column.saturating_sub(col_range.end_column);
            
        if count_cols_to_remove_left > 0 {
            for _ in 0..count_cols_to_remove_left {
                self
                    .display_div
                    .query_selector_all(
                        "table tbody tr td:first-child, table thead tr th:first-child",
                    )
                    .to_base_result()?
                    .values()
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .map(|e| e.dyn_into::<website_base::web_sys::Element>().ok().unwrap())
                    .for_each(|e| e.remove());
            }
        }

        if count_cols_to_remove_right > 0 {
            for _ in 0..count_cols_to_remove_right {
                self
                    .display_div
                    .query_selector_all(
                        "table tbody tr td:last-child, table thead tr th:last-child",
                    )
                    .to_base_result()?
                    .values()
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .map(|e| e.dyn_into::<website_base::web_sys::Element>().ok().unwrap())
                    .for_each(|e| e.remove());
            }
        }

        if col_range.start_column < current_rows_columns_range.start_column {
            let columns_to_add_left = &self.table_builder.columns
                [col_range.start_column..current_rows_columns_range.start_column]
                .to_vec();
            thead_builder::add_columns_to_thead(
                self.display_div,
                columns_to_add_left,
                current_rows_columns_range.header_max_rows,
                PrependOrAppend::Prepend,
                &self.table_builder.style,
            )?;

            let row_cells_to_add_left = &self.table_builder.rows
                [row_range.0..row_range.1]
                .iter()
                .map(|r| TableRow {
                    cells: r.cells[col_range.start_column..current_rows_columns_range.start_column]
                        .to_vec(),
                })
                .collect::<Vec<TableRow>>();

            tbody_builder::add_row_cells_to_tbody(self.display_div, row_cells_to_add_left, PrependOrAppend::Prepend)?;
        }

        if col_range.end_column > current_rows_columns_range.end_column {
            let columns_to_add_right = &self.table_builder.columns
                [current_rows_columns_range.end_column + 1..=col_range.end_column]
                .to_vec();
            thead_builder::add_columns_to_thead(
                self.display_div,
                columns_to_add_right,
                current_rows_columns_range.header_max_rows,
                PrependOrAppend::Append,
                &self.table_builder.style,
            )?;

            let row_cells_to_add_right = &self.table_builder.rows
                [row_range.0..row_range.1]
                .iter()
                .map(|r| TableRow {
                    cells: r.cells
                        [current_rows_columns_range.end_column + 1..=col_range.end_column]
                        .to_vec(),
                })
                .collect::<Vec<TableRow>>();

            tbody_builder::add_row_cells_to_tbody(self.display_div, row_cells_to_add_right, PrependOrAppend::Append)?;
        }

        let rows_columns_range = RowsColumnsRange {
            header_max_rows: current_rows_columns_range.header_max_rows,
            invisible_content_left_width: col_range.invisible_content_left_width,
            start_column: col_range.start_column,
            end_column: col_range.end_column,
            start_row: row_range.0,
            end_row: row_range.1,
        };
        Ok(rows_columns_range)
    }
}
