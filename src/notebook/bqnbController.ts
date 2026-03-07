import * as vscode from 'vscode';
import { getBigQueryClient, COMMAND_DOWNLOAD_CSV, COMMAND_DOWNLOAD_JSONL, COMMAND_SEND_PUBSUB } from '../extensionCommands';

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

        const messaging = vscode.notebooks.createRendererMessaging('bstruct-bqnb-grid');
        messaging.onDidReceiveMessage(e => {
            const msg = e.message as any;
            if (!msg?.command) { return; }
            const data = {
                command: msg.command,
                jobReference: msg.job_reference,
                tableReference: msg.table_reference,
            };
            switch (msg.command) {
                case 'download_csv': vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV, data); break;
                case 'download_jsonl': vscode.commands.executeCommand(COMMAND_DOWNLOAD_JSONL, data); break;
                case 'send_pubsub': vscode.commands.executeCommand(COMMAND_SEND_PUBSUB, data); break;
            }
        });

        // Refresh tokens for persisted outputs when a notebook is opened
        vscode.workspace.onDidOpenNotebookDocument(notebook => {
            if (notebook.notebookType !== this.notebookType) { return; }
            this._refreshOutputTokens(notebook);
        });

        // Also refresh any already-open notebooks on extension activation
        for (const notebook of vscode.workspace.notebookDocuments) {
            if (notebook.notebookType === this.notebookType) {
                this._refreshOutputTokens(notebook);
            }
        }
    }

    private async _refreshOutputTokens(notebook: vscode.NotebookDocument): Promise<void> {
        const GRID_MIME = 'application/x-bstruct-bqnb-grid';
        const cellsNeedingRefresh: vscode.NotebookCell[] = [];

        for (const cell of notebook.getCells()) {
            for (const output of cell.outputs) {
                const gridItem = output.items.find(i => i.mime === GRID_MIME);
                if (!gridItem) { continue; }
                try {
                    const payload = JSON.parse(new TextDecoder().decode(gridItem.data));
                    if (!payload.token) {
                        cellsNeedingRefresh.push(cell);
                        break;
                    }
                } catch { /* skip */ }
            }
        }

        if (cellsNeedingRefresh.length === 0) { return; }

        try {
            const bqClient = await getBigQueryClient();
            const token = await bqClient.getToken();

            for (const cell of cellsNeedingRefresh) {
                const newOutputs: vscode.NotebookCellOutput[] = [];
                for (const output of cell.outputs) {
                    const gridItem = output.items.find(i => i.mime === GRID_MIME);
                    if (!gridItem) { continue; }
                    try {
                        const payload = JSON.parse(new TextDecoder().decode(gridItem.data));
                        newOutputs.push(new vscode.NotebookCellOutput([
                            vscode.NotebookCellOutputItem.json({ ...payload, token }, GRID_MIME)
                        ]));
                    } catch { /* skip */ }
                }

                if (newOutputs.length === 0) { continue; }

                const execution = this._controller.createNotebookCellExecution(cell);
                execution.start(Date.now());
                await execution.replaceOutput(newOutputs);
                execution.end(true, Date.now());
            }
        } catch {
            // silently fail — user can re-run cells to get fresh results
        }
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
        execution.start(Date.now());

        try {
            await execution.clearOutput();

            await this._doQueryExecution(cell, execution);

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
            execution.end(false, Date.now());
        }
    }

    private async _doQueryExecution(cell: vscode.NotebookCell, execution: vscode.NotebookCellExecution): Promise<void> {
        const queryText = cell.document.getText();
        const bqClient = await getBigQueryClient();

        // Start the query job
        const job = await bqClient.runQuery(queryText);
        const jobWait: any = await job.promise();

        // We try to handle SCRIPT and straightforward executions here
        const jobMeta = jobWait[0] as any;
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
                    statementType,
                    jobMetadata: currentJob.metadata
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
        execution.end(true, Date.now());
    }

}
