
import * as preact from 'preact';
import * as p from 'preact-render-to-string';
import { AuthenticationListItem } from '../services/authentication-list-item';

export class AuthenticationGrid extends Object {

    private items: AuthenticationListItem[];

    constructor(items: AuthenticationListItem[]) {
        super();
        this.items = items;
    }

    private render(): preact.VNode {

        const headerCellStyle = 'background-color: var(--list-hover-background);';

        //cells that contain the top level schema column names
        function getHeaderCells() {
            const cells: preact.VNode[] = [];
            const columnNames = ["account", "status", "actions"];
            for (let fieldIndex = 0; fieldIndex < columnNames.length; fieldIndex++) {
                const fieldName = columnNames[fieldIndex];
                const cell = preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': (fieldIndex + 1).toString() }, fieldName);
                cells.push(cell);
            }
            return cells;
        }

        const rows = [];
        rows.push(preact.h('vscode-data-grid-row', { 'row-type': 'header' }, getHeaderCells()));

        for (let itemIndex = 0; itemIndex < this.items.length; itemIndex++) {

            const item = this.items[itemIndex];
            const cells: preact.VNode[] = [];
            cells.push(preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '1' }, item.account));
            cells.push(preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '2' }, item.status));

            const actions: preact.VNode[] = [];
            if (item.status === '') {
                actions.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick':`vscode.postMessage({'command':'activate', 'value': '${item.account}'})` }, 'activate'));
            }
            actions.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick':`vscode.postMessage({'command':'revoke', 'value': '${item.account}'})` }, 'revoke'));

            cells.push(preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '3' }, actions));

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        const table = preact.h('vscode-data-grid', { 'generate-header': 'sticky', 'grid-template-columns': '50% 20% 30%' }, rows);

        return table;
    }

    override toString(): string {
        return p.render(this.render());
    }

}