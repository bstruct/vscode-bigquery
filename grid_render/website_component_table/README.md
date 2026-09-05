# Component Table

A lightweight, customizable table component built with vanilla JavaScript.

### Basic Usage

```rust
let repeated_text = "This is a long sentence that should overflow the column width. ".repeat(3);

  let table = TableBuilder {
      style: website_component_table::TableStyle::default_dark(),
      dynamic_table_render: false,
      columns: vec![
          TableColumnDefinition::Column(TableColumn {
              name: "index".to_string(),
              text: "#".to_string(),
              width_px: 60,
          }),
          TableColumnDefinition::Column(TableColumn {
              name: "narrow_col".to_string(),
              text: "Narrow Column".to_string(),
              width_px: 150,
          }),
          TableColumnDefinition::Column(TableColumn {
              name: "wide_col".to_string(),
              text: "Wide Column".to_string(),
              width_px: 400,
          }),
      ],
      rows: vec![
          TableRow {
              cells: vec![
                  TableValue::Index(1),
                  TableValue::String(repeated_text.clone()),
                  TableValue::String(repeated_text.clone()),
              ],
          },
          TableRow {
              cells: vec![
                  TableValue::Index(2),
                  TableValue::String("Short".to_string()),
                  TableValue::String("Short text".to_string()),
              ],
          },
      ],

  };

  let rendered = table.render().unwrap();
```

