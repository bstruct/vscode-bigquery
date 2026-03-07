import * as vscode from 'vscode';
import { getBigQueryClient } from '../extensionCommands';

export class BqnbController {
    private readonly controllerId = 'bqnb-controller';
    private readonly notebookType = 'bqnb';
    private readonly label = 'BigQuery Notebook';
    private readonly supportedLanguages = ['bqsql'];

    private readonly _controller: vscode.NotebookController;
    private _executionOrder = 0;

    constructor() {
        this._controller = vscode.notebooks.createNotebookController(
            this.controllerId,
            this.notebookType,
            this.label
        );

        this._controller.supportedLanguages = this.supportedLanguages;
        this._controller.supportsExecutionOrder = true;
        this._controller.executeHandler = this._execute.bind(this);
    }

    public dispose(): void {
        this._controller.dispose();
    }

    private async _execute(
        cells: vscode.NotebookCell[],
        _notebook: vscode.NotebookDocument,
        _controller: vscode.NotebookController
    ): Promise<void> {
        for (const cell of cells) {
            await this._doExecution(cell);
        }
    }

    private async _doExecution(cell: vscode.NotebookCell): Promise<void> {
        const execution = this._controller.createNotebookCellExecution(cell);
        execution.executionOrder = ++this._executionOrder;
        execution.start(Date.now()); // Start execution timing

        try {
            await execution.clearOutput();

            const queryText = cell.document.getText();
            const bqClient = await getBigQueryClient();

            // Start the query job
            const job = await bqClient.runQuery(queryText);
            const jobWait: any = await job.promise();

            // We try to handle SCRIPT and straightforward executions here
            const jobMeta = jobWait[1] as any;
            const statementType: string = jobMeta?.statistics?.query?.statementType || '';

            let outputJobs: any[] = [];

            if (statementType === 'SCRIPT') {
                const jobId = jobMeta.jobReference?.jobId || '';
                const getJobsResponse = await bqClient.bigQuery.getJobs({ parentJobId: jobId });
                const jobs = getJobsResponse[0];

                outputJobs = jobs.sort((a, b) => {
                    const id1 = a.id || '';
                    const id2 = b.id || '';
                    const n1 = Number(id1.substring(id1.lastIndexOf('_') + 1));
                    const n2 = Number(id2.substring(id2.lastIndexOf('_') + 1));
                    return n1 > n2 ? 1 : -1;
                });

            } else {
                outputJobs = [job];
            }

            // Map jobs to outputs natively
            const outputs: vscode.NotebookCellOutput[] = [];
            const token = await bqClient.getToken();

            for (const currentJob of outputJobs) {
                try {
                    const results = await currentJob.getQueryResults();
                    const rows = results[0];

                    const jobId = currentJob.metadata?.jobReference?.jobId;
                    const projectId = currentJob.metadata?.jobReference?.projectId;
                    const location = currentJob.metadata?.jobReference?.location;
                    const statementType = currentJob.metadata?.statistics?.query?.statementType || 'SELECT';

                    const renderPayload = {
                        jobId,
                        projectId,
                        location,
                        token,
                        statementType
                    };

                    if (rows && rows.length > 0) {
                        // Convert rows into a simple HTML table for native display mapping
                        let html = '<style>table { border-collapse: collapse; } th, td { border: 1px solid var(--vscode-editor-snippetFinalTabstopHighlightBorder, gray); padding: 4px; text-align: left; }</style><table><thead><tr>';

                        const columns = Object.keys(rows[0]);
                        for (const col of columns) {
                            html += `<th>${col}</th>`;
                        }
                        html += '</tr></thead><tbody>';

                        for (const row of rows) {
                            html += '<tr>';
                            for (const col of columns) {
                                html += `<td>${row[col] !== null ? String(row[col]) : 'null'}</td>`;
                            }
                            html += '</tr>';
                        }

                        html += '</tbody></table>';

                        outputs.push(new vscode.NotebookCellOutput([
                            vscode.NotebookCellOutputItem.json(renderPayload, 'application/x-bstruct-bqnb-grid'),
                            vscode.NotebookCellOutputItem.text(html, 'text/html'),
                            vscode.NotebookCellOutputItem.json(rows, 'application/json')
                        ]));
                    } else {
                        outputs.push(new vscode.NotebookCellOutput([
                            vscode.NotebookCellOutputItem.json(renderPayload, 'application/x-bstruct-bqnb-grid'),
                            vscode.NotebookCellOutputItem.text('✅ Query executed successfully (no results).')
                        ]));
                    }
                } catch (e: any) {
                    // Job might not have results or might be a variable declaration
                    console.log('Skipping output for job', currentJob.id, e);
                }
            }

            await execution.replaceOutput(outputs);
            execution.end(true, Date.now()); // marking as success

        } catch (error: any) {
            const errorMessage = error.message || 'Error occurred';
            await execution.replaceOutput([
                new vscode.NotebookCellOutput([
                    vscode.NotebookCellOutputItem.error({
                        name: 'Execution Error',
                        message: errorMessage
                    })
                ])
            ]);
            execution.end(false, Date.now()); // marking as failure
        }
    }
}
