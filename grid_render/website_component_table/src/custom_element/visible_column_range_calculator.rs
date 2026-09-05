pub(crate) struct VisibleColumnRange<'a> {
    pub(crate) scroll_left: i32,
    pub(crate) border_width: usize,
    pub(crate) visible_width: usize,
    pub(crate) columns: &'a Vec<crate::TableColumnDefinition>,
}

impl VisibleColumnRange<'_> {
    pub(crate) fn calculate(&self) -> CalculatedVisibleColumnRange {
        let mut scroll_left = self.scroll_left;
        let mut displace_left: usize = 0;
        let mut start_column: usize = 0;
        let mut invisible_content_left_width = 0i32;

        // Find the starting column based on scroll_left
        for (index, column) in self.columns.iter().enumerate() {
            if scroll_left == 0 {
                start_column = 0;
                displace_left = 0;
                invisible_content_left_width = 0;
                break;
            } else {
                let col_width = (column.calculate_width() + (self.border_width * 2)) as i32;

                if scroll_left < col_width {
                    start_column = index;
                    displace_left = scroll_left as usize;
                    break;
                } else {
                    scroll_left -= col_width;
                    invisible_content_left_width += col_width;
                }
            }
        }

        let mut accumulative_width = 0usize;
        let mut end_column: usize = 0;

        // Find the ending column based on visible_width
        for (index, column) in self.columns[start_column..].iter().enumerate() {
            let col_width = column.calculate_width() + (self.border_width * 2);
            accumulative_width += col_width;
            end_column = start_column + index;

            if accumulative_width >= self.visible_width {
                break;
            }
        }

        // If we haven't found an end column, use the last column
        if accumulative_width < self.visible_width {
            end_column = self.columns.len().saturating_sub(1);
        }

        CalculatedVisibleColumnRange {
            invisible_content_left_width,
            displace_left,
            start_column,
            end_column,
        }
    }
}

#[derive(Clone)]
pub(crate) struct CalculatedVisibleColumnRange {
    pub(crate) invisible_content_left_width: i32,
    pub(crate) displace_left: usize,
    pub(crate) start_column: usize,
    pub(crate) end_column: usize,
}

impl std::fmt::Display for CalculatedVisibleColumnRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[start_column: {}, end_column: {}, displace_left: {}, invisible_content_left_width: {}]",
            self.start_column,
            self.end_column,
            self.displace_left,
            self.invisible_content_left_width
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TableColumnDefinition;

    fn create_test_columns(count: usize, width: usize) -> Vec<TableColumnDefinition> {
        (0..count)
            .map(|i| {
                TableColumnDefinition::Column(crate::TableColumn {
                    name: format!("col_{}", i),
                    text: format!("Column {}", i),
                    width_px: width,
                })
            })
            .collect()
    }

    #[test]
    fn test_visible_column_range_scroll_left_0() {
        let columns = create_test_columns(10, 50);

        let range = VisibleColumnRange {
            scroll_left: 0,
            border_width: 1,
            visible_width: 200,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(
            result.displace_left, 0,
            "Displace left should be 0 when scroll_left is 0"
        );
        assert_eq!(result.start_column, 0, "Should start at column 0");
        assert_eq!(
            result.end_column, 3,
            "Should end at column 3 (200px / 50px = 4 columns, index 0-3)"
        );
    }

    #[test]
    fn test_visible_column_range_scroll_left_100() {
        let columns = create_test_columns(10, 50);

        let range = VisibleColumnRange {
            scroll_left: 100,
            border_width: 1,
            visible_width: 200,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(
            result.start_column, 1,
            "Should start at column 1 (100px / 50px)"
        );
        assert_eq!(
            result.displace_left, 48,
            "Displace should be 48 when perfectly aligned with column boundary"
        );
        assert_eq!(
            result.end_column, 4,
            "Should end at column 4 (columns 1-4 = 200px)"
        );
    }

    #[test]
    fn test_visible_column_range_scroll_left_partial() {
        let columns = create_test_columns(10, 50);

        let range = VisibleColumnRange {
            scroll_left: 75,
            border_width: 1,
            visible_width: 200,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(
            result.start_column, 1,
            "Should start at column 1 (75px is within column 1)"
        );
        assert_eq!(
            result.displace_left, 23,
            "Displace should be 23px (75px - 50px of column 0)"
        );
        assert!(
            result.end_column >= 4,
            "Should include enough columns to cover 200px visible width"
        );
    }

    #[test]
    fn test_visible_column_range_scroll_left_end_of_table() {
        let columns = create_test_columns(5, 50);

        let range = VisibleColumnRange {
            scroll_left: 200,
            border_width: 1,
            visible_width: 200,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(result.start_column, 3, "Should start at last column");
        assert_eq!(
            result.end_column, 4,
            "Should end at last column when scrolled to end"
        );
    }

    #[test]
    fn test_visible_column_range_large_visible_width() {
        let columns = create_test_columns(5, 50);

        let range = VisibleColumnRange {
            scroll_left: 0,
            border_width: 1,
            visible_width: 1000,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(result.start_column, 0, "Should start at column 0");
        assert_eq!(
            result.end_column, 4,
            "Should end at last column when viewport is larger than table"
        );
    }

    #[test]
    fn test_visible_column_range_varying_widths() {
        let mut columns = Vec::new();
        columns.push(TableColumnDefinition::Column(crate::TableColumn {
            name: "col_0".to_string(),
            text: "Column 0".to_string(),
            width_px: 60,
        }));
        create_test_columns(20, 100)
            .into_iter()
            .for_each(|col| columns.push(col));

        let range = VisibleColumnRange {
            scroll_left: 120,
            border_width: 1,
            visible_width: 752,
            columns: &columns,
        };

        let result = range.calculate();

        assert_eq!(result.start_column, 1);
        assert_eq!(result.displace_left, 58);
        assert_eq!(result.invisible_content_left_width, 62);
        assert_eq!(result.end_column, 8);
    }
}
