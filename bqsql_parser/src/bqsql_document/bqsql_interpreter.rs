use lazy_static::lazy_static;
use regex::Regex;

use super::{
    bqsql_delimiter::BqsqlDelimiter, bqsql_operator::BqsqlOperator, BqsqlDocumentItem,
    BqsqlDocumentItemType, BqsqlKeyword,
};
use crate::bqsql_document::token_parser;

pub(crate) struct BqsqlInterpreter<'a> {
    pub(crate) lines: Vec<&'a str>,
    pub(crate) tokens: Vec<[usize; 3]>,
    pub(crate) index: usize,
}

impl BqsqlInterpreter<'_> {
    pub(crate) fn new(bqsql: &str) -> BqsqlInterpreter {
        let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = token_parser::parse_tokens(bqsql);

        BqsqlInterpreter {
            lines: lines,
            tokens: tokens,
            index: 0,
        }
    }

    pub(crate) fn collect(&mut self) -> Vec<BqsqlDocumentItem> {
        let mut monitor_index = self.index;
        let mut items: Vec<BqsqlDocumentItem> = Vec::new();

        while self.tokens.len() > self.index {
            if is_line_comment(self, self.index) {
                items.push(
                    handle_document_item(self, BqsqlDocumentItemType::LineComment, None).unwrap(),
                );
            }

            if let Some(query) = self.handle_query() {
                items.push(query);
            }

            if monitor_index == self.index {
                if let Some(unknown) = handle_unknown(self) {
                    items.push(unknown);
                }
            } else {
                monitor_index = self.index;
            }

            self.index += 1;
        }

        items
    }
}

pub(crate) fn is_keyword(
    interpreter: &BqsqlInterpreter,
    index: usize,
    keyword: BqsqlKeyword,
) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, index) {
        return string_in_range == keyword;
    }
    false
}

pub(crate) fn is_line_comment(interpreter: &BqsqlInterpreter, index: usize) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, index) {
        return string_in_range.starts_with("--") || string_in_range.starts_with("#");
    }
    false
}

pub(crate) fn is_in_range(interpreter: &BqsqlInterpreter, index: usize) -> bool {
    interpreter.tokens.len() > index
}

pub(crate) fn is_delimiter(
    interpreter: &BqsqlInterpreter,
    index: usize,
    delimiter: BqsqlDelimiter,
) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, index) {
        if string_in_range == delimiter {
            return string_in_range == delimiter;
        }
    }
    false
}

pub(crate) fn is_string(interpreter: &BqsqlInterpreter, index: usize) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, index) {
        return string_in_range.starts_with("'") || string_in_range.starts_with("\"");
    }
    false
}

pub(crate) fn is_string_identifier(interpreter: &BqsqlInterpreter, index: usize) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, index) {
        return string_in_range.starts_with("`");
    }
    false
}

pub(crate) fn get_string_in_range<'a>(
    interpreter: &'a BqsqlInterpreter,
    index: usize,
) -> Option<&'a str> {
    if is_in_range(interpreter, index) {
        let range = &interpreter.tokens[index];
        if interpreter.lines[range[0]].len() >= range[2] {
            return Some(&interpreter.lines[range[0]][range[1]..range[2]]);
        }
    }
    None
}

/*most generic verstion of the handle_
if return a BqsqlDocumentItem, moves the index by 1 */
pub(crate) fn handle_document_item(
    interpreter: &mut BqsqlInterpreter,
    item_type: BqsqlDocumentItemType,
    keyword: Option<BqsqlKeyword>,
) -> Option<BqsqlDocumentItem> {
    if is_in_range(interpreter, interpreter.index) {
        let item = BqsqlDocumentItem {
            item_type: item_type,
            range: Some(interpreter.tokens[interpreter.index]),
            items: vec![],
            keyword: keyword,
        };

        interpreter.index += 1;

        return Some(item);
    }
    None
}

pub(crate) fn handle_unknown(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    if is_in_range(interpreter, interpreter.index) {
        let item = BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::Unknown,
            range: Some(interpreter.tokens[interpreter.index]),
            items: vec![],
            keyword: None,
        };

        interpreter.index += 1;

        return Some(item);
    }

    None
}

pub(crate) fn is_number(interpreter: &BqsqlInterpreter) -> bool {
    if let Some(string_in_range) = get_string_in_range(interpreter, interpreter.index) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d+$|^\d*\.{1}\d*$").unwrap();
        }

        return RE.is_match(string_in_range);
    }
    false
}

#[cfg(test)]
mod tests_is_number {
    use crate::bqsql_document::bqsql_interpreter::{is_number, BqsqlInterpreter};

    #[test]
    fn is_number_q1() {
        let mut interpreter = BqsqlInterpreter::new("SELECT q1");
        interpreter.index = 1;
        assert_eq!(&2, &interpreter.tokens.len());
        assert!(!is_number(&interpreter));
    }

    #[test]
    fn is_number_1() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1");
        interpreter.index = 1;
        assert_eq!(&2, &interpreter.tokens.len());
        assert!(is_number(&interpreter));
    }

    #[test]
    fn is_number_1_112() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1.112");
        interpreter.index = 1;
        assert_eq!(&2, &interpreter.tokens.len());
        assert!(is_number(&interpreter));
    }

    #[test]
    fn is_number_1_() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 1.");
        interpreter.index = 1;
        assert_eq!(&2, &interpreter.tokens.len());
        assert!(is_number(&interpreter));
    }

    #[test]
    fn is_number_dot_112() {
        let mut interpreter = BqsqlInterpreter::new("SELECT .112");
        interpreter.index = 1;
        assert_eq!(&2, &interpreter.tokens.len());
        assert!(is_number(&interpreter));
    }

    #[test]
    fn is_number_dot_after_operator() {
        let mut interpreter = BqsqlInterpreter::new("SELECT 2+2");
        interpreter.index = 3;
        assert_eq!(&4, &interpreter.tokens.len());
        assert!(is_number(&interpreter));
    }
}

/*
try to match a sequence of keywords
the "relevant" part of the name, means that line comments in the middle will be ignored and match will still be possible
comments in the beginning will not match
 */
pub(crate) fn get_relevant_keywords_match(
    interpreter: &BqsqlInterpreter,
    index: usize,
    keywords_to_match: &Vec<Vec<BqsqlKeyword>>,
) -> Option<Vec<BqsqlKeyword>> {
    //do not accept comments in the beginning
    if is_line_comment(interpreter, index) {
        return None;
    }

    for keywords in keywords_to_match {
        let mut matched = 0;
        let mut i = index;

        while is_in_range(interpreter, i) && matched < keywords.len() {
            let keyword = keywords[matched];

            if is_line_comment(interpreter, i) {
                i += 1;
            } else {
                if is_keyword(interpreter, i, keyword) {
                    i += 1;
                    matched += 1;
                } else {
                    break;
                }
            }
            if matched >= keywords.len() {
                return Some(keywords.to_owned());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests_get_relevant_keywords_match {
    use crate::bqsql_document::{
        bqsql_interpreter::{get_relevant_keywords_match, BqsqlInterpreter},
        bqsql_query_structure::BqsqlQueryStructure,
        BqsqlKeyword,
    };

    #[test]
    fn get_relevant_keywords_match_select_short() {
        let interpreter = BqsqlInterpreter::new("SELECT 1");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter,interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(1, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
    }

    #[test]
    fn get_relevant_keywords_match_select() {
        let interpreter = BqsqlInterpreter::new("SELECT 1,2,3");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(1, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
    }

    #[test]
    fn get_relevant_keywords_match_select_all() {
        let interpreter = BqsqlInterpreter::new("SELECT ALL 1,2,3");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(2, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
        assert_eq!(BqsqlKeyword::All, keywords[1]);
    }

    #[test]
    fn get_relevant_keywords_match_select_distinct() {
        let interpreter = BqsqlInterpreter::new("SELECT\n DISTINCT 1,2,3");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(2, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
        assert_eq!(BqsqlKeyword::Distinct, keywords[1]);
    }

    #[test]
    fn get_relevant_keywords_match_select_as_struct() {
        let interpreter = BqsqlInterpreter::new("SELECT AS STRUCT 1,2,3");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(3, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
        assert_eq!(BqsqlKeyword::As, keywords[1]);
        assert_eq!(BqsqlKeyword::Struct, keywords[2]);
    }

    #[test]
    fn get_relevant_keywords_match_select_as_value() {
        let interpreter = BqsqlInterpreter::new("SELECT AS\n--jsafkljsafd\n VALUE 1,2,3");
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(3, keywords.len());
        assert_eq!(BqsqlKeyword::Select, keywords[0]);
        assert_eq!(BqsqlKeyword::As, keywords[1]);
        assert_eq!(BqsqlKeyword::Value, keywords[2]);
    }

    #[test]
    fn get_relevant_keywords_match_from() {
        let mut interpreter =
            BqsqlInterpreter::new("SELECT AS\n--jsafkljsafd\n VALUE 1,2,3 FROM a");
        interpreter.index = 9;
        let keywords_to_match = BqsqlQueryStructure::From.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(1, keywords.len());
        assert_eq!(BqsqlKeyword::From, keywords[0]);
    }

    #[test]
    fn get_relevant_keywords_match_where() {
        let mut interpreter =
            BqsqlInterpreter::new("SELECT AS\n#jsa fkl jsafd\n VALUE 1,2,3 FROM a WHERE 1=1");
        interpreter.index = 11;
        let keywords_to_match = BqsqlQueryStructure::Where.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_some());
        let keywords = keywords_option.unwrap();
        assert_eq!(1, keywords.len());
        assert_eq!(BqsqlKeyword::Where, keywords[0]);
    }

    #[test]
    fn get_relevant_keywords_match_with() {
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
        let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

        let keywords_option = get_relevant_keywords_match(&interpreter, interpreter.index, &keywords_to_match);

        assert!(keywords_option.is_none());
    }
}

/*
try to match any operator defined in BqsqlOperator
 */
pub(crate) fn get_relevant_operators_all_match<'a>(
    interpreter: &'a BqsqlInterpreter,
) -> Option<BqsqlOperator> {
    //do not accept comments in the beginning
    if is_line_comment(interpreter, interpreter.index) {
        return None;
    }

    let all_operators = &BqsqlOperator::get_all();
    let mut operators_list: Vec<(&BqsqlOperator, Vec<&str>)> =
        all_operators.iter().map(|i| (i, i.to_vec())).collect();

    operators_list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    for operator in operators_list {
        let mut matched = 0;
        let mut index = interpreter.index;

        while is_in_range(interpreter, index) && matched < operator.1.len() {
            let op = operator.1[matched];

            if is_line_comment(interpreter, index) {
                index += 1;
            } else {
                if let Some(string_in_range) = get_string_in_range(interpreter, index) {
                    if string_in_range.to_uppercase() == op {
                        index += 1;
                        matched += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if matched >= operator.1.len() {
                return Some(operator.0.clone());
            }
        }
    }

    None
}

pub(crate) fn handle_semicolon(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    //do not accept comments in the beginning
    if let Some(string_in_range) = get_string_in_range(interpreter, interpreter.index) {
        if string_in_range == ";" {
            return handle_document_item(interpreter, BqsqlDocumentItemType::Semicolon, None);
        }
    }

    None
}

impl BqsqlDocumentItem {
    pub(crate) fn new(
        item_type: BqsqlDocumentItemType,
        items: Vec<Option<BqsqlDocumentItem>>,
    ) -> BqsqlDocumentItem {
        let items = items.into_iter().filter_map(|f| f).collect();

        BqsqlDocumentItem {
            item_type: item_type,
            range: None,
            items: items,
            keyword: None,
        }
    }
}
