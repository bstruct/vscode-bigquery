import * as vscode from 'vscode';
import { TextDecoder, TextEncoder } from 'util';

/**
 * Interface representing the JSON structure of a .bqnb file.
 */
interface BqnbData {
    cells: BqnbCell[];
}

interface BqnbCell {
    kind: vscode.NotebookCellKind;
    value: string;
    languageId: string;
}

export class BqnbSerializer implements vscode.NotebookSerializer {
    public async deserializeNotebook(
        content: Uint8Array,
        _token: vscode.CancellationToken
    ): Promise<vscode.NotebookData> {
        const contents = new TextDecoder().decode(content);

        let raw: BqnbData | undefined;
        try {
            raw = <BqnbData>JSON.parse(contents);
        } catch {
            // fallback if file is empty or corrupted
            raw = undefined;
        }

        if (!raw || !raw.cells || raw.cells.length === 0) {
            // Provide a default empty cell if file is brand new
            return new vscode.NotebookData([
                new vscode.NotebookCellData(vscode.NotebookCellKind.Code, '', 'bqsql')
            ]);
        }

        const cells = raw.cells.map(
            item => new vscode.NotebookCellData(item.kind, item.value, item.languageId)
        );

        return new vscode.NotebookData(cells);
    }

    public async serializeNotebook(
        data: vscode.NotebookData,
        _token: vscode.CancellationToken
    ): Promise<Uint8Array> {
        const contents: BqnbData = { cells: [] };

        for (const cell of data.cells) {
            contents.cells.push({
                kind: cell.kind,
                languageId: cell.languageId,
                value: cell.value
            });
        }

        return new TextEncoder().encode(JSON.stringify(contents, null, 2));
    }
}
