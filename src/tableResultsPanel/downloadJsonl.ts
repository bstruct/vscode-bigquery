import * as vscode from 'vscode';
import * as fs from 'fs';
import { JobReference } from '../services/queryResultsMapping';
import { BigQueryClient } from '../services/bigqueryClient';
import { Table } from '@google-cloud/bigquery';

export class DownloadJsonl {

    static async downloadTable(bigqueryClient: BigQueryClient, table: Table) {

        try {

            const date = new Date();
            const filename = `${table.dataset.projectId}_${table.dataset.id}_${table.id}_${date.toLocaleTimeString().replace(/:/g, '')}.jsonl`;

            //download start
            let defaultUri: vscode.Uri | undefined;
            if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
                defaultUri = vscode.Uri.joinPath(vscode.workspace.workspaceFolders[0].uri, filename);
            }

            const uri: vscode.Uri | undefined = await vscode.window.showSaveDialog(
                {
                    title: 'Save export',
                    filters: {
                        'jsonl': ['jsonl']
                    },
                    defaultUri: defaultUri
                }
            );

            if (uri !== undefined) {
                try {
                    const fsPath = uri.fsPath;

                    await vscode.window.withProgress({
                        location: vscode.ProgressLocation.Notification,
                        cancellable: true,
                        title: `Downloading results into:\n${filename}`
                    }, async (progress, token) => {

                        let cancelled = false;

                        token.onCancellationRequested(() => {
                            cancelled = true;
                            console.log("User canceled the long running operation");
                        });

                        //resolve if job is INSERT, UPDATE, DELETE or MERGE
                        const metadata = await table.getMetadata();
                        const totalRows = Number(metadata[0].numRows || 0);

                        let increment = 10000 * 100 / totalRows;

                        let totalDownloadedRows = 0;
                        let startIndex = 0;

                        while (!token.isCancellationRequested) {

                            const rows = (await table.getRows({ startIndex: startIndex.toString(), maxResults: 10000 }))[0];

                            //transform complex objects into string
                            let adjustedRecords = DownloadJsonl.objectsToString(rows);
                            fs.appendFileSync(fsPath, adjustedRecords.join('\n'));

                            // https://github.com/microsoft/vscode-extension-samples/blob/main/progress-sample/src/extension.ts
                            progress.report({ increment: increment });

                            totalDownloadedRows += rows.length;

                            if (totalDownloadedRows >= totalRows) {
                                break;
                            }
                            startIndex += 10000;

                        }

                        progress.report({ message: 'Done' });

                    });

                } catch (error: any) {
                    vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
                }
            }

        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    public static async download(bigqueryClient: BigQueryClient, jobReference: JobReference) {

        try {

            const job = bigqueryClient.getJob(jobReference);

            const date = new Date();
            const filename = `${date.getFullYear()}${(date.getMonth() + 1).toString(2)}${date.getDay()}${date.toLocaleTimeString().replace(/:/g, '')}_${job.id}.jsonl`;

            //download start
            let defaultUri: vscode.Uri | undefined;
            if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
                defaultUri = vscode.Uri.joinPath(vscode.workspace.workspaceFolders[0].uri, filename);
            }

            const uri: vscode.Uri | undefined = await vscode.window.showSaveDialog(
                {
                    title: 'Save export',
                    filters: {
                        'jsonl': ['jsonl']
                    },
                    defaultUri: defaultUri
                }
            );

            if (uri !== undefined) {
                try {
                    const fsPath = uri.fsPath;

                    await vscode.window.withProgress({
                        location: vscode.ProgressLocation.Notification,
                        cancellable: true,
                        title: `Downloading results into:\n${filename}`
                    }, async (progress, token) => {

                        let cancelled = false;

                        token.onCancellationRequested(() => {
                            cancelled = true;
                            console.log("User canceled the long running operation");
                        });

                        //resolve if job is INSERT, UPDATE, DELETE or MERGE
                        const metadata = await job.getMetadata();
                        const statementType = metadata[0].statistics.query.statementType;
                        if (statementType === 'INSERT' || statementType === 'UPDATE' || statementType === 'DELETE' || statementType === 'MERGE') {
                            const dmlStats = metadata[0].statistics.query.dmlStats;

                            const row = {
                                insertedRowCount: dmlStats.insertedRowCount ?? null,
                                updatedRowCount: dmlStats.updatedRowCount ?? null,
                                deletedRowCount: dmlStats.deletedRowCount ?? null,
                            };

                            fs.appendFileSync(fsPath, JSON.stringify(row));

                        } else {

                            let queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000 });
                            const totalRows = Number.parseInt(queryResults[2]?.totalRows as string);

                            let records = queryResults[0];

                            let increment = 10000 * 100 / totalRows;

                            let totalDownloadedRows = 0;

                            while (!token.isCancellationRequested) {

                                //transform complex objects into string
                                let adjustedRecords = DownloadJsonl.objectsToString(records);
                                fs.appendFileSync(fsPath, adjustedRecords.join('\n'));

                                // https://github.com/microsoft/vscode-extension-samples/blob/main/progress-sample/src/extension.ts
                                progress.report({ increment: increment });

                                totalDownloadedRows += records.length;
                                const pageToken = queryResults[1]?.pageToken;

                                if (totalDownloadedRows >= totalRows || (!pageToken)) {
                                    break;
                                }

                                queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000, pageToken: pageToken });
                                records = queryResults[0];
                            }
                        }

                        progress.report({ message: 'Done' });

                    });

                } catch (error: any) {
                    vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
                }
            }


        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    private static objectsToString(records: any[]): string[] {

        let adjustedRecords = [];

        for (let i = 0; i < records.length; i++) {
            const iItem = records[i];
            let newItem: string = JSON.stringify(iItem);
            adjustedRecords.push(newItem);
        }

        return adjustedRecords;
    }

}