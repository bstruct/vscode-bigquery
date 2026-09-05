use website_base::base_result::BaseResult;

#[derive(Clone)]
pub(crate) struct RowsColumnsRange {
    /// to help with rowspan
    pub(crate) header_max_rows: usize,
    pub(crate) invisible_content_left_width: i32,
    pub(crate) start_column: usize,
    pub(crate) end_column: usize,
    pub(crate) start_row: usize,
    pub(crate) end_row: usize,
}

impl std::fmt::Display for RowsColumnsRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[hmr:{}, clw:{}, r0:{}, r1:{}, c0:{}, c1:{}]",
            self.header_max_rows,
            self.invisible_content_left_width,
            self.start_row,
            self.end_row,
            self.start_column,
            self.end_column
        )
    }
}

impl RowsColumnsRange {
    pub(crate) fn set_properties(
        &self,
        display_div: &website_base::web_sys::Element,
    ) -> BaseResult<Self> {
        website_base::document::HtmlNode::ElementNode(display_div.clone()).set_attributes(vec![
            ["hmr", &self.header_max_rows.to_string()],
            ["clw", &self.invisible_content_left_width.to_string()],
            ["r0", &self.start_row.to_string()],
            ["r1", &self.end_row.to_string()],
            ["c0", &self.start_column.to_string()],
            ["c1", &self.end_column.to_string()],
        ])?;

        Ok(self.clone())
    }

    pub(crate) fn from_display_div(
        display_div: &website_base::web_sys::Element,
    ) -> BaseResult<RowsColumnsRange> {
        let header_max_rows = get_attr_usize_value(display_div, "hmr");
        let invisible_content_left_width = get_attr_i32_value(display_div, "clw");
        let start_row = get_attr_usize_value(display_div, "r0");
        let end_row = get_attr_usize_value(display_div, "r1");
        let start_col = get_attr_usize_value(display_div, "c0");
        let end_col = get_attr_usize_value(display_div, "c1");

        let columns_range = RowsColumnsRange {
            header_max_rows,
            invisible_content_left_width,
            start_column: start_col,
            end_column: end_col,
            start_row,
            end_row,
        };

        Ok(columns_range)
    }
}

fn get_attr_usize_value(element: &website_base::web_sys::Element, attr_name: &str) -> usize {
    element
        .get_attribute(attr_name)
        .unwrap_or_else(|| "0".to_string())
        .parse::<usize>()
        .unwrap_or(0)
}

fn get_attr_i32_value(element: &website_base::web_sys::Element, attr_name: &str) -> i32 {
    element
        .get_attribute(attr_name)
        .unwrap_or_else(|| "0".to_string())
        .parse::<i32>()
        .unwrap_or(0)
}