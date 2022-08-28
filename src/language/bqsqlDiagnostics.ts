import { DiagnosticCollection, ExtensionContext } from "vscode";
import { getBigQueryClient } from "../extensionCommands";
import * as vscode from 'vscode';
import { BigqueryJobErrorItem } from "../services/bigqueryJobError";
import { parse } from "@bstruct/bqsql-parser";
import { BqsqlDocument, BqsqlDocumentItem } from "./bqsqlDocument";

export class BqsqlDiagnostics {

    private static refreshDiagnostics(document: vscode.TextDocument, bqsqlDiagnostics: vscode.DiagnosticCollection): void {

        const parsed = parse(document.getText()) as BqsqlDocument;

        function createDiagnostic(errorItem: BigqueryJobErrorItem): vscode.Diagnostic | null {

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

            return null;
        }

        //trigger query validation
        let _ = getBigQueryClient().validateQuery(document.getText())
            .then(error => {
                const diagnostics: vscode.Diagnostic[] = [];

                if (error && error.errors && error.errors.length > 0) {

                    for (let index = 0; index < error.errors.length; index++) {
                        const element = error.errors[index];
                        const diagnostic = createDiagnostic(element);
                        if (diagnostic) { diagnostics.push(diagnostic); }
                    }
                }

                bqsqlDiagnostics.set(document.uri, diagnostics);
            });

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

