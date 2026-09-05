import * as vscode from 'vscode';
import { format, KeywordCase } from 'sql-formatter';

/**
 * "Format Document" / "Format Selection" for `.bqsql` files, backed by
 * sql-formatter's BigQuery dialect.
 */
export class BqsqlFormattingProvider
    implements vscode.DocumentFormattingEditProvider, vscode.DocumentRangeFormattingEditProvider {

    provideDocumentFormattingEdits(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions,
    ): vscode.TextEdit[] {
        const lastLine = document.lineAt(document.lineCount - 1);
        const fullRange = new vscode.Range(new vscode.Position(0, 0), lastLine.range.end);
        return this.formatRange(document, fullRange, options);
    }

    provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions,
    ): vscode.TextEdit[] {
        return this.formatRange(document, range, options);
    }

    private formatRange(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions,
    ): vscode.TextEdit[] {
        const original = document.getText(range);
        if (original.trim().length === 0) {
            return [];
        }

        const keywordCase = vscode.workspace
            .getConfiguration('vscode-bigquery')
            .get<KeywordCase>('format.keyword-case', 'upper');

        let formatted: string;
        try {
            formatted = format(original, {
                language: 'bigquery',
                tabWidth: options.tabSize,
                useTabs: !options.insertSpaces,
                keywordCase: keywordCase,
                dataTypeCase: keywordCase,
                functionCase: keywordCase,
                linesBetweenQueries: 2,
            });
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            vscode.window.showWarningMessage(`BigQuery SQL: could not format the document. ${message}`);
            return [];
        }

        // sql-formatter drops the trailing newline; keep whatever the file had.
        if (original.endsWith('\n') && !formatted.endsWith('\n')) {
            formatted += '\n';
        }

        if (formatted === original) {
            return [];
        }
        return [vscode.TextEdit.replace(range, formatted)];
    }
}
