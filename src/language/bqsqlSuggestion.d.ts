import { BqsqlDocumentItem } from "./bqsqlDocument";

export interface BqsqlSuggestion {
    suggestion_type: string;
    table_identifier: BqsqlDocumentItem;
    snippets: BqsqlSnippet[]
}

export interface BqsqlSnippet {
    name: string,
    snippet: string,
    url: string
}
