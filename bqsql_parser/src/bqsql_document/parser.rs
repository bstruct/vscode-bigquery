// use lazy_static::lazy_static;
// use regex::Regex;

// use crate::bqsql_document::*;

// use super::bsql_interpreter::BqsqlInterpreter;

// use crate::bqsql_document::token_parser;

// impl BqsqlDocument {
//     pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {

//         return BqsqlInterpreter::new(bqsql).iterate().compile();

//         // let mut items: Vec<BqsqlDocumentItem> = Vec::new();

//         // let lines = &bqsql.lines().map(|l| l).collect::<Vec<&str>>();
//         // let tokens = &token_parser::parse_tokens(bqsql);

//         // let mut index: usize = 0;

//         // while index < tokens.len() {
//         //     //comments are not relevant for now
//         //     if is_comment(lines, tokens, index) {
//         //         index = index + 1;
//         //         continue;
//         //     }

//         //     if let Some((query, new_index)) = handle_query(lines, tokens, index) {
//         //         index = new_index;
//         //         items.push(query);
//         //         continue;
//         //     }

//         //     //for now, only queries are supported.
//         //     //if there's other types of syntaxes, (DDL, DML, DCL,...), they will be ignored
//         //     while index < tokens.len() {
//         //         if let Some(string_in_range) = get_string_in_range(lines, &tokens[index]) {
//         //             if string_in_range != ";" {
//         //                 index = index + 1;
//         //             } else {

//         //                 items.push(BqsqlDocumentItem{
//         //                     item_type: BqsqlDocumentItemType::Semicolon,
//         //                     items:vec![],
//         //                     range: Some(tokens[index])
//         //                 });

//         //                 break;
//         //             }
//         //         } else {
//         //             index = index + 1;
//         //         }
//         //     }

//         //     index = index + 1;
//         // }

//         // BqsqlDocument { items: items }
//     }
// }

// fn get_string_in_range<'a>(lines: &Vec<&'a str>, range: &[usize; 3]) -> Option<&'a str> {
//     if &lines[range[0]].len() >= &range[2] {
//         return Some(&lines[range[0]][range[1]..range[2]]);
//     }
//     None
// }

// fn is_comment<'a>(lines: &Vec<&'a str>, tokens: &[[usize; 3]], index: usize) -> bool {
//     if let Some(string_in_range) = get_string_in_range(lines, &tokens[index]) {
//         return string_in_range.starts_with("--");
//     }
//     false
// }

// fn handle_query<'a>(
//     lines: &Vec<&'a str>,
//     tokens: &[[usize; 3]],
//     start_index: usize,
// ) -> Option<(BqsqlDocumentItem, usize)> {
//     let mut index = start_index;

//     if let Some(string_in_range) = get_string_in_range(lines, &tokens[index]) {
//         if string_in_range.to_uppercase() == "SELECT" {
//             let mut items: Vec<BqsqlDocumentItem> = Vec::new();

//             //QuerySelect
//             let select = handle_query_resolve_select(lines, tokens, index);
//             // items.push(select.0);
//             index = select.1;

//             //QuerySelectListItem(s)
//             if let Some(select_list) = handle_query_resolve_select_list(lines, tokens, index) {
//                 let mut new_item_list: Vec<BqsqlDocumentItem> = Vec::from(select.0.items);
//                 let mut append_list = Vec::from(select_list.0);
//                 new_item_list.append(&mut append_list);

//                 items.push(BqsqlDocumentItem {
//                     item_type: select.0.item_type,
//                     range: select.0.range,
//                     items: new_item_list,
//                 });

//                 index = select_list.1;
//             } else {
//                 items.push(select.0);
//             }

//             //FROM
//             //..

//             let item = BqsqlDocumentItem {
//                 item_type: BqsqlDocumentItemType::Query,
//                 range: None,
//                 items: items,
//             };

//             return Some((item, index));
//         }
//     }

//     None
// }

// fn handle_query_resolve_select<'a>(
//     lines: &Vec<&'a str>,
//     tokens: &[[usize; 3]],
//     index: usize,
// ) -> (BqsqlDocumentItem, usize) {
//     if tokens.len() > index + 1 {
//         if let Some(string_in_range) = get_string_in_range(lines, &tokens[index + 1]) {
//             let string_in_range_upper = string_in_range.to_uppercase();

//             //
//             //QuerySelectAll
//             let select_all = string_in_range_upper == "ALL";
//             if select_all {
//                 let item = BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::QuerySelectAll,
//                     range: None,
//                     items: vec![
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index]),
//                             items: vec![],
//                         },
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index + 1]),
//                             items: vec![],
//                         },
//                     ],
//                 };

//                 return (item, index + 2);
//             }

//             //
//             //QuerySelectDistinct
//             let select_distinct = string_in_range_upper == "DISTINCT";
//             if select_distinct {
//                 let item = BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::QuerySelectDistinct,
//                     range: None,
//                     items: vec![
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index]),
//                             items: vec![],
//                         },
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index + 1]),
//                             items: vec![],
//                         },
//                     ],
//                 };

//                 return (item, index + 2);
//             }
//         }
//     }

//     if tokens.len() > index + 2 {
//         if let Some(string_in_range_1) = get_string_in_range(lines, &tokens[index + 1]) {
//             if let Some(string_in_range_2) = get_string_in_range(lines, &tokens[index + 2]) {
//                 let string_in_range_1_upper = string_in_range_1.to_uppercase();
//                 let string_in_range_2_upper = string_in_range_2.to_uppercase();

//                 //
//                 //QuerySelectAsStruct
//                 let select_as_struct =
//                     string_in_range_1_upper == "AS" && string_in_range_2_upper == "STRUCT";
//                 if select_as_struct {
//                     let item = BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::QuerySelectAsStruct,
//                         range: None,
//                         items: vec![
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 1]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 2]),
//                                 items: vec![],
//                             },
//                         ],
//                     };

//                     return (item, index + 3);
//                 }

//                 //
//                 //QuerySelectAsValue
//                 let select_as_value =
//                     string_in_range_1_upper == "AS" && string_in_range_2_upper == "VALUE";
//                 if select_as_value {
//                     let item = BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::QuerySelectAsValue,
//                         range: None,
//                         items: vec![
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 1]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 2]),
//                                 items: vec![],
//                             },
//                         ],
//                     };

//                     return (item, index + 4);
//                 }
//             }
//         }
//     }
//     //QuerySelect
//     let item = BqsqlDocumentItem {
//         item_type: BqsqlDocumentItemType::QuerySelect,
//         range: None,
//         items: vec![BqsqlDocumentItem {
//             item_type: BqsqlDocumentItemType::Keyword,
//             range: Some(tokens[index]),
//             items: vec![],
//         }],
//     };

//     return (item, index + 1);
// }

// #[test]
// fn handle_query_resolve_select_select() {
//     let document = handle_query_resolve_select(&vec!["SELECT"], &vec![[0, 0, 6]], 0);

//     assert_eq!(BqsqlDocumentItemType::QuerySelect, document.0.item_type);
//     assert_eq!(None, document.0.range);
//     let items = document.0.items;
//     assert_eq!(1, items.len());

//     let item_1 = &items[0];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_1.item_type);
//     assert_eq!(Some([0, 0, 6]), item_1.range);
//     assert_eq!(0, item_1.items.len());
// }

// #[test]
// fn handle_query_resolve_select_select_all() {
//     let document =
//         handle_query_resolve_select(&vec!["SELECT ALL"], &vec![[0, 0, 6], [0, 7, 10]], 0);

//     assert_eq!(BqsqlDocumentItemType::QuerySelectAll, document.0.item_type);
//     assert_eq!(None, document.0.range);
//     let items = document.0.items;
//     assert_eq!(2, items.len());

//     let item_1 = &items[0];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_1.item_type);
//     assert_eq!(Some([0, 0, 6]), item_1.range);
//     assert_eq!(0, item_1.items.len());

//     let item_2 = &items[1];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_2.item_type);
//     assert_eq!(Some([0, 7, 10]), item_2.range);
//     assert_eq!(0, item_2.items.len());
// }

// #[test]
// fn handle_query_resolve_select_select_distinct() {
//     let document =
//         handle_query_resolve_select(&vec!["SELECT DISTINCT"], &vec![[0, 0, 6], [0, 7, 15]], 0);

//     assert_eq!(
//         BqsqlDocumentItemType::QuerySelectDistinct,
//         document.0.item_type
//     );
//     assert_eq!(None, document.0.range);
//     let items = document.0.items;
//     assert_eq!(2, items.len());

//     let item_1 = &items[0];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_1.item_type);
//     assert_eq!(Some([0, 0, 6]), item_1.range);
//     assert_eq!(0, item_1.items.len());

//     let item_2 = &items[1];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_2.item_type);
//     assert_eq!(Some([0, 7, 15]), item_2.range);
//     assert_eq!(0, item_2.items.len());
// }

// #[test]
// fn handle_query_resolve_select_select_as_value() {
//     let document = handle_query_resolve_select(
//         &vec!["SELECT AS VALUE"],
//         &vec![[0, 0, 6], [0, 7, 9], [0, 10, 15]],
//         0,
//     );

//     assert_eq!(
//         BqsqlDocumentItemType::QuerySelectAsValue,
//         document.0.item_type
//     );
//     assert_eq!(None, document.0.range);
//     let items = document.0.items;
//     assert_eq!(3, items.len());

//     let item_0 = &items[0];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_0.item_type);
//     assert_eq!(Some([0, 0, 6]), item_0.range);
//     assert_eq!(0, item_0.items.len());

//     let item_1 = &items[1];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_1.item_type);
//     assert_eq!(Some([0, 7, 9]), item_1.range);
//     assert_eq!(0, item_1.items.len());

//     let item_2 = &items[2];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_2.item_type);
//     assert_eq!(Some([0, 10, 15]), item_2.range);
//     assert_eq!(0, item_2.items.len());
// }

// #[test]
// fn handle_query_resolve_select_select_as_struct() {
//     let document = handle_query_resolve_select(
//         &vec!["SELECT AS STRUCT"],
//         &vec![[0, 0, 6], [0, 7, 9], [0, 10, 16]],
//         0,
//     );

//     assert_eq!(
//         BqsqlDocumentItemType::QuerySelectAsStruct,
//         document.0.item_type
//     );
//     assert_eq!(None, document.0.range);
//     let items = document.0.items;
//     assert_eq!(3, items.len());

//     let item_0 = &items[0];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_0.item_type);
//     assert_eq!(Some([0, 0, 6]), item_0.range);
//     assert_eq!(0, item_0.items.len());

//     let item_1 = &items[1];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_1.item_type);
//     assert_eq!(Some([0, 7, 9]), item_1.range);
//     assert_eq!(0, item_1.items.len());

//     let item_2 = &items[2];
//     assert_eq!(BqsqlDocumentItemType::Keyword, item_2.item_type);
//     assert_eq!(Some([0, 10, 16]), item_2.range);
//     assert_eq!(0, item_2.items.len());
// }

// fn handle_query_resolve_select_list<'a>(
//     lines: &Vec<&'a str>,
//     tokens: &[[usize; 3]],
//     start_index: usize,
// ) -> Option<(Vec<BqsqlDocumentItem>, usize)> {
//     lazy_static! {
//         static ref NUMBER: Regex = Regex::new(r"\d+|\d*\.{1}\d*").unwrap();
//     }
//     let mut index = start_index;
//     let mut count_open_parentheses: usize = 0;
//     let mut document_items: Vec<BqsqlDocumentItem> = Vec::new();
//     let mut select_item_items: Vec<BqsqlDocumentItem> = Vec::new();
//     let mut alias_expected = false;

//     while tokens.len() > index {
//         if let Some(string_in_range) = get_string_in_range(lines, &tokens[index]) {
//             //the point is only to solve the select list. The from should be handled in the parent function
//             if string_in_range.to_uppercase() == "FROM" || string_in_range == ";" {
//                 return Some((document_items, index));
//             }

//             if string_in_range == "," {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::Comma,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });

//                 if count_open_parentheses == 0 {
//                     document_items.push(BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::QuerySelectListItem,
//                         range: None,
//                         items: select_item_items,
//                     });

//                     select_item_items = Vec::new();
//                 }

//                 index = index + 1;
//                 alias_expected = false;
//                 continue;
//             }

//             //confirm that * is not an operator
//             if string_in_range == "*" {}

//             if ["+", "-", "/", "*"].contains(&string_in_range) {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::Operator,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });

//                 index = index + 1;
//                 alias_expected = false;
//                 continue;
//             }

//             if string_in_range == "(" {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::ParenthesesOpen,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });
//                 count_open_parentheses = count_open_parentheses + 1;

//                 index = index + 1;
//                 alias_expected = false;
//                 continue;
//             }

//             if string_in_range == ")" {
//                 if count_open_parentheses == 0 {
//                     if select_item_items.len() > 0 {
//                         document_items.push(BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::QuerySelectListItem,
//                             range: None,
//                             items: select_item_items,
//                         });
//                     }

//                     return Some((document_items, index));
//                 } else {
//                     select_item_items.push(BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::ParenthesesClose,
//                         range: Some(tokens[index]),
//                         items: vec![],
//                     });
//                     count_open_parentheses = count_open_parentheses - 1;
//                 }
//                 index = index + 1;
//                 alias_expected = true;
//                 continue;
//             }

//             if string_in_range.starts_with("\"") || string_in_range.starts_with("'") {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::String,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });

//                 index = index + 1;
//                 alias_expected = true;
//                 continue;
//             }

//             if NUMBER.is_match(string_in_range) {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::Number,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });

//                 index = index + 1;
//                 alias_expected = true;
//                 continue;
//             }

//             if string_in_range == "SELECT" {
//                 if let Some(query) = handle_query(lines, tokens, index) {
//                     select_item_items.push(query.0);
//                     index = query.1;
//                     alias_expected = false;
//                     continue;
//                 } else {
//                     panic!("query expected");
//                 }
//             }

//             if string_in_range == "AS" {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::KeywordAs,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });
//                 index = index + 1;
//                 alias_expected = true;
//                 continue;
//             }

//             //alias
//             if alias_expected && select_item_items.len() > 0 {
//                 select_item_items.push(BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::Alias,
//                     range: Some(tokens[index]),
//                     items: vec![],
//                 });

//                 index = index + 1;
//                 alias_expected = false;
//                 continue;
//             }
//         }

//         index = index + 1;
//     }

//     if select_item_items.len() > 0 {
//         document_items.push(BqsqlDocumentItem {
//             item_type: BqsqlDocumentItemType::QuerySelectListItem,
//             range: None,
//             items: select_item_items,
//         });
//     }

//     if document_items.len() > 0 {
//         return Some((document_items, index));
//     }

//     None
// }
