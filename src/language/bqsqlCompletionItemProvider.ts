import * as vscode from 'vscode';
import { CompletionItemProvider, CompletionItem, CancellationToken, CompletionContext, CompletionList, Position, ProviderResult, TextDocument, CompletionItemKind, MarkdownString, TextEdit } from 'vscode';


export class BqsqlCompletionItemProvider implements CompletionItemProvider<CompletionItem> {

    provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): ProviderResult<CompletionItem[] | CompletionList<CompletionItem>> {

        const list = new CompletionList<CompletionItem>();

        list.items.push(new CompletionItem("columnA", CompletionItemKind.Field));
        list.items.push(new CompletionItem("columnB", CompletionItemKind.Field));
        list.items.push(new CompletionItem("columnC", CompletionItemKind.Field));

        return list;
    }

    private getBqFunctions() {
        const list = new CompletionList<CompletionItem>();

        list.items.push(new CompletionItem("CURRENT_TIME", CompletionItemKind.Function));
        list.items.push(new CompletionItem("TIME", CompletionItemKind.Function));

        const generateUuid = new CompletionItem("GENERATE_UUID", CompletionItemKind.Function);
        generateUuid.insertText = new vscode.SnippetString('GENERATE_UUID() AS ${0:uuid},');
        generateUuid.documentation = new MarkdownString('#### Description\nReturns a random universally unique identifier (UUID) as a `STRING`.\nThe returned STRING consists of 32 hexadecimal digits in five groups separated by hyphens in the form 8-4-4-4-12. The hexadecimal digits represent 122 random bits and 6 fixed bits, in compliance with [RFC 4122 section 4.4](https://tools.ietf.org/html/rfc4122#section-4.4). The returned STRING is lowercase.\n#### Return Data Type\nSTRING');
        list.items.push(generateUuid);

        const rowNumber = new CompletionItem("ROW_NUMBER", CompletionItemKind.Function);
        rowNumber.insertText = new vscode.SnippetString('ROW_NUMBER() OVER (PARTITION BY ${1:column1} ORDER BY ${2:column2} ${3:ASC}) AS ${4:row_number},');
        rowNumber.documentation = new MarkdownString('### Description\nDoes not require the ORDER BY clause. Returns the sequential row ordinal (1-based) of each row for each ordered partition. If the ORDER BY clause is unspecified then the result is non-deterministic.');
        list.items.push(rowNumber);

        return list;
    }

}