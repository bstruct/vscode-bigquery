
# trait base_element_trait

- new<T>(some_id, value: Option<T>)
- render(&self, parent_element: &Node)


# base_element

- id
- element
- new_and_shadow_append
- ...



# trait custom_element_definition

- define




# ideal scenario for bq_table_custom_element

Inside render

```
    BaseElement::new("bq-table", "some_id")
        .append_to_element(some_element)
        .append_shadow()
        .append_child_style("some css path")
        .append_sibling_base_element(DataTableControls::new("some_id_1"))
        .append_sibling_base_element(DataTable::new("some_id_2"))
        ;
```

- How to pass data? Generically.