use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum BqsqlFunction {
    //https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions
    AnyValue,
    ArrayAgg,
    ArrayConcatAgg,
    Avg,
    BitAnd,
    BitOr,
    BitXor,
    Count,
    CountIf,
    LogicalAnd,
    LogicalOr,
    Max,
    Min,
    StringAgg,
    Sum,
}

#[derive(Serialize, Clone)]
pub struct BqsqlFunctionSnippet {
    name: &'static str,
    snippet: &'static str,
    url: &'static str,
}

impl BqsqlFunction {
    pub(crate) fn get_all() -> Vec<BqsqlFunction> {
        return vec![
            BqsqlFunction::AnyValue,
            BqsqlFunction::ArrayAgg,
            BqsqlFunction::ArrayConcatAgg,
            BqsqlFunction::Avg,
            BqsqlFunction::BitAnd,
            BqsqlFunction::BitOr,
            BqsqlFunction::BitXor,
            BqsqlFunction::Count,
            BqsqlFunction::CountIf,
            BqsqlFunction::LogicalAnd,
            BqsqlFunction::LogicalOr,
            BqsqlFunction::Max,
            BqsqlFunction::Min,
            BqsqlFunction::StringAgg,
            BqsqlFunction::Sum,
        ];
    }

    pub(crate) fn get_snippets(&self) -> Vec<BqsqlFunctionSnippet> {
        match self {
            BqsqlFunction::AnyValue=>Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "ANY_VALUE",
                    snippet: "ANY_VALUE(${1:some_column}) AS ${2:any_value},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#any_value"
                },
                BqsqlFunctionSnippet {
                    name: "ANY_VALUE OVER",
                    snippet: "ANY_VALUE(${1:some_column}) OVER (ORDER BY ${2:some_column} ROWS BETWEEN ${3:1} PRECEDING AND CURRENT ROW) AS ${4:any_value},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#any_value"
                },
            ]),
            BqsqlFunction::ArrayAgg => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "ARRAY_AGG",
                    snippet: "ARRAY_AGG(${1:some_column}) AS ${2:array_agg},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#array_agg"
                },
            ]),
            BqsqlFunction::ArrayConcatAgg => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "ARRAY_CONCAT_AGG",
                    snippet: "ARRAY_CONCAT_AGG(${1:some_column}) AS ${2:array_concat_agg},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#array_concat_agg"
                },
            ]),
            BqsqlFunction::Avg => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "AVG",
                    snippet: "AVG(${1:some_column}) AS ${2:avg},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#avg"
                },
            ]),
            BqsqlFunction::BitAnd => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "BIT_AND",
                    snippet: "BIT_AND(${1:some_column}) AS ${2:bit_and},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#bit_and"
                },
            ]),
            BqsqlFunction::BitOr => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "BIT_OR",
                    snippet: "BIT_OR(${1:some_column}) AS ${2:bit_or},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#bit_or"
                },
            ]),
            BqsqlFunction::BitXor => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "BIT_XOR",
                    snippet: "BIT_XOR(${1:some_column}) AS ${2:bit_xor},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#bit_xor"
                },
            ]),
            BqsqlFunction::Count => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "COUNT",
                    snippet: "COUNT(${1:some_column}) AS ${2:count},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#count"
                },BqsqlFunctionSnippet {
                    name: "COUNT OVER",
                    snippet: "COUNT(${1:some_column}) OVER (PARTITION BY ${2:some_column}) AS ${3:count},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#count"
                }
            ]),
            BqsqlFunction::CountIf => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "COUNTIF",
                    snippet: "COUNTIF(${1:some_column}) AS ${2:count_if},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#countif"
                },BqsqlFunctionSnippet {
                    name: "COUNTIF",
                    snippet: "COUNTIF(${1:some_column}) OVER (ORDER BY ${2:some_column} ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING) AS ${3:count_if},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#countif"
                },
            ]),
            BqsqlFunction::LogicalAnd => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "LOGICAL_AND",
                    snippet: "LOGICAL_AND(${1:some_column}) AS ${2:logical_and},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#logical_and"
                },
            ]),
            BqsqlFunction::LogicalOr => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "LOGICAL_OR",
                    snippet: "LOGICAL_OR(${1:some_column}) AS ${2:logical_or},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#logical_or"
                },
            ]),
            BqsqlFunction::Max => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "MAX",
                    snippet: "MAX(${1:some_column}) AS ${2:max},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#max"
                },
            ]),
            BqsqlFunction::Min => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "MIN",
                    snippet: "MIN(${1:some_column}) AS ${2:min},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#min"
                },
            ]),
            BqsqlFunction::StringAgg => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "STRING_AGG",
                    snippet: "STRING_AGG(${1:some_column}) AS ${2:string_agg},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#string_agg"
                },
            ]),
            BqsqlFunction::Sum => Vec::from(vec![
                BqsqlFunctionSnippet {
                    name: "SUM",
                    snippet: "SUM(${1:some_column}) AS ${2:sum},",
                    url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#sum"
                },
            ]),
    }
    }
}
