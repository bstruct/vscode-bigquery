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

        const documentContent = document.getText();
        const parsed = parse(documentContent) as BqsqlDocument;

        function createDiagnostic(errorItem: BigqueryJobErrorItem): vscode.Diagnostic | null {

            if (errorItem.reason === 'notFound') {

                return findMissingTableIdentifier(documentContent, parsed.items, errorItem);

            } else {
                const reg = new RegExp(/at \[(\d+):(\d+)\]$/g);
                const matches = errorItem.message.matchAll(reg);
                let item = matches.next();
                if ((!item.done) && item.value) {
                    const p1 = Number.parseInt(item.value[1]);
                    const p2 = Number.parseInt(item.value[2]);

                    const errorDocumentItem = findDocumentItem(parsed.items, p1, p2);
                    if (errorDocumentItem !== null) {
                        return new vscode.Diagnostic(
                            new vscode.Range(errorDocumentItem.range[0], errorDocumentItem.range[1], errorDocumentItem.range[0], errorDocumentItem.range[2]),
                            errorItem.message,
                            vscode.DiagnosticSeverity.Error
                        );
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

function findMissingTableIdentifier(documentContent: string, items: BqsqlDocumentItem[], errorItem: BigqueryJobErrorItem): vscode.Diagnostic | null {
    //Not found: Table damiao-project-1:PvhTest.PimExportw was not found in location EU'

    const message = errorItem.message;

    if (message.startsWith('Not found: Table ')) {

        const nextIndex = message.indexOf(' was not found in');

        if (nextIndex > 0) {

            const tableNameNotFound = message.substring(17, nextIndex);

            const lines = documentContent.split('\n');

            const sp = tableNameNotFound.split(':');
            const projectId = sp[0];
            const sp1 = sp[1].split('.');
            const datasetId = sp1[0];
            const tableId = sp1[1];

            function getString(item: BqsqlDocumentItem): string | null {

                if (item.range) {
                    try {
                        return lines[item.range[0]].substring(item.range[1], item.range[2]);
                    } catch { }
                }

                return null;
            }

            function findTableIdentifier(items: BqsqlDocumentItem[]): BqsqlDocumentItem | null {
                for (let index = 0; index < items.length; index++) {
                    const element = items[index];

                    let testElement = element.range === undefined;
                    testElement = testElement && element.item_type === 'TableIdentifier';
                    if (testElement) {

                        //TableIdentifierProjectId
                        const qTableIdentifierProjectId = element.items.find(c => c.item_type === 'TableIdentifierProjectId');
                        if (qTableIdentifierProjectId) {
                            const foundProjectId = getString(qTableIdentifierProjectId);
                            if (foundProjectId !== projectId) {
                                testElement = false;
                            }
                        }

                        //TableIdentifierDatasetId
                        const qTableIdentifierDatasetId = element.items.find(c => c.item_type === 'TableIdentifierDatasetId');
                        if (testElement && qTableIdentifierDatasetId) {
                            const foundDatasetId = getString(qTableIdentifierDatasetId);
                            if (foundDatasetId !== datasetId) {
                                testElement = false;
                            }
                        }

                        //TableIdentifierTableId
                        const qTableIdentifierTableId = element.items.find(c => c.item_type === 'TableIdentifierTableId');
                        if (testElement && qTableIdentifierTableId) {
                            const foundTableId = getString(qTableIdentifierTableId);
                            if (foundTableId !== tableId) {
                                testElement = false;
                            }
                        }

                        //TableIdentifierProjectIdDatasetIdTableId
                        const qTableIdentifierProjectIdDatasetIdTableId = element.items.find(c => c.item_type === 'TableIdentifierProjectIdDatasetIdTableId');
                        if (testElement && qTableIdentifierProjectIdDatasetIdTableId) {
                            let found = getString(qTableIdentifierProjectIdDatasetIdTableId);
                            if (found && found.startsWith('`')) { found = found?.substring(1, found.length - 1); }
                            const sp = found?.split('.');
                            if (sp?.length === 3) {
                                testElement = sp[0] === projectId
                                    && sp[1] === datasetId
                                    && sp[2] === tableId
                                    ;
                            }
                        }

                        //TableIdentifierProjectIdDatasetId
                        const qTableIdentifierProjectIdDatasetId = element.items.find(c => c.item_type === 'TableIdentifierProjectIdDatasetId');
                        if (testElement && qTableIdentifierProjectIdDatasetId) {
                            let found = getString(qTableIdentifierProjectIdDatasetId);
                            if (found && found.startsWith('`')) { found = found?.substring(1, found.length - 1); }
                            const sp = found?.split('.');
                            if (sp?.length === 2) {
                                testElement = sp[0] === projectId
                                    && sp[1] === datasetId
                                    ;
                            }
                        }

                        //TableIdentifierDatasetIdTableId
                        const qTableIdentifierDatasetIdTableId = element.items.find(c => c.item_type === 'TableIdentifierDatasetIdTableId');
                        if (testElement && qTableIdentifierDatasetIdTableId) {
                            let found = getString(qTableIdentifierDatasetIdTableId);
                            if (found && found.startsWith('`')) { found = found?.substring(1, found.length - 1); }
                            const sp = found?.split('.');
                            if (sp?.length === 2) {
                                testElement = sp[0] === datasetId
                                    && sp[1] === tableId
                                    ;
                            }
                        }
                    }

                    if (testElement) {
                        return element;
                    } else {
                        if (element.items && element.items.length > 0) {
                            const e1 = findTableIdentifier(element.items);
                            if (e1) {
                                return e1;
                            }
                        }
                    }

                }
                return null;
            }

            const item = findTableIdentifier(items);
            if (item) {

                const line = item.items.filter(c => c.range).map(c => c.range[0])[0];
                const char1 = Math.min(...item.items.filter(c => c.range).map(c => c.range[1]));
                const char2 = Math.max(...item.items.filter(c => c.range).map(c => c.range[2]));

                return new vscode.Diagnostic(
                    new vscode.Range(line, char1, line, char2),
                    errorItem.message,
                    vscode.DiagnosticSeverity.Error
                );

            }

        }
    }

    return null;
}
