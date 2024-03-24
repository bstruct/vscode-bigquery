// query_statement:
//  query_expr

// query_expr:
// [ WITH [ RECURSIVE ] { non_recursive_cte | recursive_cte }[, ...] ]
// { select | ( query_expr ) | set_operation }
// [ ORDER BY expression [{ ASC | DESC }] [, ...] ]
// [ LIMIT count [ OFFSET skip_rows ] ]

// select:
// SELECT
//     [ { ALL | DISTINCT } ]
//     [ AS { STRUCT | VALUE } ]
//     select_list
// [ FROM from_clause[, ...] ]
// [ WHERE bool_expression ]
// [ GROUP BY { expression [, ...] | ROLLUP ( expression [, ...] ) } ]
// [ HAVING bool_expression ]
// [ QUALIFY bool_expression ]
// [ WINDOW window_clause ]

use super::{BqsqlDocumentItemType, BqsqlKeyword};

#[derive(Debug, Clone, Copy)]
pub(crate) enum BqsqlQueryStructure {
    With = 1,
    Select = 2,
    From = 3,
    Where = 4,
    GroupBy = 5,
    Rollup = 6,
    Having = 7,
    Qualify = 8,
    Window = 9,
    OrderBy = 10,
    Limit = 11,
    Offset = 12,
}

impl BqsqlQueryStructure {
    pub(crate) fn get_keywords(&self) -> Vec<Vec<BqsqlKeyword>> {
        match self {
            BqsqlQueryStructure::With => {
                return vec![
                    vec![BqsqlKeyword::With, BqsqlKeyword::Recursive],
                    vec![BqsqlKeyword::With],
                ];
            }
            BqsqlQueryStructure::Select => {
                return vec![
                    vec![BqsqlKeyword::Select, BqsqlKeyword::As, BqsqlKeyword::Struct],
                    vec![BqsqlKeyword::Select, BqsqlKeyword::As, BqsqlKeyword::Value],
                    vec![BqsqlKeyword::Select, BqsqlKeyword::Distinct],
                    vec![BqsqlKeyword::Select, BqsqlKeyword::All],
                    vec![BqsqlKeyword::Select],
                ];
            }
            BqsqlQueryStructure::From => {
                return vec![vec![BqsqlKeyword::From]];
            }
            BqsqlQueryStructure::Where => {
                return vec![vec![BqsqlKeyword::Where]];
            }
            BqsqlQueryStructure::GroupBy => {
                return vec![vec![BqsqlKeyword::Group, BqsqlKeyword::By]];
            }
            BqsqlQueryStructure::Rollup => {
                return vec![vec![BqsqlKeyword::Rollup]];
            }
            BqsqlQueryStructure::Having => {
                return vec![vec![BqsqlKeyword::Having]];
            }
            BqsqlQueryStructure::Qualify => {
                return vec![vec![BqsqlKeyword::Qualify]];
            }
            BqsqlQueryStructure::Window => {
                return vec![vec![BqsqlKeyword::Window]];
            }
            BqsqlQueryStructure::OrderBy => {
                return vec![vec![BqsqlKeyword::Order, BqsqlKeyword::By]];
            }
            BqsqlQueryStructure::Limit => {
                return vec![vec![BqsqlKeyword::Limit]];
            }
            BqsqlQueryStructure::Offset => {
                return vec![vec![BqsqlKeyword::Offset]];
            }
        }
    }

    pub(crate) fn get_all() -> Vec<BqsqlQueryStructure> {
        vec![
            BqsqlQueryStructure::With,
            BqsqlQueryStructure::Select,
            BqsqlQueryStructure::From,
            BqsqlQueryStructure::Where,
            BqsqlQueryStructure::GroupBy,
            BqsqlQueryStructure::Rollup,
            BqsqlQueryStructure::Having,
            BqsqlQueryStructure::Qualify,
            BqsqlQueryStructure::Window,
            BqsqlQueryStructure::OrderBy,
            BqsqlQueryStructure::Limit,
            BqsqlQueryStructure::Offset,
        ]
    }

    pub(crate) fn get_subsequent_query_structure(&self) -> Vec<BqsqlQueryStructure> {
        BqsqlQueryStructure::get_all()[(*self as usize)..].to_vec()
    }

    pub(crate) fn get_document_item_type(&self) -> super::BqsqlDocumentItemType {
        match self {
            BqsqlQueryStructure::With => {
                return BqsqlDocumentItemType::QueryWith;
            }
            BqsqlQueryStructure::Select => {
                return BqsqlDocumentItemType::QuerySelect;
            }
            BqsqlQueryStructure::From => {
                return BqsqlDocumentItemType::QueryFrom;
            }
            BqsqlQueryStructure::Where => {
                return BqsqlDocumentItemType::QueryWhere;
            }
            BqsqlQueryStructure::GroupBy => {
                return BqsqlDocumentItemType::QueryGroupBy;
            }
            BqsqlQueryStructure::Rollup => {
                return BqsqlDocumentItemType::QueryRollup;
            }
            BqsqlQueryStructure::Having => {
                return BqsqlDocumentItemType::QueryHaving;
            }
            BqsqlQueryStructure::Qualify => {
                return BqsqlDocumentItemType::QueryQualify;
            }
            BqsqlQueryStructure::Window => {
                return BqsqlDocumentItemType::QueryWindow;
            }
            BqsqlQueryStructure::OrderBy => {
                return BqsqlDocumentItemType::QueryOrderBy;
            }
            BqsqlQueryStructure::Limit => {
                return BqsqlDocumentItemType::QueryLimit;
            }
            BqsqlQueryStructure::Offset => {
                return BqsqlDocumentItemType::QueryOffset;
            }
        }
    }
}
