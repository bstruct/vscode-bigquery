import * as vscode from 'vscode';
import { TextDecoder, TextEncoder } from 'util';

const GRID_MIME = 'application/x-bstruct-bqnb-grid';

interface BqnbData {
    cells: BqnbCell[];
}

interface BqnbCell {
    kind: vscode.NotebookCellKind;
    value: string;
    languageId: string;
    outputs?: BqnbOutputGroup[];
}

/** One NotebookCellOutput = one group of MIME items */
interface BqnbOutputGroup {
    items: BqnbOutputItem[];
}

interface BqnbOutputItem {
    mime: string;
    data: any;
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
            raw = undefined;
        }

        if (!raw || !raw.cells || raw.cells.length === 0) {
            return new vscode.NotebookData([
                new vscode.NotebookCellData(vscode.NotebookCellKind.Code, '', 'bqsql')
            ]);
        }

        const cells = raw.cells.map(item => {
            const cell = new vscode.NotebookCellData(item.kind, item.value, item.languageId);
            if (item.outputs && item.outputs.length > 0) {
                cell.outputs = item.outputs.map(group =>
                    new vscode.NotebookCellOutput(
                        group.items.map(i => vscode.NotebookCellOutputItem.json(i.data, i.mime))
                    )
                );
            }
            return cell;
        });

        return new vscode.NotebookData(cells);
    }

    public async serializeNotebook(
        data: vscode.NotebookData,
        _token: vscode.CancellationToken
    ): Promise<Uint8Array> {
        const contents: BqnbData = { cells: [] };

        for (const cell of data.cells) {
            const serializedOutputs: BqnbOutputGroup[] = [];

            for (const output of (cell.outputs || [])) {
                const gridItem = output.items.find(i => i.mime === GRID_MIME);
                if (!gridItem) { continue; }

                try {
                    const payload = JSON.parse(new TextDecoder().decode(gridItem.data));
                    // Strip token — it expires; controller will refresh on open
                    const { token: _t, ...payloadWithoutToken } = payload;
                    serializedOutputs.push({
                        items: [{ mime: GRID_MIME, data: payloadWithoutToken }]
                    });
                } catch {
                    // skip malformed output
                }
            }

            const cellData: BqnbCell = {
                kind: cell.kind,
                languageId: cell.languageId,
                value: cell.value,
            };
            if (serializedOutputs.length > 0) {
                cellData.outputs = serializedOutputs;
            }
            contents.cells.push(cellData);
        }

        return new TextEncoder().encode(JSON.stringify(contents, null, 2));
    }
}
