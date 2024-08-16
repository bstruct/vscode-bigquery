
# bq_table_custom_element

Must contain only the HTML functionality. All the BQ API understanding must be deferred to the `bq_to_table` service.


Composition of the "bq-table" (tag name) -- custom element
- shadow root
  - css - style
  - div - `data_table_controls_element`
  - table - `data_table_element`


Methods:
- from_element
- element
- new_element
- render
  


# bq_to_table

Must contain the necessary services to break down the BQ API response and feed it back to the the `bq_table_custom_element`. 

-- impl `BigqueryTableCustomElement`
 - from_job
 - as_query_results_request


-- impl `GetQueryResultsResponse`
 - render_bq_table