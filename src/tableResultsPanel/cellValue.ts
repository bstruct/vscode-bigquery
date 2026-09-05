import * as vscode from 'vscode';

/**
 * Open the full value of a grid cell in a new (untitled) editor tab.
 *
 * The text is opened verbatim: JSON-looking values get the `json` language so
 * VS Code can highlight/fold/format them, but they are not re-serialised, which
 * would silently lose precision on large integers.
 */
export async function openCellValueInEditor(text: string): Promise<void> {
    const language = looksLikeJson(text) ? 'json' : 'plaintext';
    const document = await vscode.workspace.openTextDocument({ content: text, language });
    await vscode.window.showTextDocument(document, { preview: false, preserveFocus: false });
}

/** Copy the full value of a grid cell to the system clipboard. */
export async function copyCellValueToClipboard(text: string): Promise<void> {
    await vscode.env.clipboard.writeText(text);
    vscode.window.setStatusBarMessage('BigQuery: cell value copied to the clipboard', 2000);
}

function looksLikeJson(text: string): boolean {
    const trimmed = text.trim();
    const isObject = trimmed.startsWith('{') && trimmed.endsWith('}');
    const isArray = trimmed.startsWith('[') && trimmed.endsWith(']');
    if (!isObject && !isArray) {
        return false;
    }
    try {
        JSON.parse(trimmed);
        return true;
    } catch {
        return false;
    }
}
