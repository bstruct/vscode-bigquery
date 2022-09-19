import { DiagnosticCollection, ExtensionContext } from "vscode";
import { getBigQueryClient } from "../extensionCommands";
import * as vscode from 'vscode';
import { BigqueryJobErrorItem } from "../services/bigqueryJobError";
import { parse } from "@bstruct/bqsql-parser";
import { BqsqlDocument, BqsqlDocumentItem } from "./bqsqlDocument";
import { statusBarInfo } from "../extension";

export class BqsqlDiagnostics {

    private static refreshDiagnostics(document: vscode.TextDocument, bqsqlDiagnostics: vscode.DiagnosticCollection): void {

        if (statusBarInfo) {
            statusBarInfo.text = '';
            statusBarInfo.hide();
        }

        const parsed = parse(document.getText()) as BqsqlDocument;

        function createDiagnostic(errorItem: BigqueryJobErrorItem): vscode.Diagnostic | null {

            if (errorItem.reason === 'notFound') {

                debugger;

            } else {
                const reg = new RegExp(/at \[(\d+):(\d+)\]$/g);
                const matches = errorItem.message.matchAll(reg);
                let item = matches.next();
                if ((!item.done) && item.value) {
                    const p1 = Number.parseInt(item.value[1]);
                    const p2 = Number.parseInt(item.value[2]);

                    const errorDocumentItem = findDocumentItem(parsed.items, p1, p2);
                    if (errorDocumentItem !== null) {
                        let diagnostic = new vscode.Diagnostic(
                            new vscode.Range(errorDocumentItem.range[0], errorDocumentItem.range[1], errorDocumentItem.range[0], errorDocumentItem.range[2]),
                            errorItem.message,
                            vscode.DiagnosticSeverity.Error
                        );

                        return diagnostic;
                    }
                }
            }

            return null;
        }

        //trigger query validation
        let _ = getBigQueryClient().validateQuery(document.getText())
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

                if (error && error.errors && error.errors.length > 0) {

                    for (let index = 0; index < error.errors.length; index++) {
                        const element = error.errors[index];
                        const diagnostic = createDiagnostic(element);
                        if (diagnostic) { diagnostics.push(diagnostic); }
                    }
                }

                bqsqlDiagnostics.set(document.uri, diagnostics);
            });

        //statusBarInfo

    }

    static subscribeToDocumentChanges(context: ExtensionContext, emojiDiagnostics: DiagnosticCollection): void {

        if (vscode.window.activeTextEditor) {
            BqsqlDiagnostics.refreshDiagnostics(vscode.window.activeTextEditor.document, emojiDiagnostics);
        }
        context.subscriptions.push(
            vscode.window.onDidChangeActiveTextEditor(editor => {
                if (editor) {
                    BqsqlDiagnostics.refreshDiagnostics(editor.document, emojiDiagnostics);
                }
            })
        );

        context.subscriptions.push(
            vscode.workspace.onDidChangeTextDocument(e => BqsqlDiagnostics.refreshDiagnostics(e.document, emojiDiagnostics))
        );

        context.subscriptions.push(
            vscode.workspace.onDidCloseTextDocument(doc => emojiDiagnostics.delete(doc.uri))
        );

    }
}

function findDocumentItem(items: BqsqlDocumentItem[], p1: number, p2: number): BqsqlDocumentItem | null {
    for (let index = 0; index < items.length; index++) {
        const element = items[index];

        if (element.range && element.range.length === 3) {
            if (element.range[0] === p1 - 1 && element.range[1] === p2 - 1) {
                return element;
            }
        } else {
            if (element.items && element.items.length > 0) {
                const e1 = findDocumentItem(element.items, p1, p2);
                if (e1) {
                    return e1;
                }
            }
        }

    }
    return null;
}

function findTableIdentifier(items: BqsqlDocumentItem[]): BqsqlDocumentItem | null {
    //Not found: Table damiao-project-1:PvhTest.PimExportw was not found in location EU'

    return null;
}