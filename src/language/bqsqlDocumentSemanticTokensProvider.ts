import { parse } from "@bstruct/bqsql-parser";
import { CancellationToken, DocumentSemanticTokensProvider, Event, Position, Range, ProviderResult, SemanticTokens, SemanticTokensBuilder, TextDocument, SemanticTokensLegend } from "vscode";
import { BqsqlDocument, BqsqlDocumentItem } from "./bqsqlDocument";

export class BqsqlDocumentSemanticTokensProvider implements DocumentSemanticTokensProvider {

    onDidChangeSemanticTokens?: Event<void> | undefined;

    provideDocumentSemanticTokens(document: TextDocument, token: CancellationToken): ProviderResult<SemanticTokens> {

        const tokensBuilder = new SemanticTokensBuilder(BqsqlDocumentSemanticTokensProvider.getSemanticTokensLegend());

        const parsed = parse(document.getText()) as BqsqlDocument;

        this.buildTokens(tokensBuilder, parsed.items);

        return tokensBuilder.build();

    }

    static getSemanticTokensLegend(): SemanticTokensLegend {
        const tokenTypes = ['comment', 'string', 'keyword', 'number', 'operator', 'type', 'function', 'method'];
        const tokenModifiers: string[] = [];
        return new SemanticTokensLegend(
            tokenTypes,
            tokenModifiers
        );
    }

    buildTokens(tokensBuilder: SemanticTokensBuilder, items: BqsqlDocumentItem[]) {
        for (let index = 0; index < items.length; index++) {
            const element = items[index];
            if (element.range && element.range.length > 0) {
                const range = new Range(new Position(element.range[0], element.range[1]), new Position(element.range[0], element.range[2]));
                if (element.item_type === 'Keyword' || element.item_type === 'KeywordAs') {
                    tokensBuilder.push(range, 'keyword', []);
                } else {
                    if (element.item_type === 'Number') {
                        tokensBuilder.push(range, 'number', []);
                    } else {
                        if (element.item_type === 'Operator') {
                            tokensBuilder.push(range, 'operator', []);
                        }
                    }
                }
            }

            if (element.items.length > 0) {
                this.buildTokens(tokensBuilder, element.items);
            }
        }
    }

}