use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType, BqsqlKeyword};

#[test]
fn query_from_dataset_dot_table() {
    let document =
        BqsqlDocument::parse("SELECT column_a, column_a, column_c, FROM dataset_id.table_id");

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(2, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(4, query_select.items.len());
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_select.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[1].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[2].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[3].item_type
    );

    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(2, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );

    let table_identifier = &query_from.items[1];
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        table_identifier.item_type
    );
    assert_eq!(None, table_identifier.range);
    assert_eq!(3, table_identifier.items.len());

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierProjectId,
        table_identifier.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::Dot,
        table_identifier.items[1].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierTableId,
        table_identifier.items[2].item_type
    );
}

#[test]
fn query_from_string_identifier() {
    let document = BqsqlDocument::parse(
        r#"
SELECT 
    "this is a \" -- string ",  --test, another `table` 123 \"back\" to 'dust'a
    wqert AS something,
    123 as something_else,
    qwer AS new_column,
FROM `Chigau.Events`
order by timestamp;
        "#,
    );

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(4, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(5, query_select.items.len());
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_select.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[1].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[2].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[3].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[4].item_type
    );

    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(2, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );

    let table_identifier = &query_from.items[1];
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        table_identifier.item_type
    );
    assert_eq!(None, table_identifier.range);
    assert_eq!(1, table_identifier.items.len());

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierDatasetIdTableId,
        table_identifier.items[0].item_type
    );

    let query_order_by = &query.items[2];
    assert_eq!(
        BqsqlDocumentItemType::QueryOrderBy,
        query_order_by.item_type
    );

    //;
    assert_eq!(BqsqlDocumentItemType::Semicolon, &query.items[3].item_type);
}

#[test]
fn query_from_full_string_identifier() {
    let document = BqsqlDocument::parse(
        r#"
        SELECT 
            pimExportDate, 
            Combi_number,
            columnC,
            
            -- Flavour_Copy
        FROM `damiao-project-1.PvhTest.PimExport` pim
        WHERE 
            pimExportDate = "2022-03-23"
            -- AND (
            --     Combi_number = '0000F3223E001'
            --     OR Combi_number = "0000F2934E101"
            -- )
        LIMIT 101;"#,
    );

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(5, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(5, query_select.items.len());
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_select.items[0].item_type
    );
  
    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(2, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );

    let table_identifier = &query_from.items[1];
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        table_identifier.item_type
    );
    assert_eq!(None, table_identifier.range);
    assert_eq!(2, table_identifier.items.len());

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierProjectIdDatasetIdTableId,
        table_identifier.items[0].item_type
    );
    assert_eq!(Some([7, 13, 49]), table_identifier.items[0].range);

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierAlias,
        table_identifier.items[1].item_type
    );
    assert_eq!(Some([7, 50, 53]), table_identifier.items[1].range);

    //where
    let query_where = &query.items[2];
    assert_eq!(
        BqsqlDocumentItemType::QueryWhere,
        query_where.item_type
    );

    //where
    let query_limit = &query.items[3];
    assert_eq!(
        BqsqlDocumentItemType::QueryLimit,
        query_limit.item_type
    );

    //;
    assert_eq!(BqsqlDocumentItemType::Semicolon, &query.items[4].item_type);
}

#[test]
fn query_from_query() {
    let document = BqsqlDocument::parse(
        "SELECT column_a, column_a, column_c, FROM (SELECT * FROM dataset_id.table_id)",
    );

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(2, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(4, query_select.items.len());
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_select.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[1].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[2].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[3].item_type
    );

    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(4, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_from.items[1].item_type
    );
    assert_eq!(BqsqlDocumentItemType::Query, query_from.items[2].item_type);
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_from.items[3].item_type
    );
}

#[test]
fn query_from_full_table_name() {
    let document = BqsqlDocument::parse(
        r#"SELECT column_a, column_a, column_c, FROM `project_id.dataset_id.table_id`"#,
    );

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(2, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(4, query_select.items.len());

    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(2, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );

    let table_identifier = &query_from.items[1];
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        table_identifier.item_type
    );
    assert_eq!(None, table_identifier.range);
    assert_eq!(1, table_identifier.items.len());

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierProjectIdDatasetIdTableId,
        table_identifier.items[0].item_type
    );
}

#[test]
fn query_from_project_in_quotes() {
    let document = BqsqlDocument::parse(
        r#"SELECT column_a, column_a, column_c, FROM `project_id`.dataset_id.table_id"#,
    );

    assert_eq!(1, document.items.len());

    //
    //Query
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(2, query.items.len());

    //--- QuerySelect
    let query_select = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(4, query_select.items.len());

    //--- QueryFrom
    let query_from = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(2, query_from.items.len());

    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );

    let table_identifier = &query_from.items[1];
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        table_identifier.item_type
    );
    assert_eq!(None, table_identifier.range);
    assert_eq!(5, table_identifier.items.len());

    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierProjectId,
        table_identifier.items[0].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::Dot,
        table_identifier.items[1].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierDatasetId,
        table_identifier.items[2].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::Dot,
        table_identifier.items[3].item_type
    );
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifierTableId,
        table_identifier.items[4].item_type
    );
}

#[test]
fn queries_with() {
    let document = BqsqlDocument::parse(
        r#"WITH q1 AS (SELECT SchoolID FROM Roster) #my_query
SELECT *
FROM
(WITH q2 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q3 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q1 AS (SELECT * FROM q1),  # q1 (in the query) resolves to my_query
    q4 AS (SELECT * FROM q1)   # q1 resolves to the WITH subquery on the previous line.
SELECT * FROM q1);             # q1 resolves to the third inner WITH subquery."#,
    );

    //
    //Query

    //--- QueryWith
    //--- --- Keyword
    //--- --- QueryCteName
    //--- --- KeywordAs
    //--- --- ParenthesesOpen
    //--- --- Query
    //--- --- --- QuerySelect
    //--- --- ---  ---Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectColumnName
    //--- --- --- QueryFrom
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- ParenthesesClose
    //--- --- LineComment

    //--- QuerySelect
    //--- --- Keyword
    //--- --- QuerySelectListItem
    //--- --- --- QuerySelectStar

    //--- QueryFrom
    //--- --- Keyword

    //--- --- ParenthesesOpen

    //--- --- Query
    //--- --- --- QueryWith
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- Query
    //--- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- Query
    //--- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- --- KeywordAs
    //--- --- --- --- --- ParenthesesOpen
    //--- --- --- --- --- --- Query
    //--- --- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- --- ParenthesesClose
    //--- --- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- --- KeywordAs
    //--- --- --- --- --- ParenthesesOpen
    //--- --- --- --- --- --- Query
    //--- --- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- --- ParenthesesClose

    //--- --- --- QuerySelect
    //--- --- --- --- Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectStar
    //--- --- --- QueryFrom
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName

    //--- --- ParenthesesClose
    //--- Semicolon
    //

    assert_eq!(1, document.items.len());

    //
    //Query

    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(4, query.items.len());

    //--- QueryWith
    let query_with = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QueryWith, query_with.item_type);
    assert_eq!(None, query_with.range);
    assert_eq!(7, query_with.items.len());

    //--- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_with.items[0].item_type
    );
    assert_eq!(Some([0, 0, 4]), query_with.items[0].range);
    assert_eq!(0, query_with.items[0].items.len());

    //--- --- QueryCteName
    assert_eq!(
        BqsqlDocumentItemType::TableCteId,
        query_with.items[1].item_type
    );
    assert_eq!(Some([0, 5, 7]), query_with.items[1].range);
    assert_eq!(0, query_with.items[1].items.len());

    //--- --- KeywordAs
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_with.items[2].item_type
    );
    assert_eq!(Some([0, 8, 10]), query_with.items[2].range);
    assert_eq!(0, query_with.items[2].items.len());
    assert_eq!(Some(BqsqlKeyword::As), query_with.items[2].keyword);

    //--- --- ParenthesesOpen
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_with.items[3].item_type
    );
    assert_eq!(Some([0, 11, 12]), query_with.items[3].range);
    assert_eq!(0, query_with.items[3].items.len());

    //--- --- Query
    assert_eq!(BqsqlDocumentItemType::Query, query_with.items[4].item_type);
    assert_eq!(None, query_with.items[4].range);
    assert_eq!(2, query_with.items[4].items.len());

    let query_1_items = &query_with.items[4].items;
    //--- --- --- QuerySelect
    assert_eq!(
        BqsqlDocumentItemType::QuerySelect,
        query_1_items[0].item_type
    );
    assert_eq!(None, query_1_items[0].range);
    assert_eq!(2, query_1_items[0].items.len());

    let query_1_items_0_items = &query_1_items[0].items;
    //--- --- --- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_1_items_0_items[0].item_type
    );
    assert_eq!(Some([0, 12, 18]), query_1_items_0_items[0].range);
    assert_eq!(0, query_1_items_0_items[0].items.len());

    //--- --- --- --- QuerySelectListItem
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_1_items_0_items[1].item_type
    );
    assert_eq!(None, query_1_items_0_items[1].range);
    assert_eq!(1, query_1_items_0_items[1].items.len());

    //--- --- --- --- --- QuerySelectColumnName

    //--- --- --- QueryFrom
    let query_1_from = &query_1_items[1];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_1_from.item_type);
    assert_eq!(None, query_1_from.range);
    assert_eq!(2, query_1_from.items.len());

    //--- --- --- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_1_from.items[0].item_type
    );
    assert_eq!(Some([0, 28, 32]), query_1_from.items[0].range);
    assert_eq!(0, query_1_from.items[0].items.len());

    //--- --- --- --- QueryCteName
    assert_eq!(
        BqsqlDocumentItemType::TableIdentifier,
        query_1_from.items[1].item_type
    );
    assert_eq!(None, query_1_from.items[1].range);
    assert_eq!(1, query_1_from.items[1].items.len());
    assert_eq!(Some([0, 33, 39]), query_1_from.items[1].items[0].range);

    //--- --- ParenthesesClose
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_with.items[5].item_type
    );
    assert_eq!(Some([0, 39, 40]), query_with.items[5].range);
    assert_eq!(0, query_with.items[5].items.len());

    //--- --- LineComment
    assert_eq!(
        BqsqlDocumentItemType::LineComment,
        query_with.items[6].item_type
    );
    assert_eq!(Some([0, 41, 50]), query_with.items[6].range);
    assert_eq!(0, query_with.items[6].items.len());

    //--- QuerySelect
    let query_select = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_select.items[0].item_type
    );
    assert_eq!(Some([1, 0, 6]), query_select.items[0].range);
    assert_eq!(0, query_select.items[0].items.len());

    //--- --- QuerySelectListItem
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_select.items[1].item_type
    );
    assert_eq!(None, query_select.items[1].range);
    assert_eq!(1, query_select.items[1].items.len());

    //--- --- --- QuerySelectStar
    assert_eq!(
        BqsqlDocumentItemType::Operator,
        query_select.items[1].items[0].item_type
    );
    assert_eq!(Some([1, 7, 8]), query_select.items[1].items[0].range);
    assert_eq!(0, query_select.items[1].items[0].items.len());

    //--- QueryFrom
    let query_from = &query.items[2];
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_from.item_type);
    assert_eq!(None, query_from.range);
    assert_eq!(4, query_from.items.len());

    //--- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_from.items[0].item_type
    );
    assert_eq!(Some([2, 0, 4]), query_from.items[0].range);
    assert_eq!(0, query_from.items[0].items.len());

    //--- --- ParenthesesOpen
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_from.items[1].item_type
    );
    assert_eq!(Some([3, 0, 1]), query_from.items[1].range);
    assert_eq!(0, query_from.items[1].items.len());

    //--- --- Query
    let query_f = &query_from.items[2];
    assert_eq!(BqsqlDocumentItemType::Query, query_f.item_type);
    assert_eq!(None, query_f.range);
    assert_eq!(3, query_f.items.len());

    //--- --- --- QueryWith
    assert_eq!(BqsqlDocumentItemType::QueryWith, query_f.items[0].item_type);
    assert_eq!(None, query_f.items[0].range);
    assert_eq!(28, query_f.items[0].items.len());

    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- Query
    //--- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- Query
    //--- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- --- KeywordAs
    //--- --- --- --- --- ParenthesesOpen
    //--- --- --- --- --- --- Query
    //--- --- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- --- ParenthesesClose
    //--- --- --- --- --- Comma

    //--- --- --- --- QueryCteName
    //--- --- --- --- --- KeywordAs
    //--- --- --- --- --- ParenthesesOpen
    //--- --- --- --- --- --- Query
    //--- --- --- --- --- --- --- QuerySelect
    //--- --- --- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- --- --- QueryCteName
    //--- --- --- --- --- ParenthesesClose

    //--- --- --- QuerySelect
    assert_eq!(
        BqsqlDocumentItemType::QuerySelect,
        query_f.items[1].item_type
    );
    assert_eq!(None, query_f.items[1].range);
    assert_eq!(2, query_f.items[1].items.len());

    //--- --- --- --- Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectStar
    //--- --- --- QueryFrom
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_f.items[2].item_type);
    assert_eq!(None, query_f.items[2].range);
    assert_eq!(2, query_f.items[2].items.len());

    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName

    //--- --- ParenthesesClose
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_from.items[3].item_type
    );
    assert_eq!(Some([7, 16, 17]), query_from.items[3].range);
    assert_eq!(0, query_from.items[3].items.len());

    //--- Semicolon
    assert_eq!(BqsqlDocumentItemType::Semicolon, query.items[3].item_type);
    assert_eq!(Some([7, 17, 18]), query.items[3].range);
    assert_eq!(0, query.items[3].items.len());

    //
}
