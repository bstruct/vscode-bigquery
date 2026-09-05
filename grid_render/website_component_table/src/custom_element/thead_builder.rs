use std::vec;

use website_base::{
    base_result::{BaseResult, ToBaseResult},
    document::{HtmlNode, create_element_with_children, create_element_with_text},
    wasm_bindgen::JsCast,
    web_sys::Element,
};

use crate::{
    TableColumnDefinition, TableStyle,
    common::PrependOrAppend,
    custom_element::thead_builder_resize_observer::{observe_element, unobserve_element},
};

pub(crate) fn get_thead(
    columns: &[TableColumnDefinition],
    table_style: &TableStyle,
) -> BaseResult<HtmlNode> {
    create_element_with_children("thead", &get_rows(columns, table_style)?)
}

fn get_rows(
    columns: &[TableColumnDefinition],
    table_style: &TableStyle,
) -> BaseResult<Vec<HtmlNode>> {
    let levels_count = columns.iter().map(get_levels_count).max().unwrap_or(1);

    let mut index: isize = -1;

    let header_cells = columns
        .iter()
        .flat_map(|column_definition| {
            get_cells_per_level(column_definition, &mut index, levels_count, table_style)
        })
        .collect::<Vec<HeaderCell>>();

    let mut rows: Vec<HtmlNode> = Vec::new();
    for level in 0..levels_count {
        let ths = header_cells
            .iter()
            .filter(|p| p.level == level)
            .map(|c| c.get_element())
            .collect::<BaseResult<Vec<HtmlNode>>>()?;

        let row = create_element_with_children("tr", &ths)?;
        rows.push(row);
    }

    Ok(rows)
}

pub(crate) fn add_columns_to_thead(
    display_div: &Element,
    columns: &[TableColumnDefinition],
    levels_count: usize,
    prepend_or_append: PrependOrAppend,
    table_style: &TableStyle,
) -> BaseResult<()> {
    if let Ok(thead) = display_div.query_selector("table > thead")
        && let Some(thead) = thead {
            let mut index: isize = -1;

            let cells_per_level: Vec<HeaderCell> = match prepend_or_append {
                PrependOrAppend::Prepend => columns
                    .iter()
                    .rev()
                    .flat_map(|col| get_cells_per_level(col, &mut index, levels_count, table_style))
                    .collect(),
                PrependOrAppend::Append => columns
                    .iter()
                    .flat_map(|col| get_cells_per_level(col, &mut index, levels_count, table_style))
                    .collect(),
            };

            for level in 0..levels_count {
                let ths = cells_per_level
                    .iter()
                    .filter(|p| p.level == level)
                    .map(|c| c.get_element())
                    .collect::<BaseResult<Vec<HtmlNode>>>()?;

                if let Some(row_x) = thead.children().get_with_index(level as u32) {
                    for th in ths.iter() {
                        let node = &th.to_element_node()?;
                        match prepend_or_append {
                            PrependOrAppend::Prepend => {
                                row_x.prepend_with_node_1(node).to_base_result()?
                            }
                            PrependOrAppend::Append => {
                                row_x.append_with_node_1(node).to_base_result()?
                            }
                        }
                    }
                }
            }
        }

    Ok(())
}

#[derive(Clone, Debug)]
struct HeaderCell {
    pub text: String,
    pub colspan: usize,
    pub rowspan: usize,
    pub index: Option<usize>,
    pub level: usize,
    pub padding_px: usize,
}

impl HeaderCell {
    pub fn get_element(&self) -> BaseResult<HtmlNode> {
        let th = if let Some(index) = self.index {
            self.get_resizable_th_element(index)?.set_attribute(
                "style",
                &format!(
                    "min-width: var(--c{}); max-width: var(--c{});",
                    index, index
                ),
            )?
        } else {
            self.get_text_th_element()?
        };

        if self.colspan != 1 {
            th.set_attribute("colspan", &self.colspan.to_string())?;
        }
        if self.rowspan != 1 {
            th.set_attribute("rowspan", &self.rowspan.to_string())?;
        }
        Ok(th)
    }

    fn get_resizable_th_element(&self, index: usize) -> BaseResult<HtmlNode> {
        let resizable_div = create_element_with_text("div", Some(&self.text))?
            .set_mouse_event_listener("mouseenter", resizable_on_mouse_enter)?
            .set_mouse_event_listener("mouseleave", resizable_on_mouse_leave)?
            .set_attributes(vec![
                ["class", "text resizable"],
                ["index", &index.to_string()],
                ["pad", &self.padding_px.to_string()]
            ])?;

        create_element_with_children("th", &vec![resizable_div])
    }

    fn get_text_th_element(&self) -> BaseResult<HtmlNode> {
        create_element_with_children(
            "th",
            &vec![
                create_element_with_text("div", Some(&self.text))?
                    .set_attribute("class", "text")?,
            ],
        )
    }
}

fn resizable_on_mouse_enter(event: &website_base::web_sys::MouseEvent) {
    if let Some(target) = event.target()
        && event.buttons() != 1
            && let Some(element) = target.dyn_ref::<website_base::web_sys::Element>() {
                observe_element(element);
            }
    event.prevent_default();
}

fn resizable_on_mouse_leave(event: &website_base::web_sys::MouseEvent) {
    if let Some(target) = event.target()
        && event.buttons() != 1
            && let Some(element) = target.dyn_ref::<website_base::web_sys::Element>() {
                unobserve_element(element);
            }
    event.prevent_default();
}

pub(crate) fn get_levels_count(table_column: &TableColumnDefinition) -> usize {
    match table_column {
        TableColumnDefinition::Column(_) => 1,
        TableColumnDefinition::Group(group) => {
            let child_levels = group.columns.iter().map(get_levels_count);
            1 + child_levels.max().unwrap_or(0)
        }
    }
}

fn get_cells_per_level(
    table_column: &TableColumnDefinition,
    index: &mut isize,
    levels_count: usize,
    table_style: &TableStyle,
) -> Vec<HeaderCell> {
    match table_column {
        TableColumnDefinition::Column(column) => {
            *index += 1;

            vec![HeaderCell {
                text: column.text.clone(),
                colspan: 1,
                rowspan: levels_count,
                index: Some(*index as usize),
                level: 0,
                padding_px: table_style.padding_px,
            }]
        }
        TableColumnDefinition::Group(group) => {
            let mut cells = Vec::new();

            for child in &group.columns {
                let mut child_cells =
                    get_cells_per_level(child, index, levels_count - 1, table_style);
                // Update level for child cells
                for cell in &mut child_cells {
                    cell.level += 1;
                }
                cells.extend(child_cells);
            }

            // let colspan: usize = cells.iter().map(|c| c.colspan).sum();
            let colspan: usize = cells
                .iter()
                .filter(|p| p.level == 1)
                .map(|c| c.colspan)
                .sum();

            let group_cell = HeaderCell {
                text: group.text.clone(),
                colspan,
                rowspan: 1,
                index: None,
                level: 0,
                padding_px: table_style.padding_px,
            };

            cells.insert(0, group_cell);

            cells
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TableColumn, TableColumnGroup};

    fn create_test_column(name: &str, width: usize) -> TableColumnDefinition {
        TableColumnDefinition::Column(TableColumn {
            name: name.to_string(),
            text: name.to_string(),
            width_px: width,
        })
    }

    fn create_test_group(name: &str, columns: Vec<TableColumnDefinition>) -> TableColumnDefinition {
        TableColumnDefinition::Group(TableColumnGroup {
            text: name.to_string(),
            name: name.to_string(),
            columns,
        })
    }

    fn get_three_level_group_example() -> Vec<TableColumnDefinition> {
        vec![
            // Simple column
            TableColumnDefinition::Column(TableColumn {
                name: "id".to_string(),
                text: "ID".to_string(),
                width_px: 80,
            }),
            // Nested group with subcolumns
            TableColumnDefinition::Group(TableColumnGroup {
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
                ],
            }),
            // Another nested group with deeper nesting
            TableColumnDefinition::Group(TableColumnGroup {
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
                        text: "Phone".to_string(),
                        columns: vec![
                            TableColumnDefinition::Column(TableColumn {
                                name: "home".to_string(),
                                text: "Home".to_string(),
                                width_px: 120,
                            }),
                            TableColumnDefinition::Group(TableColumnGroup {
                                name: "mobile".to_string(),
                                text: "Mobile".to_string(),
                                columns: vec![
                                    TableColumnDefinition::Column(TableColumn {
                                        name: "personal".to_string(),
                                        text: "Personal".to_string(),
                                        width_px: 120,
                                    }),
                                    TableColumnDefinition::Column(TableColumn {
                                        name: "work".to_string(),
                                        text: "Work".to_string(),
                                        width_px: 120,
                                    }),
                                ],
                            }),
                        ],
                    }),
                ],
            }),
        ]
    }

    #[test]
    fn test_get_levels_count_single_column() {
        let column = create_test_column("Col1", 100);
        assert_eq!(get_levels_count(&column), 1);
    }

    #[test]
    fn test_get_levels_count_single_group() {
        let group = create_test_group(
            "Group1",
            vec![
                create_test_column("Col1", 100),
                create_test_column("Col2", 100),
            ],
        );
        assert_eq!(get_levels_count(&group), 2);
    }

    #[test]
    fn test_get_levels_count_nested_groups() {
        let nested_group = create_test_group(
            "Nested",
            vec![
                create_test_column("Col1", 100),
                create_test_column("Col2", 100),
            ],
        );
        let outer_group =
            create_test_group("Outer", vec![nested_group, create_test_column("Col3", 100)]);
        assert_eq!(get_levels_count(&outer_group), 3);
    }

    #[test]
    fn test_get_levels_count_deeply_nested() {
        let level3 = create_test_column("Col1", 100);
        let level2 = create_test_group("Level2", vec![level3]);
        let level1 = create_test_group("Level1", vec![level2]);
        assert_eq!(get_levels_count(&level1), 3);
    }

    #[test]
    fn test_get_levels_count_three_level_group_example() {
        let table_columns = get_three_level_group_example();

        // ID column
        assert_eq!(get_levels_count(&table_columns[0]), 1);

        // User Information group (has direct column children)
        assert_eq!(get_levels_count(&table_columns[1]), 2);

        // Contact Details group (has nested groups going 4 levels deep)
        assert_eq!(get_levels_count(&table_columns[2]), 4);

        // Overall max should be 4
        let max_levels = table_columns
            .iter()
            .map(get_levels_count)
            .max()
            .unwrap_or(1);
        assert_eq!(max_levels, 4);
    }

    #[test]
    fn test_get_cells_per_level_single_column() {
        let column = create_test_column("Col1", 100);
        let mut index: isize = -1;
        let style = &TableStyle::default();
        let cells = get_cells_per_level(&column, &mut index, 1, style);

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].text, "Col1");
        assert_eq!(cells[0].colspan, 1);
        assert_eq!(cells[0].rowspan, 1);
        assert_eq!(cells[0].index, Some(0));
        assert_eq!(cells[0].level, 0);
    }

    #[test]
    fn test_get_cells_per_level_column_with_rowspan() {
        let column = create_test_column("Col1", 100);
        let mut index: isize = -1;
        let style = &TableStyle::default();
        let cells = get_cells_per_level(&column, &mut index, 3, style); // 3 levels

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].text, "Col1");
        assert_eq!(cells[0].colspan, 1);
        assert_eq!(cells[0].rowspan, 3); // Should span all 3 levels
        assert_eq!(cells[0].level, 0);
    }

    #[test]
    fn test_get_cells_per_level_simple_group() {
        let group = create_test_group(
            "Group1",
            vec![
                create_test_column("Col1", 100),
                create_test_column("Col2", 150),
            ],
        );
        let levels_count = get_levels_count(&group);
        let mut index: isize = -1;
        let style = &TableStyle::default();
        let cells = get_cells_per_level(&group, &mut index, levels_count, style);

        assert_eq!(cells.len(), 3); // 1 group + 2 columns

        // First cell should be the group header
        assert_eq!(cells[0].text, "Group1");
        assert_eq!(cells[0].colspan, 2); // Spans 2 columns
        assert_eq!(cells[0].rowspan, 1);
        assert_eq!(cells[0].level, 0);
        assert_eq!(cells[0].index, None);

        // Second cell should be Col1
        assert_eq!(cells[1].text, "Col1");
        assert_eq!(cells[1].colspan, 1);
        assert_eq!(cells[1].rowspan, 1);
        assert_eq!(cells[1].level, 1);
        assert_eq!(cells[1].index, Some(0));

        // Third cell should be Col2
        assert_eq!(cells[2].text, "Col2");
        assert_eq!(cells[2].colspan, 1);
        assert_eq!(cells[2].rowspan, 1);
        assert_eq!(cells[2].level, 1);
        assert_eq!(cells[2].index, Some(1));
    }

    #[test]
    fn test_get_cells_per_level_nested_groups() {
        let table_columns = get_three_level_group_example();
        let style = &TableStyle::default();

        let levels_count = table_columns
            .iter()
            .map(get_levels_count)
            .max()
            .unwrap_or(1);
        assert_eq!(levels_count, 4); // Deepest nesting: Contact -> Phone -> Mobile -> columns

        let mut index: isize = -1;

        let all_cells: Vec<HeaderCell> = table_columns
            .iter()
            .flat_map(|column_definition| {
                get_cells_per_level(column_definition, &mut index, levels_count, style)
            })
            .collect();

        // Verify ID column (simple column at level 0, should span all 4 levels)
        let id_cells: Vec<&HeaderCell> = all_cells.iter().filter(|c| c.text == "ID").collect();
        assert_eq!(id_cells.len(), 1);
        assert_eq!(id_cells[0].level, 0);
        assert_eq!(id_cells[0].rowspan, 4);
        assert_eq!(id_cells[0].colspan, 1);

        // Verify User Information group (level 0)
        let user_info_cells: Vec<&HeaderCell> = all_cells
            .iter()
            .filter(|c| c.text == "User Information")
            .collect();
        assert_eq!(user_info_cells.len(), 1);
        assert_eq!(user_info_cells[0].level, 0);
        assert_eq!(user_info_cells[0].rowspan, 1);
        assert_eq!(user_info_cells[0].colspan, 2); // First Name + Last Name

        // Verify First Name and Last Name (level 1, should span remaining 3 levels)
        let first_name_cells: Vec<&HeaderCell> = all_cells
            .iter()
            .filter(|c| c.text == "First Name")
            .collect();
        assert_eq!(first_name_cells.len(), 1);
        assert_eq!(first_name_cells[0].level, 1);
        assert_eq!(first_name_cells[0].rowspan, 3);

        // Verify Contact Details group (level 0)
        let contact_cells: Vec<&HeaderCell> = all_cells
            .iter()
            .filter(|c| c.text == "Contact Details")
            .collect();
        assert_eq!(contact_cells.len(), 1);
        assert_eq!(contact_cells[0].level, 0);
        assert_eq!(contact_cells[0].colspan, 4); // Email + Home + Personal + Work

        // Verify Email (level 1, should span remaining 3 levels)
        let email_cells: Vec<&HeaderCell> =
            all_cells.iter().filter(|c| c.text == "Email").collect();
        assert_eq!(email_cells.len(), 1);
        assert_eq!(email_cells[0].level, 1);
        assert_eq!(email_cells[0].rowspan, 3);

        // Verify Phone group (level 1)
        let phone_cells: Vec<&HeaderCell> =
            all_cells.iter().filter(|c| c.text == "Phone").collect();
        assert_eq!(phone_cells.len(), 1);
        assert_eq!(phone_cells[0].level, 1);
        assert_eq!(phone_cells[0].colspan, 3); // Home + Personal + Work

        // Verify Home (level 2, should span remaining 2 levels)
        let home_cells: Vec<&HeaderCell> = all_cells.iter().filter(|c| c.text == "Home").collect();
        assert_eq!(home_cells.len(), 1);
        assert_eq!(home_cells[0].level, 2);
        assert_eq!(home_cells[0].rowspan, 2);

        // Verify Mobile group (level 2)
        let mobile_cells: Vec<&HeaderCell> =
            all_cells.iter().filter(|c| c.text == "Mobile").collect();
        assert_eq!(mobile_cells.len(), 1);
        assert_eq!(mobile_cells[0].level, 2);
        assert_eq!(mobile_cells[0].colspan, 2); // Personal + Work

        // Verify Personal and Work (level 3, deepest level, rowspan 1)
        let personal_cells: Vec<&HeaderCell> =
            all_cells.iter().filter(|c| c.text == "Personal").collect();
        assert_eq!(personal_cells.len(), 1);
        assert_eq!(personal_cells[0].level, 3);
        assert_eq!(personal_cells[0].rowspan, 1);

        let work_cells: Vec<&HeaderCell> = all_cells.iter().filter(|c| c.text == "Work").collect();
        assert_eq!(work_cells.len(), 1);
        assert_eq!(work_cells[0].level, 3);
        assert_eq!(work_cells[0].rowspan, 1);
    }

    #[test]
    fn test_get_cells_per_level_mixed_levels() {
        let style = &TableStyle::default();

        // Create a group with mixed nesting levels
        let nested = create_test_group("Sub", vec![create_test_column("A", 50)]);
        let group = create_test_group(
            "Main",
            vec![
                nested,
                create_test_column("B", 100),
                create_test_column("C", 100),
            ],
        );

        let mut index: isize = -1;
        let cells = get_cells_per_level(&group, &mut index, 3, style);

        // Main group at level 0
        assert_eq!(cells[0].text, "Main");
        assert_eq!(cells[0].level, 0);
        assert_eq!(cells[0].colspan, 3);

        // Sub group at level 1
        assert_eq!(cells[1].text, "Sub");
        assert_eq!(cells[1].level, 1);
        assert_eq!(cells[1].colspan, 1);

        // Column A at level 2
        assert_eq!(cells[2].text, "A");
        assert_eq!(cells[2].level, 2);

        // Columns B and C at level 1
        assert_eq!(cells[3].text, "B");
        assert_eq!(cells[3].level, 1);

        assert_eq!(cells[4].text, "C");
        assert_eq!(cells[4].level, 1);
    }
}
