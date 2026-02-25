import * as vscode from 'vscode';
import * as fs from 'fs';
import * as os from 'os';
import { JobReference } from '../services/queryResultsMapping';
import { BigQueryClient } from '../services/bigqueryClient';
import { BigQueryInt, Table } from '@google-cloud/bigquery';

export class DownloadJsonl {

    static async downloadTable(bigqueryClient: BigQueryClient, table: Table) {

        try {

            const date = new Date();
            const dateStr = `${date.getFullYear()}${(date.getMonth() + 1).toString().padStart(2, '0')}${date.getDate().toString().padStart(2, '0')}`;
            const timeStr = `${date.getHours().toString().padStart(2, '0')}${date.getMinutes().toString().padStart(2, '0')}${date.getSeconds().toString().padStart(2, '0')}`;
            const filename = `${dateStr}${timeStr}_${table.dataset.projectId}_${table.dataset.id}_${table.id}.jsonl`;

            //download start
            const baseFolder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0
                ? vscode.workspace.workspaceFolders[0].uri
                : vscode.Uri.file(os.homedir());
            const defaultUri = vscode.Uri.joinPath(baseFolder, filename);

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
                        title: `Downloading results into:\n${fsPath}`
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

                            const rows = (await table.getRows({ startIndex: startIndex.toString(), maxResults: 10000, wrapIntegers: true }))[0];

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
            const dateStr = `${date.getFullYear()}${(date.getMonth() + 1).toString().padStart(2, '0')}${date.getDate().toString().padStart(2, '0')}`;
            const timeStr = `${date.getHours().toString().padStart(2, '0')}${date.getMinutes().toString().padStart(2, '0')}${date.getSeconds().toString().padStart(2, '0')}`;
            const filename = `${dateStr}${timeStr}_${job.id}.jsonl`;

            //download start
            const baseFolder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0
                ? vscode.workspace.workspaceFolders[0].uri
                : vscode.Uri.file(os.homedir());
            const defaultUri = vscode.Uri.joinPath(baseFolder, filename);

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
                        title: `Downloading results into:\n${fsPath}`
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

                            let queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000, wrapIntegers: true });
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

                                queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000, pageToken: pageToken, wrapIntegers: true });
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

    private static serializeRow(obj: any): string {
        // BigQueryInt values must be emitted as raw JSON numbers (not JS numbers,
        // which lose precision for values outside Number.MAX_SAFE_INTEGER).
        // Strategy: replace BigQueryInt instances with a unique placeholder string
        // during JSON.stringify, then swap the quoted placeholder for the raw digit
        // string so the number appears unquoted in the final JSON.
        const placeholders: string[] = [];

        const json = JSON.stringify(obj, (_key, value) => {
            if (value instanceof BigQueryInt || (value && typeof value === 'object' && value.type === 'BigQueryInt' && typeof value.value === 'string')) {
                // INTEGER: emit as raw JSON number to preserve full precision
                const idx = placeholders.length;
                placeholders.push(value.value); // exact decimal string
                return `__BIGINT_${idx}__`;
            }
            // BigQueryTimestamp / BigQueryDate / BigQueryDatetime / BigQueryTime
            // all serialize via toJSON() to a plain { value: string } object.
            // Unwrap to just the string so the output is a proper JSON string, not a nested object.
            if (value && typeof value === 'object' && typeof value.value === 'string' && Object.keys(value).length === 1) {
                return value.value;
            }
            return value;
        });

        // Replace `"__BIGINT_0__"` (with surrounding quotes) with the raw number string.
        return json.replace(/"__BIGINT_(\d+)__"/g, (_match, idx) => placeholders[parseInt(idx, 10)]);
    }

    private static objectsToString(records: any[]): string[] {

        let adjustedRecords = [];

        for (let i = 0; i < records.length; i++) {
            const iItem = records[i];
            let newItem: string = DownloadJsonl.serializeRow(iItem);
            adjustedRecords.push(newItem);
        }

        return adjustedRecords;
    }

}