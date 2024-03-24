wasm-pack build --release --target nodejs

cp ./pkg/bqsql_parser_bg.wasm ../dist/bqsql_parser_bg.wasm
cp ./pkg/bqsql_parser.js ../dist/bqsql_parser.js