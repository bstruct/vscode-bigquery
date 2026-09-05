use crate::TableColumnDefinition;

impl TableColumnDefinition {
    pub(crate) fn calculate_width(&self) -> usize {
        self.get_widths().iter().sum()
    }

    pub(crate) fn get_widths(&self) -> Vec<usize> {
        match self {
            TableColumnDefinition::Column(col) => vec![col.width_px],
            TableColumnDefinition::Group(group) => group
                .columns
                .iter()
                .flat_map(|col| col.get_widths())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{TableColumn, TableColumnGroup};

    use super::*;

    #[test]
    fn test_calculate_width_single_column() {
        let column = TableColumnDefinition::Column(TableColumn {
            name: "test".to_string(),
            text: "Test Column".to_string(),
            width_px: 150,
        });

        assert_eq!(column.calculate_width(), 150);
    }

    #[test]
    fn test_calculate_width_column_different_sizes() {
        let column1 = TableColumnDefinition::Column(TableColumn {
            name: "col1".to_string(),
            text: "Column 1".to_string(),
            width_px: 100,
        });

        let column2 = TableColumnDefinition::Column(TableColumn {
            name: "col2".to_string(),
            text: "Column 2".to_string(),
            width_px: 250,
        });

        assert_eq!(column1.calculate_width(), 100);
        assert_eq!(column2.calculate_width(), 250);
    }

    #[test]
    fn test_calculate_width_group_with_two_columns() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "group".to_string(),
            text: "Group Header".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "col1".to_string(),
                    text: "Col 1".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "col2".to_string(),
                    text: "Col 2".to_string(),
                    width_px: 200,
                }),
            ],
        });

        assert_eq!(group.calculate_width(), 300); // 100 + 200
    }

    #[test]
    fn test_calculate_width_group_with_three_columns() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "user_info".to_string(),
            text: "User Information".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "first_name".to_string(),
                    text: "First Name".to_string(),
                    width_px: 120,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "last_name".to_string(),
                    text: "Last Name".to_string(),
                    width_px: 120,
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "email".to_string(),
                    text: "Email".to_string(),
                    width_px: 200,
                }),
            ],
        });

        assert_eq!(group.calculate_width(), 440); // 120 + 120 + 200
    }

    #[test]
    fn test_calculate_width_nested_groups() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "contact".to_string(),
            text: "Contact Details".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "email".to_string(),
                    text: "Email".to_string(),
                    width_px: 200,
                }),
                TableColumnDefinition::Group(TableColumnGroup {
                    name: "phone".to_string(),
                    text: "Phone Numbers".to_string(),
                    columns: vec![
                        TableColumnDefinition::Column(TableColumn {
                            name: "home".to_string(),
                            text: "Home".to_string(),
                            width_px: 120,
                        }),
                        TableColumnDefinition::Column(TableColumn {
                            name: "mobile".to_string(),
                            text: "Mobile".to_string(),
                            width_px: 120,
                        }),
                    ],
                }),
            ],
        });

        assert_eq!(group.calculate_width(), 440); // 200 + 120 + 120
    }

    #[test]
    fn test_calculate_width_deeply_nested_groups() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "level1".to_string(),
            text: "Level 1".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "col1".to_string(),
                    text: "Column 1".to_string(),
                    width_px: 100,
                }),
                TableColumnDefinition::Group(TableColumnGroup {
                    name: "level2".to_string(),
                    text: "Level 2".to_string(),
                    columns: vec![
                        TableColumnDefinition::Column(TableColumn {
                            name: "col2".to_string(),
                            text: "Column 2".to_string(),
                            width_px: 150,
                        }),
                        TableColumnDefinition::Group(TableColumnGroup {
                            name: "level3".to_string(),
                            text: "Level 3".to_string(),
                            columns: vec![
                                TableColumnDefinition::Column(TableColumn {
                                    name: "col3".to_string(),
                                    text: "Column 3".to_string(),
                                    width_px: 80,
                                }),
                                TableColumnDefinition::Column(TableColumn {
                                    name: "col4".to_string(),
                                    text: "Column 4".to_string(),
                                    width_px: 120,
                                }),
                            ],
                        }),
                    ],
                }),
            ],
        });

        assert_eq!(group.calculate_width(), 450); // 100 + 150 + 80 + 120
    }

    #[test]
    fn test_calculate_width_empty_group() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "empty".to_string(),
            text: "Empty Group".to_string(),
            columns: vec![],
        });

        assert_eq!(group.calculate_width(), 0);
    }

    #[test]
    fn test_calculate_width_mixed_group() {
        let group = TableColumnDefinition::Group(TableColumnGroup {
            name: "mixed".to_string(),
            text: "Mixed Group".to_string(),
            columns: vec![
                TableColumnDefinition::Column(TableColumn {
                    name: "id".to_string(),
                    text: "ID".to_string(),
                    width_px: 80,
                }),
                TableColumnDefinition::Group(TableColumnGroup {
                    name: "sub_group".to_string(),
                    text: "Sub Group".to_string(),
                    columns: vec![TableColumnDefinition::Column(TableColumn {
                        name: "name".to_string(),
                        text: "Name".to_string(),
                        width_px: 150,
                    })],
                }),
                TableColumnDefinition::Column(TableColumn {
                    name: "status".to_string(),
                    text: "Status".to_string(),
                    width_px: 100,
                }),
            ],
        });

        assert_eq!(group.calculate_width(), 330); // 80 + 150 + 100
    }
}
