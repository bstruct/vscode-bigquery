import * as vscode from 'vscode';

/**
 * Shared output channel for BigQuery SQL diagnostics.
 * Visible in VS Code under View → Output → BigQuery SQL.
 */
const channel = vscode.window.createOutputChannel('BigQuery SQL');
let shownOnce = false;

export const outputChannel = {
    appendLine(msg: string): void {
        if (!shownOnce) {
            shownOnce = true;
            channel.show(true); // reveal the panel, keep focus in editor
        }
        channel.appendLine(msg);
    },
};
