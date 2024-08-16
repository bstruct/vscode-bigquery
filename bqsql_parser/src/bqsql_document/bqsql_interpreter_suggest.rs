use crate::bqsql_function::BqsqlFunction;

use super::{
    bqsql_interpreter::BqsqlInterpreter, BqsqlDocumentItem, BqsqlDocumentItemType,
    BqsqlDocumentSuggestion, BqsqlDocumentSuggestionType,
};

impl BqsqlInterpreter<'_> {
    pub(crate) fn suggest(bqsql: &str, position: [usize; 2]) -> Vec<BqsqlDocumentSuggestion> {
        let document_items = &BqsqlInterpreter::new(bqsql).collect();
        // let location_in_document = locate_in_document(flat_items, position);

        // let mut suggestions = Vec::new();
        // suggestions.append(&mut suggest_syntax(flat_items, location_in_document));
        // suggestions.append(&mut suggest_functions(flat_items, location_in_document));
        // suggestions.append(&mut suggest_columns(flat_items, location_in_document));

        // suggestions
        suggest_in_select(document_items, position)
    }
}

fn suggest_in_select(
    document_items: &Vec<BqsqlDocumentItem>,
    position: [usize; 2],
) -> Vec<BqsqlDocumentSuggestion> {
    //for now limit this for 1 query only support

    if document_items
        .iter()
        .filter(|i| i.item_type == BqsqlDocumentItemType::Query)
        .count()
        == 1
    {
        // let mut suggestions: Vec<BqsqlDocumentSuggestion> = Vec::new();
        if is_in_select_an_impactable(document_items, position) {
            let table_identifiers = get_table_identifiers(document_items);

            let mut suggestions: Vec<BqsqlDocumentSuggestion> = table_identifiers
                .iter()
                .map(|i| BqsqlDocumentSuggestion {
                    suggestion_type: BqsqlDocumentSuggestionType::TableColumns,
                    table_identifier: Some(i.clone().to_owned()),
                    snippets: None,
                })
                .collect();

            let mut all_functions: Vec<BqsqlDocumentSuggestion> = BqsqlFunction::get_all()
                .iter()
                .map(|i| BqsqlDocumentSuggestion {
                    suggestion_type: BqsqlDocumentSuggestionType::Function,
                    table_identifier: None,
                    snippets: Some(i.get_snippets()),
                })
                .collect();

            suggestions.append(&mut all_functions);

            return suggestions;
        }
    }

    Vec::new()
}

fn is_in_select_an_impactable(
    document_items: &Vec<BqsqlDocumentItem>,
    position: [usize; 2],
) -> bool {
    let flat_items = &flat_document(document_items);
    let document_location = locate_in_document(flat_items, position);
    if document_location.0 == LocationInDocumentType::After {
        if let Some(index) = document_location.1 {
            //find previous item with no range
            if let Some(previous_item_with_no_range) =
                get_previous_item_with_no_range(flat_items, index)
            {
                if previous_item_with_no_range.item_type == BqsqlDocumentItemType::QuerySelect
                    || previous_item_with_no_range.item_type
                        == BqsqlDocumentItemType::QuerySelectListItem
                {
                    return true;
                }
            }
        }
    }

    false
}

fn get_previous_item_with_no_range<'a>(
    flat_items: &Vec<&'a BqsqlDocumentItem>,
    index: usize,
) -> Option<&'a BqsqlDocumentItem> {
    let mut i = index;
    while i > 0 {
        i -= 1;

        if flat_items[i].range.is_none() {
            return Some(&flat_items[i]);
        }
    }

    None
}

fn get_table_identifiers<'a>(
    document_items: &'a Vec<BqsqlDocumentItem>,
) -> Vec<&'a BqsqlDocumentItem> {
    let mut table_identifiers: Vec<&BqsqlDocumentItem> = Vec::new();

    let mut index: usize = 0;
    while index < document_items.len() {
        if document_items[index].item_type == BqsqlDocumentItemType::TableIdentifier {
            table_identifiers.push(&document_items[index]);
        } else if document_items[index].items.len() > 0 {
            table_identifiers.append(&mut get_table_identifiers(&document_items[index].items));
        }
        index += 1;
    }

    table_identifiers
}

#[cfg(test)]
mod tests_is_in_select_an_impactable {
    use crate::bqsql_document::{
        bqsql_interpreter::BqsqlInterpreter, bqsql_interpreter_suggest::is_in_select_an_impactable,
    };

    #[test]
    fn is_in_select_an_impactable_after_select_3() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *");
        let document_items = &interpreter.collect();

        assert!(!is_in_select_an_impactable(document_items, [0, 3]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_6() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *");
        let document_items = &interpreter.collect();

        assert!(!is_in_select_an_impactable(document_items, [0, 6]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_8() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *");
        let document_items = &interpreter.collect();

        assert!(!is_in_select_an_impactable(document_items, [0, 8]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_9() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *,");
        let document_items = &interpreter.collect();

        assert!(!is_in_select_an_impactable(document_items, [0, 9]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_1_0() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *,\n");
        let document_items = &interpreter.collect();

        assert!(is_in_select_an_impactable(document_items, [1, 0]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_10() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *, ");
        let document_items = &interpreter.collect();

        assert!(is_in_select_an_impactable(document_items, [0, 10]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_10_from() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *, \nFROM");
        let document_items = &interpreter.collect();

        assert!(is_in_select_an_impactable(document_items, [0, 10]));
    }

    #[test]
    fn is_in_select_an_impactable_after_select_10_from_table() {
        let interpreter = &mut BqsqlInterpreter::new("SELECT *, \nFROM dataset_id.table_id");
        let document_items = &interpreter.collect();

        assert!(is_in_select_an_impactable(document_items, [0, 10]));
    }
}

#[cfg(test)]
mod tests {
    use crate::bqsql_document::{
        bqsql_interpreter::BqsqlInterpreter, BqsqlDocumentItemType, BqsqlDocumentSuggestionType,
    };

    #[test]
    fn suggest_nothing() {
        let suggestions = BqsqlInterpreter::suggest("SELECT * ", [0, 3]);

        assert_eq!(0, suggestions.len());
    }

    #[test]
    fn suggest_nothing_after_comment() {
        let suggestions = BqsqlInterpreter::suggest("SELECT * --test comment ", [0, 24]);

        assert_eq!(0, suggestions.len());
    }

    #[test]
    fn suggest_columns_from_table() {
        let suggestions =
            BqsqlInterpreter::suggest("SELECT a,b,c,  FROM dataset_id.table_id", [0, 14]);

        assert_eq!(16, suggestions.len());

        assert_eq!(
            BqsqlDocumentSuggestionType::TableColumns,
            suggestions[0].suggestion_type
        );
        assert!(suggestions[0].table_identifier.is_some());

        let table_identifier = suggestions[0].table_identifier.as_ref().unwrap();
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

    // #[test]
    // #[ignore = "not ready yet"]
    // fn suggest_except_from_after_comment() {
    //     let suggestions = BqsqlInterpreter::suggest("SELECT * --comment\n", [1, 0]);

    //     assert_eq!(2, suggestions.len());

    //     //EXCEPT
    //     assert_eq!("EXCEPT", suggestions[0].name);
    //     assert_eq!("EXCEPT(${0:some_column}),", suggestions[0].snippet);

    //     //FROM
    //     assert_eq!("FROM", suggestions[1].name);
    //     assert_eq!("FROM ${0:some_table}", suggestions[1].snippet);
    // }
}

fn flat_document<'a>(document_items: &'a Vec<BqsqlDocumentItem>) -> Vec<&'a BqsqlDocumentItem> {
    let mut flat_items: Vec<&'a BqsqlDocumentItem> = Vec::new();

    for item in document_items {
        flat_items.push(item);
        if item.items.len() > 0 {
            flat_items.append(&mut flat_document(&item.items));
        }
    }

    flat_items
}

#[cfg(test)]
mod tests_flat_document {
    use crate::bqsql_document::bqsql_interpreter::BqsqlInterpreter;

    use super::flat_document;

    #[test]
    fn flat_document_simple() {
        let document_items = &BqsqlInterpreter::new("SELECT 1,2,3,4,5 FROM t").collect();

        let flat_items = flat_document(document_items);
        assert_eq!(21, flat_items.len())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocationInDocumentType {
    None,
    Middle,
    After,
}

fn locate_in_document<'a>(
    flat_items: &'a Vec<&'a BqsqlDocumentItem>,
    position: [usize; 2],
) -> (LocationInDocumentType, Option<usize>) {
    let mut previous_index: Option<usize> = None;

    if position != [0, 0] {
        if flat_items.len() > 0 {
            // let mut iter = flat_items.iter();
            let mut index: usize = 0;

            while index < flat_items.len() {
                let item = flat_items[index];

                if let Some(range) = item.range {
                    if range[0] == position[0] {
                        if range[1] < position[1] && range[2] >= position[1] {
                            return (LocationInDocumentType::Middle, Some(index));
                        } else if range[1] >= position[1] {
                            return (LocationInDocumentType::After, previous_index);
                        }
                    } else if range[0] > position[0] {
                        return (LocationInDocumentType::After, previous_index);
                    }
                    previous_index = Some(index);
                }
                index += 1;
            }
            if previous_index.is_some() {
                return (LocationInDocumentType::After, previous_index);
            }
        }
    }

    (LocationInDocumentType::None, None)
}

#[cfg(test)]
mod tests_locate_in_document {
    use crate::bqsql_document::{
        bqsql_interpreter::BqsqlInterpreter,
        bqsql_interpreter_suggest::{flat_document, locate_in_document, LocationInDocumentType},
        BqsqlDocumentItemType,
    };

    #[test]
    fn locate_in_document_beggining() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 0]);
        assert_eq!(LocationInDocumentType::None, locate.0);
        assert!(locate.1.is_none());
    }

    #[test]
    fn locate_in_document_middle_1() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 1]);
        assert_eq!(LocationInDocumentType::Middle, locate.0);
        assert!(locate.1.is_some());
        let item = flat_items[locate.1.unwrap()];
        assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
        assert_eq!(Some([0, 0, 6]), item.range);
    }

    #[test]
    fn locate_in_document_middle_3() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 3]);
        assert_eq!(LocationInDocumentType::Middle, locate.0);
        assert!(locate.1.is_some());
        let item = flat_items[locate.1.unwrap()];
        assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
        assert_eq!(Some([0, 0, 6]), item.range);
    }

    #[test]
    fn locate_in_document_just_after_6() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 6]);
        assert_eq!(LocationInDocumentType::Middle, locate.0);
        assert!(locate.1.is_some());
        let item = flat_items[locate.1.unwrap()];
        assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
        assert_eq!(Some([0, 0, 6]), item.range);
    }

    #[test]
    fn locate_in_document_after_7() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 7]);
        assert_eq!(LocationInDocumentType::After, locate.0);
        assert!(locate.1.is_some());
        let item = flat_items[locate.1.unwrap()];
        assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
        assert_eq!(Some([0, 0, 6]), item.range);
    }

    #[test]
    fn locate_in_document_just_after_8() {
        let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
        let flat_items = &flat_document(document_items);

        let locate = locate_in_document(flat_items, [0, 8]);
        assert_eq!(LocationInDocumentType::Middle, locate.0);
        assert!(locate.1.is_some());
        let item = flat_items[locate.1.unwrap()];
        assert_eq!(BqsqlDocumentItemType::Operator, item.item_type);
        assert_eq!(Some([0, 7, 8]), item.range);
    }
}
