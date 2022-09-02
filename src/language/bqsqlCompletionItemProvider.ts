import { suggest } from '@bstruct/bqsql-parser';
import * as vscode from 'vscode';
import { CompletionItemProvider, CompletionItem, CancellationToken, CompletionContext, CompletionList, Position, ProviderResult, TextDocument, CompletionItemKind, MarkdownString } from 'vscode';
import { bigqueryTableSchemaService } from '../extension';
import { BqsqlSuggestion } from './bqsqlSuggestion';


export class BqsqlCompletionItemProvider implements CompletionItemProvider<CompletionItem> {

    provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): vscode.CompletionList<vscode.CompletionItem> | vscode.CompletionItem[] | null | undefined {

        const suggestions = suggest(document.getText(), position.line, position.character) as BqsqlSuggestion[];

        const list = new CompletionList<CompletionItem>();

        function pad(num: number | string) {
            var s = "0000" + num;
            return s.substring(s.length - 4);
        }

        if (suggestions.length > 0) {
            for (let index0 = 0; index0 < suggestions.length; index0++) {
                const element = suggestions[index0];

                if (element.suggestion_type === 'TableColumns') {

                    const bqsql = document.getText();

                    let schema = bigqueryTableSchemaService.getSchemaFromCache(bqsql, element.table_identifier);

                    for (let index1 = 0; index1 < schema.length; index1++) {
                        const element = schema[index1];
                        let c1 = new CompletionItem(element.column_name, CompletionItemKind.Field);
                        c1.insertText = `${element.column_name},\n`;
                        c1.detail = `${element.data_type}${element.is_partitioning_column === 'YES' ? " - PARTITION COLUMN" : ""}\n\n${element.description ? element.description : ""}`;
                        c1.command = {
                            command: "editor.action.triggerSuggest"
                        } as vscode.Command;
                        c1.sortText = pad(index0) + pad(element.ordinal_position);

                        list.items.push(c1);
                    }
                }

                if (element.suggestion_type === 'Function') {
                    for (let j = 0; j < element.snippets.length; j++) {
                        const func = element.snippets[j];

                        const fn = new CompletionItem(func.name, CompletionItemKind.Function);

                        fn.insertText = new vscode.SnippetString(func.snippet);
                        // fn.documentation = new MarkdownString('#### Description\nReturns a random universally unique identifier (UUID) as a `STRING`.\nThe returned STRING consists of 32 hexadecimal digits in five groups separated by hyphens in the form 8-4-4-4-12. The hexadecimal digits represent 122 random bits and 6 fixed bits, in compliance with [RFC 4122 section 4.4](https://tools.ietf.org/html/rfc4122#section-4.4). The returned STRING is lowercase.\n#### Return Data Type\nSTRING');
                        list.items.push(fn);
                    }

                }

            }
        }

        return list;
    }

}
