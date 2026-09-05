import * as vscode from 'vscode';
import { BqsqlTsParser } from './bqsqlTsParser';

/**
 * Opens the suggest widget automatically after a line break inside a SELECT
 * column list of a `.bqsql` document, so the schema-aware column list shows up
 * without a keyboard shortcut (Ctrl+Space is taken by the input-source switch
 * on macOS).
 *
 * Disabled with `vscode-bigquery.suggest-on-new-line: false`.
 */
export function registerBqsqlAutoSuggest(context: vscode.ExtensionContext): void {
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId !== 'bqsql' || event.contentChanges.length !== 1) {
                return;
            }
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document !== event.document) {
                return;
            }
            const enabled = vscode.workspace
                .getConfiguration('vscode-bigquery')
                .get<boolean>('suggest-on-new-line', true);
            if (!enabled) {
                return;
            }

            // A plain line break, possibly followed by auto-indentation.
            const change = event.contentChanges[0];
            const newline = /^\r?\n([ \t]*)$/.exec(change.text);
            if (!newline) {
                return;
            }

            const line = change.range.start.line + 1;
            const column = newline[1].length;
            const selectContext = BqsqlTsParser.getSelectContext(event.document.getText(), line, column);
            if (!selectContext || selectContext.sources.length === 0) {
                return;
            }

            void vscode.commands.executeCommand('editor.action.triggerSuggest');
        })
    );
}
