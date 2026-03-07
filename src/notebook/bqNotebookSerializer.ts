import * as vscode from 'vscode';

interface BqNotebookCellData {
    kind: 'code' | 'markdown';
    value: string;
    language?: string;
}

interface BqNotebookFileFormat {
    cells: BqNotebookCellData[];
}

export class BqNotebookSerializer implements vscode.NotebookSerializer {

    async deserializeNotebook(content: Uint8Array, _token: vscode.CancellationToken): Promise<vscode.NotebookData> {
        const text = new TextDecoder().decode(content);

        let notebook: BqNotebookFileFormat;
        try {
            notebook = text.trim() ? JSON.parse(text) : { cells: [] };
        } catch {
            notebook = { cells: [] };
        }

        const cells = (notebook.cells ?? []).map(cell => {
            const kind = cell.kind === 'markdown'
                ? vscode.NotebookCellKind.Markup
                : vscode.NotebookCellKind.Code;
            return new vscode.NotebookCellData(kind, cell.value ?? '', cell.language ?? 'bqsql');
        });

        return new vscode.NotebookData(cells);
    }

    async serializeNotebook(data: vscode.NotebookData, _token: vscode.CancellationToken): Promise<Uint8Array> {
        const cells: BqNotebookCellData[] = data.cells.map(cell => ({
            kind: cell.kind === vscode.NotebookCellKind.Markup ? 'markdown' : 'code',
            value: cell.value,
            language: cell.languageId,
        }));

        const notebook: BqNotebookFileFormat = { cells };
        return new TextEncoder().encode(JSON.stringify(notebook, null, 2));
    }
}
