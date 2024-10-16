import * as vscode from 'vscode';
import { JobReference } from '../services/queryResultsMapping';
import { BigQueryClient } from '../services/bigqueryClient';
import { QueryRowsResponse, Table, TableSchema } from '@google-cloud/bigquery';
import { PubSub } from '@google-cloud/pubsub';

export class SendToPubsub {

    public static async sendJobResult(bigqueryClient: BigQueryClient, jobReference: JobReference) {

        try {

            const job = bigqueryClient.getJob(jobReference);
            //check if job is INSERT, UPDATE, DELETE or MERGE
            const metadata = await job.getMetadata();
            const statementType = metadata[0].statistics.query.statementType;
            if (statementType === 'INSERT' || statementType === 'UPDATE' || statementType === 'DELETE' || statementType === 'MERGE') {
                vscode.window.showErrorMessage('Bigquery jobs of type `INSERT`, `UPDATE`, `DELETE`, or `MERGE` are not supported in sending to Pub/Sub.');
                return;
            }

            //check for `body` column 
            let queryResults = await job.getQueryResults({ maxResults: 1 });
            const totalRows = Number.parseInt(queryResults[2]?.totalRows as string);

            const containsAttributes = queryResults[2]?.schema?.fields?.find(c => c.name?.toLowerCase() === 'attributes' && c.type === 'RECORD');
            const containsData = queryResults[2]?.schema?.fields?.find(c => c.name?.toLowerCase() === 'data' && (c.type === 'STRING' || c.type === 'JSON'));

            if (containsData === undefined) {
                vscode.window.showErrorMessage('Please make a STRING or JSON column called `data` to be sent to Pub/Sub.');
                return;
            }

            const topicName = await vscode.window.showInputBox({
                title: 'Pub/Sub topic (projects/<project_id>/topics/<topic_name>)',
            });

            if (topicName) {

                try {

                    // Instantiates a client
                    const pubsub = new PubSub();
                    const topic = pubsub.topic(topicName, { messageOrdering: false });
                    //check if topic exists
                    if (!topic.exists()) {
                        vscode.window.showErrorMessage('The given Pub/Sub topic name does not exist or user does not have permissions.');
                        return;
                    }

                    await vscode.window.withProgress({
                        location: vscode.ProgressLocation.Notification,
                        cancellable: true,
                        title: `Sending results into Pub/Sub`,
                    }, async (progress, token) => {

                        let cancelled = false;

                        token.onCancellationRequested(() => {
                            cancelled = true;
                            console.log("User canceled the long running operation");
                        });

                        const totalRows = Number.parseInt(queryResults[2]?.totalRows as string);

                        let increment = 10000 * 100 / totalRows;

                        let totalDownloadedRows = 0;

                        let pageToken: string | undefined = undefined;

                        while (!token.isCancellationRequested) {

                            const queryResults: QueryRowsResponse = await job.getQueryResults({ autoPaginate: true, maxResults: 10000, pageToken: pageToken });
                            const records = queryResults[0];

                            let promisses = [];
                            for (let index = 0; index < records.length; index++) {
                                const element = records[index];

                                const data = Buffer.from(element['data']);

                                let attributes = null;
                                if (containsAttributes) {
                                    attributes = element['attributes'];
                                    promisses.push(await topic.publishMessage({ data, attributes }));
                                } else {
                                    promisses.push(await topic.publishMessage({ data }));
                                }

                                if (promisses.length > 1000) {
                                    await Promise.all(promisses);
                                    promisses = [];
                                }
                            }

                            if (promisses.length > 0) {
                                await Promise.all(promisses);
                            }

                            progress.report({ increment: increment });

                            totalDownloadedRows += records.length;
                            pageToken = queryResults[1]?.pageToken;

                            if (totalDownloadedRows >= totalRows || (!pageToken)) {
                                break;
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

}