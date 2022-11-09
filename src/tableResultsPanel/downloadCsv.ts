import * as vscode from 'vscode';
import { DownloadCsvRequest } from "./downloadCsvRequest";
import * as fs from 'fs';
import * as csv_writer from 'csv-writer';
import { ObjectCsvStringifierParams } from 'csv-writer/src/lib/csv-stringifier-factory';

export class DownloadCsv {

    // constructor(webView: vscode.Webview) {
    //     this.webView = webView;
    // }

    public static download(downloadCsvRequest: DownloadCsvRequest) {

        try {

            downloadCsvRequest.jobsPromise.then(jobs => {

                const job = jobs[downloadCsvRequest.jobIndex];
                const date = new Date();
                const filename = `${date.getFullYear()}${(date.getMonth() + 1).toString(2)}${date.getDay()}${date.toLocaleTimeString().replace(/:/g, '')}_${job.id}.csv`;

                //download start
                let defaultUri: vscode.Uri | undefined;
                if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
                    defaultUri = vscode.Uri.joinPath(vscode.workspace.workspaceFolders[0].uri, filename);
                }
                // const baseUri= vscode.workspace.workspaceFolders[0].uri;
                // const path = vscode.workspace.asRelativePath('test.csv', true);

                vscode.window.showSaveDialog(
                    {
                        title: 'Save export',
                        filters: {
                            'csv': ['csv']
                        },
                        defaultUri: defaultUri
                    }
                ).then(async (uri: vscode.Uri | undefined) => {

                    if (uri !== undefined) {

                        vscode.window.showInformationMessage(`Initiated downloading into the file:\n${filename}`);

                        const createCsvStringifier = csv_writer.createObjectCsvStringifier;

                        // job.metadata
                        let queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 1000 });
                        const totalRows = Number.parseInt(queryResults[2]?.totalRows as string);

                        const columnNames = queryResults[2]?.schema?.fields?.filter(c => c.name && c.name.length > 0).map(c => { return { id: c.name as string, title: c.name as string }; });
                        const csvStringifier = createCsvStringifier({ header: columnNames } as ObjectCsvStringifierParams);

                        fs.writeFile(uri.path, csvStringifier.getHeaderString() as string, (err: any) => {
                            if (err) {
                                console.error(err);
                            }
                        });


                        let records = queryResults[0];
                        let totalDownloadedRows = 0;

                        while (true) {

                            //transform complex objects into string
                            let adjustedRecords = DownloadCsv.objectsToString(records);

                            fs.appendFile(uri.path, csvStringifier.stringifyRecords(adjustedRecords), (err: any) => {
                                if (err) {
                                    console.error(err);
                                }
                            });

                            totalDownloadedRows += records.length;
                            const pageToken = queryResults[1]?.pageToken;

                            if (totalDownloadedRows >= totalRows || (!pageToken)) {
                                break;
                            }

                            queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000, pageToken: pageToken });
                            records = queryResults[0];
                        }

                        //success message
                        vscode.window.showInformationMessage(`Download concluded:\n${filename}`);
                    }
                });

            });

        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }



    private static objectsToString(records: any[]): any[] {

        let adjustedRecords = [];

        for (let i = 0; i < records.length; i++) {
            const iItem = records[i];
            let newItem: any = {};
            for (const [key, value] of Object.entries(iItem)) {
                if (value && (value as any).value) {
                    newItem[key] = (value as any).value;
                } else {
                    if (value && typeof (value) === 'object') {
                        newItem[key] = (value as Buffer).toString('base64');
                    } else {
                        newItem[key] = value;
                    }
                }
            }
            adjustedRecords.push(newItem);
        }

        return adjustedRecords;
    }
}