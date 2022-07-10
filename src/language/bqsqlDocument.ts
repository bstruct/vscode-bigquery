export interface BqsqlDocument {

    items: BqsqlDocumentItem[];
}

export enum BqsqlDocumentItemType {
    unknown = 0,
    comment = 1,
    query = 2,
    //Data definition language (DDL)
    //Data manipulation language (DML)
    //Data control language (DCL)
    //Procedural language
    //Debugging statements
    //Other statements in Standard SQL

}

export interface BqsqlDocumentItem {
    type: BqsqlDocumentItemType;
}

export interface BqsqlDocumentComment {
    content: string;
}