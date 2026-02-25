import { CancellationToken, DocumentSemanticTokensProvider, Event, Position, Range, ProviderResult, SemanticTokens, SemanticTokensBuilder, TextDocument, SemanticTokensLegend } from "vscode";
import { bigqueryTableSchemaService } from "../extension";
import { BqsqlTsParser, tokenize, extractTableRefs } from "./bqsqlTsParser";

export class BqsqlDocumentSemanticTokensProvider implements DocumentSemanticTokensProvider {

    onDidChangeSemanticTokens?: Event<void> | undefined;

    provideDocumentSemanticTokens(document: TextDocument, _token: CancellationToken): ProviderResult<SemanticTokens> {

        const sql = document.getText();
        const tokens = tokenize(sql);

        // Pre-load schemas for all table references in the background
        const refs = extractTableRefs(tokens);
        for (const ref of refs) {
            bigqueryTableSchemaService
                .preLoadSchemaByFullName(ref.fullName)
                .catch(ex => console.error('schema preload error', ex));
        }

        // Build semantic tokens from the tokenizer output
        const builder = new SemanticTokensBuilder(BqsqlDocumentSemanticTokensProvider.getSemanticTokensLegend());

        for (const tok of tokens) {
            if (tok.type === 'comment') { continue; }  // comments have their own grammar colour

            let tokenTypeName: string | null = null;

            switch (tok.type) {
                case 'keyword':   tokenTypeName = 'keyword'; break;
                case 'number':    tokenTypeName = 'number'; break;
                case 'string':    tokenTypeName = 'string'; break;
                case 'operator':  tokenTypeName = 'operator'; break;
                case 'backtick':  tokenTypeName = 'string'; break;  // treat as string visually
                default: break;
            }

            if (tokenTypeName) {
                const range = new Range(
                    new Position(tok.line, tok.startChar),
                    new Position(tok.line, tok.endChar),
                );
                builder.push(range, tokenTypeName, []);
            }
        }

        return builder.build();
    }

    static getSemanticTokensLegend(): SemanticTokensLegend {
        const tokenTypes = ['comment', 'string', 'keyword', 'number', 'operator', 'type', 'function', 'method'];
        const tokenModifiers: string[] = [];
        return new SemanticTokensLegend(tokenTypes, tokenModifiers);
    }
}
