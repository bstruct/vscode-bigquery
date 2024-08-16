# ====================== EXPERIMENTAL ==================

## bqsql-parser

## query expression

https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax
```
query_statement:
    query_expr

query_expr:
    [ WITH [ RECURSIVE ] { non_recursive_cte | recursive_cte }[, ...] ]
    { select | ( query_expr ) | set_operation }
    [ ORDER BY expression [{ ASC | DESC }] [, ...] ]
    [ LIMIT count [ OFFSET skip_rows ] ]

select:
    SELECT
        [ { ALL | DISTINCT } ]
        [ AS { STRUCT | VALUE } ]
        select_list
    [ FROM from_clause[, ...] ]
    [ WHERE bool_expression ]
    [ GROUP BY { expression [, ...] | ROLLUP ( expression [, ...] ) } ]
    [ HAVING bool_expression ]
    [ QUALIFY bool_expression ]
    [ WINDOW window_clause ]
```

#### docs
https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
https://code.visualstudio.com/docs/cpp/config-wsl#_set-up-your-linux-environment
https://docs.rs/regex/latest/regex/

#### build the package
wasm-pack build --target nodejs 
wasm-pack build --target bundler

xxx wasm-pack build --target web
xxx--no-typescript
xxx--out-dir static node_modules/@bstruct/bqsql-parser/bqsql_parser_bg.wasm


#### built js is wrong

const path = require('path').join(__dirname, '..', 'node_modules', '@bstruct', 'bqsql-parser', 'bqsql_parser_bg.wasm');
