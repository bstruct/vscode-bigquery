use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType, BqsqlKeyword};

#[test]
fn empty_string() {
    let document = BqsqlDocument::parse("");

    assert_eq!(0, document.items.len());
}

#[test]
fn comment_only() {
    let document = BqsqlDocument::parse("--super comment");

    assert_eq!(1, document.items.len());

    let comment = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::LineComment, comment.item_type);
    assert_eq!(Some([0, 0, 15]), comment.range);
    assert_eq!(0, comment.items.len());
}

#[test]
fn comment_with_select_in_text() {
    let document = BqsqlDocument::parse("--super comment that includes a query SELECT 2+2");

    assert_eq!(1, document.items.len());

    let comment = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::LineComment, comment.item_type);
    assert_eq!(Some([0, 0, 48]), comment.range);
    assert_eq!(0, comment.items.len());
}

#[test]
fn space_comment_only() {
    let document = BqsqlDocument::parse("    --super comment");

    assert_eq!(1, document.items.len());

    let comment = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::LineComment, comment.item_type);
    assert_eq!(Some([0, 4, 19]), comment.range);
    assert_eq!(0, comment.items.len());
}

#[test]
fn tiny_query() {
    let document = BqsqlDocument::parse("SELECT 2+2");

    //
    //Query
    //--- QuerySelect
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- NUMBER
    //--- --- --- OPERATOR
    //--- --- --- NUMBER
    //

    assert_eq!(1, document.items.len());

    //
    //QUERY
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[0].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([0, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- ---
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(3, query_list_item_0.items.len());

    let query_list_item_0_items = &query_list_item_0.items;
    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[0].item_type
    );
    assert_eq!(Some([0, 7, 8]), query_list_item_0_items[0].range);
    assert_eq!(0, query_list_item_0_items[0].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Operator,
        query_list_item_0_items[1].item_type
    );
    assert_eq!(Some([0, 8, 9]), query_list_item_0_items[1].range);
    assert_eq!(0, query_list_item_0_items[1].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[2].item_type
    );
    assert_eq!(Some([0, 9, 10]), query_list_item_0_items[2].range);
    assert_eq!(0, query_list_item_0_items[2].items.len());
}

#[test]
fn tiny_query_second_line() {
    let document = BqsqlDocument::parse("\nSELECT 2+2");

    //
    //Query
    //--- QuerySelect
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- NUMBER
    //--- --- --- OPERATOR
    //--- --- --- NUMBER
    //

    assert_eq!(1, document.items.len());

    //
    //QUERY
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[0].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([1, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- --- QUERY_SELECT_LIST_ITEM
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(3, query_list_item_0.items.len());

    let query_list_item_0_items = &query_list_item_0.items;
    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[0].item_type
    );
    assert_eq!(Some([1, 7, 8]), query_list_item_0_items[0].range);
    assert_eq!(0, query_list_item_0_items[0].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Operator,
        query_list_item_0_items[1].item_type
    );
    assert_eq!(Some([1, 8, 9]), query_list_item_0_items[1].range);
    assert_eq!(0, query_list_item_0_items[1].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[2].item_type
    );
    assert_eq!(Some([1, 9, 10]), query_list_item_0_items[2].range);
    assert_eq!(0, query_list_item_0_items[2].items.len());
}

#[test]
fn comment_and_tiny_query() {
    let document = BqsqlDocument::parse("--super comment\nSELECT 2+2");

    //
    //Query
    //--- QuerySelect
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- NUMBER
    //--- --- --- OPERATOR
    //--- --- --- NUMBER
    //

    assert_eq!(2, document.items.len());

    //comment
    let comment = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::LineComment, comment.item_type);
    assert_eq!(Some([0, 0, 15]), comment.range);
    assert_eq!(0, comment.items.len());

    //
    //QUERY
    let query = &document.items[1];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[1].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([1, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- --- QUERY_SELECT_LIST_ITEM
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(3, query_list_item_0.items.len());

    let query_list_item_0_items = &query_list_item_0.items;
    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[0].item_type
    );
    assert_eq!(Some([1, 7, 8]), query_list_item_0_items[0].range);
    assert_eq!(0, query_list_item_0_items[0].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Operator,
        query_list_item_0_items[1].item_type
    );
    assert_eq!(Some([1, 8, 9]), query_list_item_0_items[1].range);
    assert_eq!(0, query_list_item_0_items[1].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[2].item_type
    );
    assert_eq!(Some([1, 9, 10]), query_list_item_0_items[2].range);
    assert_eq!(0, query_list_item_0_items[2].items.len());
}

#[test]
fn select_select_as_struct_query() {
    let document =
        BqsqlDocument::parse("SELECT (SELECT AS STRUCT 2+2 AS asas, 'ASDASD' AS qweqwe) AS XXX");

    //
    //QUERY
    //--- QUERY_SELECT
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- PARENTHESES_OPEN
    //--- --- --- QUERY
    //--- --- --- --- QUERY_SELECT_AS_STRUCT
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- OPERATOR
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS
    //--- --- --- --- --- --- Comma
    //--- --- --- --- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- --- --- --- STRING
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS
    //--- --- --- PARENTHESES_CLOSE
    //--- --- --- AS_ALIAS
    //--- --- --- ALIAS
    //

    assert_eq!(1, document.items.len());

    //
    //QUERY
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[0].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([0, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- --- QUERY_SELECT_LIST_ITEM
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(5, query_list_item_0.items.len());

    //--- --- --- PARENTHESES_OPEN
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_list_item_0.items[0].item_type
    );
    assert_eq!(Some([0, 7, 8]), query_list_item_0.items[0].range);
    assert_eq!(0, query_list_item_0.items[0].items.len());

    //--- --- --- QUERY
    assert_eq!(
        BqsqlDocumentItemType::Query,
        query_list_item_0.items[1].item_type
    );
    assert_eq!(None, query_list_item_0.items[1].range);
    assert_eq!(1, query_list_item_0.items[1].items.len());

    let select_as_struct = &query_list_item_0.items[1].items[0];

    //--- --- --- --- QUERY_SELECT_AS_STRUCT
    assert_eq!(
        BqsqlDocumentItemType::QuerySelect,
        select_as_struct.item_type
    );
    assert_eq!(None, select_as_struct.range);
    assert_eq!(5, select_as_struct.items.len());

    //--- --- --- --- --- KEYWORD
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        select_as_struct.items[0].item_type
    );
    assert_eq!(Some([0, 8, 14]), select_as_struct.items[0].range);
    assert_eq!(0, select_as_struct.items[0].items.len());

    //--- --- --- --- --- KEYWORD
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        select_as_struct.items[1].item_type
    );
    assert_eq!(Some([0, 15, 17]), select_as_struct.items[1].range);
    assert_eq!(0, select_as_struct.items[1].items.len());

    //--- --- --- --- --- KEYWORD
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        select_as_struct.items[2].item_type
    );
    assert_eq!(Some([0, 18, 24]), select_as_struct.items[2].range);
    assert_eq!(0, select_as_struct.items[2].items.len());

    //--- --- --- --- --- QUERY_SELECT_SELECT_LIST_ITEM
    let list_item_0 = &select_as_struct.items[3];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        list_item_0.item_type
    );
    assert_eq!(None, list_item_0.range);
    assert_eq!(6, list_item_0.items.len());

    //--- --- --- --- --- --- NUMBER
    assert_eq!(
        BqsqlDocumentItemType::Number,
        list_item_0.items[0].item_type
    );
    assert_eq!(Some([0, 25, 26]), list_item_0.items[0].range);
    assert_eq!(0, list_item_0.items[0].items.len());

    //--- --- --- --- --- --- OPERATOR
    assert_eq!(
        BqsqlDocumentItemType::Operator,
        list_item_0.items[1].item_type
    );
    assert_eq!(Some([0, 26, 27]), list_item_0.items[1].range);
    assert_eq!(0, list_item_0.items[1].items.len());

    //--- --- --- --- --- --- NUMBER
    assert_eq!(
        BqsqlDocumentItemType::Number,
        list_item_0.items[2].item_type
    );
    assert_eq!(Some([0, 27, 28]), list_item_0.items[2].range);
    assert_eq!(0, list_item_0.items[2].items.len());

    //--- --- --- --- --- --- AS_ALIAS
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        list_item_0.items[3].item_type
    );
    assert_eq!(Some([0, 29, 31]), list_item_0.items[3].range);
    assert_eq!(0, list_item_0.items[3].items.len());
    assert_eq!(Some(BqsqlKeyword::As), list_item_0.items[3].keyword);

    //--- --- --- --- --- --- ALIAS
    assert_eq!(BqsqlDocumentItemType::Alias, list_item_0.items[4].item_type);
    assert_eq!(Some([0, 32, 36]), list_item_0.items[4].range);
    assert_eq!(0, list_item_0.items[4].items.len());
    assert_eq!(None, list_item_0.items[4].keyword);

    //--- --- --- --- --- --- Comma
    assert_eq!(BqsqlDocumentItemType::Comma, list_item_0.items[5].item_type);
    assert_eq!(Some([0, 36, 37]), list_item_0.items[5].range);
    assert_eq!(0, list_item_0.items[5].items.len());

    //--- --- --- --- --- QUERY_SELECT_SELECT_LIST_ITEM
    let list_item_1 = &select_as_struct.items[4];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        list_item_1.item_type
    );
    assert_eq!(None, list_item_1.range);
    assert_eq!(3, list_item_1.items.len());

    //--- --- --- --- --- --- STRING
    assert_eq!(
        BqsqlDocumentItemType::String,
        list_item_1.items[0].item_type
    );
    assert_eq!(Some([0, 38, 46]), list_item_1.items[0].range);
    assert_eq!(0, list_item_1.items[0].items.len());

    //--- --- --- --- --- --- KeywordAs
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        list_item_1.items[1].item_type
    );
    assert_eq!(Some([0, 47, 49]), list_item_1.items[1].range);
    assert_eq!(0, list_item_1.items[1].items.len());
    assert_eq!(Some(BqsqlKeyword::As), list_item_1.items[1].keyword);

    //--- --- --- --- --- --- ALIAS
    assert_eq!(BqsqlDocumentItemType::Alias, list_item_1.items[2].item_type);
    assert_eq!(Some([0, 50, 56]), list_item_1.items[2].range);
    assert_eq!(0, list_item_1.items[2].items.len());

    //--- --- --- PARENTHESES_CLOSE
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_list_item_0.items[2].item_type
    );
    assert_eq!(Some([0, 56, 57]), query_list_item_0.items[2].range);
    assert_eq!(0, query_list_item_0.items[2].items.len());

    //--- --- --- AS_ALIAS
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_list_item_0.items[3].item_type
    );
    assert_eq!(Some([0, 58, 60]), query_list_item_0.items[3].range);
    assert_eq!(0, query_list_item_0.items[3].items.len());
    assert_eq!(Some(BqsqlKeyword::As), query_list_item_0.items[3].keyword);

    //--- --- --- ALIAS
    assert_eq!(
        BqsqlDocumentItemType::Alias,
        query_list_item_0.items[4].item_type
    );
    assert_eq!(Some([0, 61, 64]), query_list_item_0.items[4].range);
    assert_eq!(0, query_list_item_0.items[4].items.len());

    //
    //
}


#[test]
fn query_select_trailing_comma(){
    let document = BqsqlDocument::parse(
        r#"SELECT column_a, column_a, column_c, FROM dataset_id.table_id"#,
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
    assert_eq!(BqsqlDocumentItemType::Keyword, query_select.items[0].item_type);
    assert_eq!(BqsqlDocumentItemType::QuerySelectListItem, query_select.items[1].item_type);
    assert_eq!(BqsqlDocumentItemType::QuerySelectListItem, query_select.items[2].item_type);
    assert_eq!(BqsqlDocumentItemType::QuerySelectListItem, query_select.items[3].item_type);

}
