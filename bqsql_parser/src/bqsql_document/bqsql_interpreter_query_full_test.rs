use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType};

#[test]
#[ignore = "not ready yet"]
fn queries_file() {
    let bqsql = include_str!("query_files/queries.bqsql");

    let document = BqsqlDocument::parse(bqsql);

    assert_eq!(97, document.items.len());
}

#[test]
#[ignore = "not ready yet"]
fn queries_file_split() {
    let bqsql = include_str!("query_files/queries.bqsql");

    let split = bqsql.split(";");
    for s in split {
        let document = BqsqlDocument::parse(s);

        if document.items.len() > 1 {
            println!("{}", &s);
        }

        assert_eq!(1, document.items.len());
    }
}
