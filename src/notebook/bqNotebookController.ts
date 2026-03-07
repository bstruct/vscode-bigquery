import * as vscode from 'vscode';
import { Job } from '@google-cloud/bigquery';
import { getBigQueryClient } from '../extensionCommands';

export class BqNotebookController {

    private static readonly CONTROLLER_ID = 'bqnb-controller';
    private static readonly NOTEBOOK_TYPE = 'bqnb';
    private static readonly LABEL = 'BigQuery';

    private readonly controller: vscode.NotebookController;

    constructor() {
        this.controller = vscode.notebooks.createNotebookController(
            BqNotebookController.CONTROLLER_ID,
            BqNotebookController.NOTEBOOK_TYPE,
            BqNotebookController.LABEL
        );
        this.controller.supportedLanguages = ['bqsql', 'sql'];
        this.controller.supportsExecutionOrder = true;
        this.controller.executeHandler = this.execute.bind(this);
    }

    private async execute(
        cells: vscode.NotebookCell[],
        _notebook: vscode.NotebookDocument,
        _controller: vscode.NotebookController
    ): Promise<void> {
        for (const cell of cells) {
            await this.executeCell(cell);
        }
    }

    private async executeCell(cell: vscode.NotebookCell): Promise<void> {
        const execution = this.controller.createNotebookCellExecution(cell);
        execution.start(Date.now());
        await execution.clearOutput();

        try {
            const queryText = cell.document.getText();
            const bqClient = await getBigQueryClient();
            const job = await bqClient.runQuery(queryText);

            // Poll until the job is complete
            let metadata = job.metadata;
            while (!metadata || metadata.status?.state !== 'DONE') {
                await new Promise(r => setTimeout(r, 1000));
                [metadata] = await job.getMetadata();
            }
            job.metadata = metadata;

            // Check for job-level errors
            if (metadata.status?.errorResult) {
                const err = metadata.status.errorResult;
                await execution.replaceOutput([
                    new vscode.NotebookCellOutput([
                        vscode.NotebookCellOutputItem.error({
                            name: err.reason || 'BigQueryError',
                            message: err.message || 'Query failed',
                        })
                    ])
                ]);
                execution.end(false, Date.now());
                return;
            }

            const statementType: string = metadata.statistics?.query?.statementType || '';

            if (statementType === 'SCRIPT') {
                await this.handleScriptJob(execution, bqClient, job);
            } else {
                await this.handleSingleJob(execution, job);
            }

            execution.end(true, Date.now());

        } catch (err: any) {
            await execution.replaceOutput([
                new vscode.NotebookCellOutput([
                    vscode.NotebookCellOutputItem.error({
                        name: 'BigQueryError',
                        message: err.message || String(err),
                    })
                ])
            ]);
            execution.end(false, Date.now());
        }
    }

    private async handleSingleJob(
        execution: vscode.NotebookCellExecution,
        job: Job
    ): Promise<void> {
        const statementType: string = job.metadata?.statistics?.query?.statementType || '';

        if (!this.isSelectType(statementType)) {
            // DML/DDL: show affected rows count
            const affectedRows = job.metadata?.statistics?.query?.numDmlAffectedRows ?? null;
            const msg = affectedRows !== null
                ? `Statement completed. Rows affected: ${affectedRows}`
                : 'Statement completed successfully.';
            await execution.replaceOutput([
                new vscode.NotebookCellOutput([
                    vscode.NotebookCellOutputItem.text(
                        `<p style="font-style:italic;color:gray">${escapeHtml(msg)}</p>`,
                        'text/html'
                    )
                ])
            ]);
            return;
        }

        const [rows] = await job.getQueryResults({ autoPaginate: true });
        await execution.replaceOutput([
            new vscode.NotebookCellOutput([
                vscode.NotebookCellOutputItem.text(rowsToHtml(rows), 'text/html')
            ])
        ]);
    }

    private async handleScriptJob(
        execution: vscode.NotebookCellExecution,
        bqClient: Awaited<ReturnType<typeof getBigQueryClient>>,
        parentJob: Job
    ): Promise<void> {
        const [childJobs] = await bqClient.bigQuery.getJobs({ parentJobId: parentJob.id });

        const sortedJobs = (childJobs as Job[]).sort((a, b) => {
            const id1 = a.id || '';
            const id2 = b.id || '';
            const n1 = Number(id1.substring(id1.lastIndexOf('_') + 1));
            const n2 = Number(id2.substring(id2.lastIndexOf('_') + 1));
            return n1 - n2;
        });

        const outputs: vscode.NotebookCellOutput[] = [];

        for (const childJob of sortedJobs) {
            const childStatementType: string = childJob.metadata?.statistics?.query?.statementType || '';

            if (this.isSelectType(childStatementType)) {
                const [rows] = await childJob.getQueryResults({ autoPaginate: true });
                outputs.push(new vscode.NotebookCellOutput([
                    vscode.NotebookCellOutputItem.text(rowsToHtml(rows), 'text/html')
                ]));
            } else if (childStatementType) {
                // DML/DDL child statement: show affected rows
                const affectedRows = childJob.metadata?.statistics?.query?.numDmlAffectedRows ?? null;
                const msg = affectedRows !== null
                    ? `Statement completed. Rows affected: ${affectedRows}`
                    : 'Statement completed successfully.';
                outputs.push(new vscode.NotebookCellOutput([
                    vscode.NotebookCellOutputItem.text(
                        `<p style="font-style:italic;color:gray">${escapeHtml(msg)}</p>`,
                        'text/html'
                    )
                ]));
            }
        }

        if (outputs.length > 0) {
            await execution.replaceOutput(outputs);
        }
    }

    private isSelectType(statementType: string): boolean {
        return statementType === 'SELECT' || statementType === 'CREATE_TABLE_AS_SELECT';
    }

    dispose(): void {
        this.controller.dispose();
    }
}

function rowsToHtml(rows: any[]): string {
    if (!rows || rows.length === 0) {
        return '<p style="font-style:italic;color:gray">Query returned no results.</p>';
    }

    const columns = Object.keys(rows[0]);
    const headerHtml = columns.map(c => `<th>${escapeHtml(c)}</th>`).join('');
    const rowsHtml = rows.map(row => {
        const cells = columns.map(c => `<td>${escapeHtml(String(row[c] ?? ''))}</td>`).join('');
        return `<tr>${cells}</tr>`;
    }).join('');

    return `<style>
table{border-collapse:collapse;font-family:monospace;font-size:12px}
th{background:#444;color:#fff;padding:4px 8px;text-align:left}
td{padding:4px 8px;border-bottom:1px solid #333}
tr:nth-child(even) td{background:#1e1e1e}
</style>
<table>
<thead><tr>${headerHtml}</tr></thead>
<tbody>${rowsHtml}</tbody>
</table>`;
}

function escapeHtml(str: string): string {
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}
