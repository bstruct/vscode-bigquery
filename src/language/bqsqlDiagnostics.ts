import { DiagnosticCollection, ExtensionContext } from "vscode";
import { getBigQueryClient } from "../extensionCommands";
import * as vscode from 'vscode';
import { BigqueryJobErrorItem } from "../services/bigqueryJobError";
import { getStatusBarInfo } from "../extension";

export class BqsqlDiagnostics {

    private static refreshDiagnostics(document: vscode.TextDocument, bqsqlDiagnostics: vscode.DiagnosticCollection): void {

        const statusBarInfo = getStatusBarInfo();

        if (statusBarInfo) {
            statusBarInfo.text = '';
            statusBarInfo.hide();
        }

        if (document.languageId !== 'bqsql') { return; }

        const documentContent = document.getText();

        /**
         * Map a BigQuery job error to a VS Code diagnostic, using the
         * `at [line:col]` hint in the message or a text-search fallback.
         */
        function createDiagnostic(errorItem: BigqueryJobErrorItem): vscode.Diagnostic | null {

            if (errorItem.reason === 'notFound') {
                return findMissingTableIdentifier(documentContent, errorItem);
            }

            // Try to extract `at [line:col]` from the error message
            const reg = /at \[(\d+):(\d+)\]$/;
            const m = reg.exec(errorItem.message);
            if (m) {
                const p1 = Number.parseInt(m[1]);  // 1-based line
                const p2 = Number.parseInt(m[2]);  // 1-based column
                return new vscode.Diagnostic(
                    new vscode.Range(p1 - 1, p2 - 1, p1 - 1, p2 + 4),
                    errorItem.message,
                    vscode.DiagnosticSeverity.Error
                );
            }

            return null;
        }

        // Trigger query validation against BigQuery
        getBigQueryClient()
            .then(bqClient => {
                bqClient.validateQuery(document.getText())
                    .then(response => {
                        const diagnostics: vscode.Diagnostic[] = [];

                        const totalBytesProcessed = response[0];
                        const error = response[1];

                        if (totalBytesProcessed) {
                            const mb = (totalBytesProcessed / 1024 / 1024).toFixed(2);
                            if (statusBarInfo) {
                                statusBarInfo.text = `This query will process ${mb} MB when run.`;
                                statusBarInfo.show();
                            }
                        }

                        if (error?.errors?.length) {
                            for (const element of error.errors) {
                                const diagnostic = createDiagnostic(element);
                                if (diagnostic) { diagnostics.push(diagnostic); }
                            }
                        }

                        bqsqlDiagnostics.set(document.uri, diagnostics);
                    });
            })
            .catch(ex => console.error('BqsqlDiagnostics error', ex));
    }

    static subscribeToDocumentChanges(context: ExtensionContext, diagnosticsCollection: DiagnosticCollection): void {

        if (vscode.window.activeTextEditor) {
            if (vscode.window.activeTextEditor.document
                && vscode.window.activeTextEditor.document.languageId === 'bqsql') {
                BqsqlDiagnostics.refreshDiagnostics(vscode.window.activeTextEditor.document, diagnosticsCollection);
            }
        }
        context.subscriptions.push(
            vscode.window.onDidChangeActiveTextEditor(editor => {
                if (editor && editor.document && editor.document.languageId === 'bqsql') {
                    BqsqlDiagnostics.refreshDiagnostics(editor.document, diagnosticsCollection);
                }
            })
        );

        context.subscriptions.push(
            vscode.workspace.onDidChangeTextDocument(e => BqsqlDiagnostics.refreshDiagnostics(e.document, diagnosticsCollection))
        );

        context.subscriptions.push(
            vscode.workspace.onDidCloseTextDocument(doc => diagnosticsCollection.delete(doc.uri))
        );

    }
}

/**
 * Try to locate a missing table in the source text using the name from the
 * BigQuery error message ("Not found: Table project:dataset.table was not found...").
 */
function findMissingTableIdentifier(
    documentContent: string,
    errorItem: BigqueryJobErrorItem,
): vscode.Diagnostic | null {

    const message = errorItem.message;

    // "Not found: Table damiao-project-1:PvhTest.PimExport was not found in location EU"
    if (!message.startsWith('Not found: Table ')) { return null; }

    const nextIndex = message.indexOf(' was not found in');
    if (nextIndex <= 0) { return null; }

    const tableNameNotFound = message.substring(17, nextIndex);  // e.g. "project:dataset.table"

    // Normalise to  project.dataset.table  (BigQuery uses ':' as project separator)
    const normalised = tableNameNotFound.replace(':', '.');
    const parts = normalised.split('.');
    if (parts.length < 2) { return null; }

    const datasetId = parts[parts.length - 2];
    const tableId = parts[parts.length - 1];
    const tableIdentifier = `${datasetId}.${tableId}`;

    const lines = documentContent.split('\n');
    const lineIndex = lines.findIndex(l => l.includes(tableIdentifier));

    if (lineIndex >= 0) {
        const charIndex = lines[lineIndex].indexOf(tableIdentifier);
        return new vscode.Diagnostic(
            new vscode.Range(lineIndex, charIndex, lineIndex, charIndex + tableIdentifier.length),
            errorItem.message,
            vscode.DiagnosticSeverity.Error
        );
    }

    // Fallback: highlight position 0
    return new vscode.Diagnostic(
        new vscode.Range(0, 0, 0, 1),
        errorItem.message,
        vscode.DiagnosticSeverity.Error
    );
}
