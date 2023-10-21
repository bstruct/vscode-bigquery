use lazy_static::lazy_static;
use regex::Regex;

use super::{
    bqsql_delimiter::BqsqlDelimiter,
    bqsql_interpreter::{
        self, get_relevant_keywords_match, get_relevant_operators_all_match, get_string_in_range,
        handle_document_item, handle_semicolon, handle_unknown, is_delimiter, is_in_range,
        is_keyword, is_line_comment, is_number, is_string, is_string_identifier, BqsqlInterpreter,
    },
    bqsql_query_structure::BqsqlQueryStructure,
    BqsqlDocumentItem, BqsqlDocumentItemType, BqsqlKeyword,
};

impl BqsqlInterpreter<'_> {
    pub(crate) fn handle_query(&mut self) -> Option<BqsqlDocumentItem> {
        if is_query(self) {
            let document_item = BqsqlDocumentItem::new(
                BqsqlDocumentItemType::Query,
                vec![
                    handle_query_stage(self, BqsqlQueryStructure::With),
                    handle_query_stage(self, BqsqlQueryStructure::Select),
                    handle_query_stage(self, BqsqlQueryStructure::From),
                    handle_query_stage(self, BqsqlQueryStructure::Where),
                    handle_query_stage(self, BqsqlQueryStructure::GroupBy),
                    handle_query_stage(self, BqsqlQueryStructure::Rollup),
                    handle_query_stage(self, BqsqlQueryStructure::Having),
                    handle_query_stage(self, BqsqlQueryStructure::Qualify),
                    handle_query_stage(self, BqsqlQueryStructure::Window),
                    handle_query_stage(self, BqsqlQueryStructure::OrderBy),
                    handle_query_stage(self, BqsqlQueryStructure::Limit),
                    handle_query_stage(self, BqsqlQueryStructure::Offset),
                    handle_semicolon(self),
                ],
            );

            return Some(document_item);
        }
        None
    }
}

fn is_query(interpreter: &BqsqlInterpreter) -> bool {
    is_keyword(interpreter, interpreter.index, BqsqlKeyword::With)
        || is_keyword(interpreter, interpreter.index, BqsqlKeyword::Select)
}

fn handle_query_stage(
    interpreter: &mut BqsqlInterpreter,
    query_stage: BqsqlQueryStructure,
) -> Option<BqsqlDocumentItem> {
    match query_stage {
        BqsqlQueryStructure::With => {
            return handle_query_stage_default(
                interpreter,
                query_stage,
                document_item_handler_with,
            );
        }
        BqsqlQueryStructure::Select => {
            return handle_query_stage_select(interpreter, document_item_handler_select);
        }
        BqsqlQueryStructure::From => {
            return handle_query_stage_default(
                interpreter,
                query_stage,
                document_item_handler_from,
            );
        }
        _ => {
            return handle_query_stage_default(interpreter, query_stage, handle_unknown);
        }
    };
}

fn handle_query_stage_default(
    interpreter: &mut BqsqlInterpreter,
    query_stage: BqsqlQueryStructure,
    //https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    document_item_handler: fn(&mut BqsqlInterpreter) -> Option<BqsqlDocumentItem>,
) -> Option<BqsqlDocumentItem> {
    //get keywords associated with this query stage to
    // 1) confirm that they are found
    // 2) if they are found, they are added as keywords to the document_item items
    let keywords_to_match = &query_stage.get_keywords();
    if let Some(keywords_match) = bqsql_interpreter::get_relevant_keywords_match(
        interpreter,
        interpreter.index,
        keywords_to_match,
    ) {
        let mut items: Vec<Option<BqsqlDocumentItem>> =
            Vec::from(handle_query_keywords(interpreter, keywords_match));

        let subsequent_query_structure = &query_stage.get_subsequent_query_structure();

        //get items
        let new_items = &mut loop_query_default(
            interpreter,
            subsequent_query_structure,
            document_item_handler,
        );
        items.append(new_items);

        return Some(BqsqlDocumentItem::new(
            query_stage.get_document_item_type(),
            items,
        ));
    }
    None
}

fn handle_query_keywords(
    interpreter: &mut BqsqlInterpreter,
    keywords_match: Vec<BqsqlKeyword>,
) -> Vec<Option<BqsqlDocumentItem>> {
    let mut items: Vec<Option<BqsqlDocumentItem>> = Vec::new();

    //add keywords
    let mut matched = 0;
    while matched < keywords_match.len() {
        if is_line_comment(interpreter, interpreter.index) {
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::LineComment,
                None,
            ));
        } else {
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Keyword,
                Some(keywords_match[matched]),
            ));
            matched += 1;
        }
    }

    items
}

fn loop_query_default(
    interpreter: &mut BqsqlInterpreter,
    subsequent_query_structure: &Vec<BqsqlQueryStructure>,
    document_item_handler: fn(&mut BqsqlInterpreter) -> Option<BqsqlDocumentItem>,
) -> Vec<Option<BqsqlDocumentItem>> {
    let mut items: Vec<Option<BqsqlDocumentItem>> = Vec::new();

    //loop tokens inside the expected block of the query
    let mut count_open_parentheses: usize = 0;
    let mut count_open_square_brackets: usize = 0;
    while continue_loop_query(
        interpreter,
        subsequent_query_structure,
        count_open_parentheses + count_open_square_brackets,
    ) {
        if is_delimiter(
            interpreter,
            interpreter.index,
            BqsqlDelimiter::ParenthesesOpen,
        ) {
            //parentheses open
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::ParenthesesOpen,
                None,
            ));
            count_open_parentheses += 1;
            continue;
        } else if is_delimiter(
            interpreter,
            interpreter.index,
            BqsqlDelimiter::ParenthesesClose,
        ) {
            //parentheses close
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::ParenthesesClose,
                None,
            ));
            count_open_parentheses = std::cmp::max(0, count_open_parentheses - 1);
            continue;
        } else if is_delimiter(
            interpreter,
            interpreter.index,
            BqsqlDelimiter::SquareBracketsOpen,
        ) {
            //parentheses open
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::SquareBracketsOpen,
                None,
            ));
            count_open_square_brackets += 1;
            continue;
        } else if is_delimiter(
            interpreter,
            interpreter.index,
            BqsqlDelimiter::SquareBracketsClose,
        ) {
            //parentheses close
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::SquareBracketsClose,
                None,
            ));
            count_open_square_brackets = std::cmp::max(0, count_open_square_brackets - 1);
            continue;
        } else if is_delimiter(interpreter, interpreter.index, BqsqlDelimiter::Comma) {
            //comma
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Comma,
                None,
            ));
            continue;
        } else if is_query(interpreter) {
            items.push(interpreter.handle_query());
            continue;
        } else if is_number(interpreter) {
            //number
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Number,
                None,
            ));
            continue;
        } else if is_string(interpreter, interpreter.index) {
            //string
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::String,
                None,
            ));
            continue;
        } else if is_keyword(interpreter, interpreter.index, BqsqlKeyword::As) {
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Keyword,
                Some(BqsqlKeyword::As),
            ));
            continue;
        } else if is_line_comment(interpreter, interpreter.index) {
            //line comment
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::LineComment,
                None,
            ));
            continue;
        } else if let Some(operator) = get_relevant_operators_all_match(interpreter) {
            let mut index: usize = 0;
            let len = operator.to_vec().len();
            while index < len {
                items.push(handle_document_item(
                    interpreter,
                    BqsqlDocumentItemType::Operator,
                    None,
                ));
                index += 1;
            }
            continue;
        }

        items.push(document_item_handler(interpreter));
    }

    items
}

fn document_item_handler_select(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    if is_keyword(interpreter, interpreter.index - 1, BqsqlKeyword::As) {
        return handle_document_item(interpreter, BqsqlDocumentItemType::Alias, None);
    }

    handle_unknown(interpreter)
}

fn document_item_handler_with(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    if is_keyword(interpreter, interpreter.index - 1, BqsqlKeyword::With)
        || is_delimiter(interpreter, interpreter.index - 1, BqsqlDelimiter::Comma)
    {
        return handle_document_item(interpreter, BqsqlDocumentItemType::TableCteId, None);
    }
    if is_keyword(interpreter, interpreter.index, BqsqlKeyword::As) {
        return handle_document_item(
            interpreter,
            BqsqlDocumentItemType::Keyword,
            Some(BqsqlKeyword::As),
        );
    }
    if is_keyword(interpreter, interpreter.index - 1, BqsqlKeyword::As) {
        return handle_document_item(interpreter, BqsqlDocumentItemType::Alias, None);
    }

    handle_unknown(interpreter)
}

fn document_item_handler_from(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\w+").unwrap();
    }

    if is_keyword(interpreter, interpreter.index - 1, BqsqlKeyword::From) {
        let mut items: Vec<Option<BqsqlDocumentItem>> = Vec::new();

        let mut count_positions: usize = 0;
        let mut index_positions: usize = interpreter.index;
        loop {
            if is_string_identifier(interpreter, interpreter.index) {
                count_positions += 1;
            } else if let Some(string_in_range) =
                get_string_in_range(interpreter, interpreter.index)
            {
                if RE.is_match(string_in_range) {
                    count_positions += 1;
                }
            }

            if !is_delimiter(interpreter, index_positions + 1, BqsqlDelimiter::Dot) {
                break;
            }
            index_positions += 2;
        }

        //vector to store the different identifier positions
        // 1 value = either is string identifier or must be a CTE name
        // 2 values = if contains string identifier can be `project_id.dataset_id`.table_id or `dataset_id`.table_id
        //              if not contains string, must be dataset_id.table_id
        // 3 values = must be project_id.dataset_id.table_id
        if count_positions == 1 {
            if is_string_identifier(interpreter, interpreter.index) {
                if let Some(string_in_range) = get_string_in_range(interpreter, interpreter.index) {
                    let char_count = string_in_range.chars().filter(|c| c == &'.').count();

                    match char_count {
                        1 => {
                            items.push(handle_document_item(
                                interpreter,
                                BqsqlDocumentItemType::TableIdentifierDatasetIdTableId,
                                None,
                            ));
                        }
                        2 => {
                            items.push(handle_document_item(
                                interpreter,
                                BqsqlDocumentItemType::TableIdentifierProjectIdDatasetIdTableId,
                                None,
                            ));
                        }
                        _ => {
                            return handle_unknown(interpreter);
                        }
                    }
                }
            } else {
                items.push(handle_document_item(
                    interpreter,
                    BqsqlDocumentItemType::TableCteId,
                    None,
                ));
            }
        } else if count_positions == 2 {
            if is_string_identifier(interpreter, interpreter.index) {
                if let Some(string_in_range) = get_string_in_range(interpreter, interpreter.index) {
                    if string_in_range.contains(".") {
                        items.push(handle_document_item(
                            interpreter,
                            BqsqlDocumentItemType::TableIdentifierProjectIdDatasetId,
                            None,
                        ));
                    }
                }
            } else {
                items.push(handle_document_item(
                    interpreter,
                    BqsqlDocumentItemType::TableIdentifierProjectId,
                    None,
                ));
            }

            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Dot,
                None,
            ));
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::TableIdentifierTableId,
                None,
            ));
        } else if count_positions == 3 {
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::TableIdentifierProjectId,
                None,
            ));
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Dot,
                None,
            ));
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::TableIdentifierDatasetId,
                None,
            ));
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Dot,
                None,
            ));
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::TableIdentifierTableId,
                None,
            ));
        } else {
            return handle_unknown(interpreter);
        }

        if is_keyword(interpreter, interpreter.index, BqsqlKeyword::As) {
            items.push(handle_document_item(
                interpreter,
                BqsqlDocumentItemType::Keyword,
                Some(BqsqlKeyword::As),
            ));
        }

        if is_in_range(interpreter, interpreter.index) {
            if !is_inner_from_expected_keyword(interpreter, interpreter.index) {
                let subsequent_query_structure: &Vec<BqsqlQueryStructure> =
                    &BqsqlQueryStructure::From.get_subsequent_query_structure();

                if !is_subsequent_query_structure(
                    interpreter,
                    interpreter.index + 1,
                    subsequent_query_structure,
                ) {
                    if let Some(string_in_range) =
                        get_string_in_range(interpreter, interpreter.index)
                    {
                        if RE.is_match(string_in_range) {
                            items.push(handle_document_item(
                                interpreter,
                                BqsqlDocumentItemType::TableIdentifierAlias,
                                None,
                            ));
                        }
                    }
                }
            }
        }

        if items.len() > 0 {
            return Some(BqsqlDocumentItem::new(
                BqsqlDocumentItemType::TableIdentifier,
                items,
            ));
        }
    }

    handle_unknown(interpreter)
}

fn is_inner_from_expected_keyword(interpreter: &BqsqlInterpreter, index: usize) -> bool {
    is_keyword(interpreter, index, BqsqlKeyword::For)
        || is_keyword(interpreter, index, BqsqlKeyword::Unnest)
        || is_keyword(interpreter, index, BqsqlKeyword::Join)
        || is_keyword(interpreter, index, BqsqlKeyword::Inner)
        || is_keyword(interpreter, index, BqsqlKeyword::Cross)
        || is_keyword(interpreter, index, BqsqlKeyword::Full)
        || is_keyword(interpreter, index, BqsqlKeyword::Left)
        || is_keyword(interpreter, index, BqsqlKeyword::Right)
        || is_keyword(interpreter, index, BqsqlKeyword::Pivot)
        || is_keyword(interpreter, index, BqsqlKeyword::Unpivot)
        || is_keyword(interpreter, index, BqsqlKeyword::Tablesample)
        || is_keyword(interpreter, index, BqsqlKeyword::Using)
}

fn handle_query_stage_select(
    interpreter: &mut BqsqlInterpreter,
    //https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    document_item_handler: fn(&mut BqsqlInterpreter) -> Option<BqsqlDocumentItem>,
) -> Option<BqsqlDocumentItem> {
    //get keywords associated with this query stage to
    // 1) confirm that they are found
    // 2) if they are found, they are added as keywords to the document_item items
    let keywords_to_match = &BqsqlQueryStructure::Select.get_keywords();
    if let Some(keywords_match) = bqsql_interpreter::get_relevant_keywords_match(
        interpreter,
        interpreter.index,
        keywords_to_match,
    ) {
        let mut items: Vec<Option<BqsqlDocumentItem>> =
            Vec::from(handle_query_keywords(interpreter, keywords_match));

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        //get items
        let new_items = loop_query_default(
            interpreter,
            subsequent_query_structure,
            document_item_handler,
        );
        // items.append(new_items);

        //break new_items per significant (not between brackets) comma
        for list_items in handle_query_stage_select_split_list_items(&new_items) {
            if list_items.len() > 0 {
                let i = BqsqlDocumentItem::new(
                    BqsqlDocumentItemType::QuerySelectListItem,
                    list_items.to_vec(),
                );

                items.push(Some(i));
            }
        }

        return Some(BqsqlDocumentItem::new(
            BqsqlDocumentItemType::QuerySelect,
            items,
        ));
    }
    None
}

fn handle_query_stage_select_split_list_items<'a>(
    items: &'a Vec<Option<BqsqlDocumentItem>>,
) -> Vec<&'a [Option<BqsqlDocumentItem>]> {
    let mut items_to_return = Vec::new();

    let mut last_index: usize = 0;
    let mut index: usize = 0;
    let mut parentheses_open: usize = 0;
    let mut square_brackets_open: usize = 0;
    while index < items.len() {
        if let Some(item) = &items[index] {
            match item.item_type {
                BqsqlDocumentItemType::ParenthesesOpen => parentheses_open += 1,
                BqsqlDocumentItemType::ParenthesesClose => parentheses_open -= 1,
                BqsqlDocumentItemType::SquareBracketsOpen => square_brackets_open += 1,
                BqsqlDocumentItemType::SquareBracketsClose => square_brackets_open -= 1,
                BqsqlDocumentItemType::Comma => {
                    if parentheses_open + square_brackets_open == 0 {
                        index += 1;
                        items_to_return.push(&items[last_index..index]);
                        last_index = index;
                    }
                }
                _ => {}
            }
        }

        index += 1;
    }

    if last_index == 0 || last_index < index {
        items_to_return.push(&items[last_index..]);
    }

    items_to_return
}

#[cfg(test)]
mod tests_handle_query_stage_select_split_list_items {
    use crate::bqsql_document::{
        bqsql_interpreter::{handle_unknown, BqsqlInterpreter},
        bqsql_interpreter_query::{handle_query_stage_select_split_list_items, loop_query_default},
        bqsql_query_structure::BqsqlQueryStructure,
    };

    #[test]
    fn handle_query_stage_select_split_list_1_items_no_delimiters() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM t");
        interpreter.index = 1;

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        let items =
            &loop_query_default(&mut interpreter, subsequent_query_structure, handle_unknown);

        let split = handle_query_stage_select_split_list_items(items);

        assert_eq!(1, split.len());
        assert_eq!(1, split[0].len());
    }

    #[test]
    fn handle_query_stage_select_split_list_1_items_delimiters() {
        let mut interpreter = BqsqlInterpreter::new("SELECT (1) FROM t");
        interpreter.index = 1;

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        let items =
            &loop_query_default(&mut interpreter, subsequent_query_structure, handle_unknown);

        let split = handle_query_stage_select_split_list_items(items);

        assert_eq!(1, split.len());
        assert_eq!(3, split[0].len());
    }

    #[test]
    fn handle_query_stage_select_split_list_5_items_no_delimiters() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1,2,3,4,5+1 FROM t");
        interpreter.index = 1;

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        let items =
            &loop_query_default(&mut interpreter, subsequent_query_structure, handle_unknown);

        let split = handle_query_stage_select_split_list_items(items);

        assert_eq!(5, split.len());
        assert_eq!(2, split[0].len());
        assert_eq!(2, split[1].len());
        assert_eq!(2, split[2].len());
        assert_eq!(2, split[3].len());
        assert_eq!(3, split[4].len());
    }

    #[test]
    fn handle_query_stage_select_split_list_3_items_delimiters() {
        let mut interpreter = BqsqlInterpreter::new("SELECT (1,2,3),4,5+1 FROM t");

        assert_eq!(16, interpreter.tokens.len());

        interpreter.index = 1;

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        let items =
            &loop_query_default(&mut interpreter, subsequent_query_structure, handle_unknown);

        assert_eq!(13, items.len());

        let split = handle_query_stage_select_split_list_items(items);

        assert_eq!(3, split.len());
        assert_eq!(8, split[0].len());
        assert_eq!(2, split[1].len());
        assert_eq!(3, split[2].len());
    }

    #[test]
    fn handle_query_stage_select_split_list_3_items_delimiters_square() {
        let mut interpreter = BqsqlInterpreter::new("SELECT [1,2,3],4,5+1 FROM t");

        assert_eq!(16, interpreter.tokens.len());

        interpreter.index = 1;

        let subsequent_query_structure =
            &BqsqlQueryStructure::Select.get_subsequent_query_structure();

        let items =
            &loop_query_default(&mut interpreter, subsequent_query_structure, handle_unknown);

        assert_eq!(13, items.len());

        let split = handle_query_stage_select_split_list_items(items);

        assert_eq!(3, split.len());
        assert_eq!(8, split[0].len());
        assert_eq!(2, split[1].len());
        assert_eq!(3, split[2].len());
    }
}

fn continue_loop_query(
    interpreter: &BqsqlInterpreter,
    subsequent_query_structure: &Vec<BqsqlQueryStructure>,
    count_open_parentheses: usize,
) -> bool {
    if is_in_range(interpreter, interpreter.index) {
        //;
        if is_delimiter(interpreter, interpreter.index, BqsqlDelimiter::Semicolon) {
            return false;
        }

        //parentheses close
        if is_delimiter(
            interpreter,
            interpreter.index,
            BqsqlDelimiter::ParenthesesClose,
        ) {
            if count_open_parentheses == 0 {
                return false;
            }
        }

        if count_open_parentheses > 0 {
            return true;
        }

        //are any of the subsequent keywords of the query found?
        return is_subsequent_query_structure(
            interpreter,
            interpreter.index,
            subsequent_query_structure,
        );
    }

    false
}

fn is_subsequent_query_structure(
    interpreter: &BqsqlInterpreter,
    index: usize,
    subsequent_query_structure: &Vec<BqsqlQueryStructure>,
) -> bool {
    return !subsequent_query_structure
        .iter()
        .map(|i| i.get_keywords())
        .any(|i| get_relevant_keywords_match(&interpreter, index, &i).is_some());
}

#[cfg(test)]
mod tests_continue_loop_query {
    use crate::bqsql_document::{
        bqsql_interpreter::BqsqlInterpreter, bqsql_interpreter_query::continue_loop_query,
        bqsql_query_structure::BqsqlQueryStructure,
    };

    #[test]
    fn continue_loop_query_from_where_not_continue() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM t WHERE 1=1");
        interpreter.index = 4;
        let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

        assert!(!continue_loop_query(
            &interpreter,
            &query_stage.get_subsequent_query_structure(),
            0
        ));
    }

    #[test]
    fn continue_loop_query_from_where_continue() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM dataset.table WHERE 1=1");
        interpreter.index = 4;
        let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

        assert!(continue_loop_query(
            &interpreter,
            &query_stage.get_subsequent_query_structure(),
            0
        ));
    }

    #[test]
    fn continue_loop_query_from_end() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM dataset.table");
        interpreter.index = 6;
        let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

        assert!(!continue_loop_query(
            &interpreter,
            &query_stage.get_subsequent_query_structure(),
            0
        ));
    }

    #[test]
    fn continue_loop_query_with() {
        let mut interpreter = BqsqlInterpreter::new(
            r#"WITH q1 AS (SELECT SchoolID FROM Roster) #my_query
SELECT *
FROM
(WITH q2 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q3 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q1 AS (SELECT * FROM q1),  # q1 (in the query) resolves to my_query
    q4 AS (SELECT * FROM q1)   # q1 resolves to the WITH subquery on the previous line.
SELECT * FROM q1);             # q1 resolves to the third inner WITH subquery."#,
        );

        interpreter.index = 9;
        let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::With;

        assert!(continue_loop_query(
            &interpreter,
            &query_stage.get_subsequent_query_structure(),
            0
        ));
    }
}
