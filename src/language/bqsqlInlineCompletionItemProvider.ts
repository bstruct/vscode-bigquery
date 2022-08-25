import { suggest } from "@bstruct/bqsql-parser";
import { CancellationToken, CompletionList, InlineCompletionContext, InlineCompletionItem, InlineCompletionItemProvider, InlineCompletionList, Position, ProviderResult, TextDocument } from "vscode";
import { BqsqlSuggestion } from "./bqsqlSuggestion";


export class BqsqlInlineCompletionItemProvider implements InlineCompletionItemProvider {

    provideInlineCompletionItems(document: TextDocument, position: Position, context: InlineCompletionContext, token: CancellationToken): ProviderResult<InlineCompletionItem[] | InlineCompletionList> {

        // const suggestions = suggest(document.getText(), position.line, position.character) as BqsqlSuggestion[];

        const list: InlineCompletionItem[] = [];

        // if (suggestions.length > 0) {
        //     for (let index = 0; index < suggestions.length; index++) {
        //         const element = suggestions[index];

        //         if (element.suggestion_type === 'TableColumns') {
        //             // debugger;
        //             list.push(new InlineCompletionItem('abc'));
        //             list.push(new InlineCompletionItem('cde'));
        //             list.push(new InlineCompletionItem('efg'));

        //         }

        //         if (element.suggestion_type === 'Function') {
        //             for (let j = 0; j < element.snippets.length; j++) {
        //                 const func = element.snippets[j];

        //                 const fn = new InlineCompletionItem(func.name);
        //                 // fn.insertText = new vscode.SnippetString(func.snippet);
        //                 // fn.documentation = new MarkdownString('#### Description\nReturns a random universally unique identifier (UUID) as a `STRING`.\nThe returned STRING consists of 32 hexadecimal digits in five groups separated by hyphens in the form 8-4-4-4-12. The hexadecimal digits represent 122 random bits and 6 fixed bits, in compliance with [RFC 4122 section 4.4](https://tools.ietf.org/html/rfc4122#section-4.4). The returned STRING is lowercase.\n#### Return Data Type\nSTRING');
        //                 list.push(fn);
        //             }

        //         }

        //     }
        // }

        return list;
    }

}