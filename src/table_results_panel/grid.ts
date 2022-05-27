import { BigQueryDate, SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import bigquery from '@google-cloud/bigquery/build/src/types';
import * as preact from 'preact';
import * as p from 'preact-render-to-string';


// import { DataGridCell, DataGrid } from '@vscode/webview-ui-toolkit';

export class Grid extends Object {

    private simpleQueryRowsResponse: SimpleQueryRowsResponse;

    /**
     *
     */
    constructor(simpleQueryRowsResponse: SimpleQueryRowsResponse) {
        super();
        this.simpleQueryRowsResponse = simpleQueryRowsResponse;
    }

    private render(): preact.VNode {

        const headerCellStyle = 'background-color: var(--list-hover-background);';

        // const x = new DataGrid();
        // x.cellType = 'rowheader'

        //https://github.com/microsoft/vscode-webview-ui-toolkit/issues/313

        //remove type
        const response: any = this.simpleQueryRowsResponse;

        const queryResults: bigquery.IGetQueryResultsResponse = response[2];

        const schema: bigquery.ITableSchema | null = queryResults.schema || null;
        if (!schema) { throw Error('Unexpected query result'); }

        const fields: bigquery.ITableFieldSchema[] = schema.fields || [];
        const fieldNames: string[] = fields?.map(c => c.name || '') || [];

        //cells that contain the top level schema column names
        function getHeaderCells() {
            const cells: preact.VNode[] = [preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '1' }, 'Row')];
            for (let fieldIndex = 0; fieldIndex < fieldNames.length; fieldIndex++) {
                const fieldName = fieldNames[fieldIndex];
                const cell = preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': (fieldIndex + 2).toString() }, fieldName);
                cells.push(cell);
            }
            return cells;
        }


        //initialize rows array with the header column row already
        const rows = [];
        rows.push(preact.h('vscode-data-grid-row', { 'row-type': 'header' }, getHeaderCells()));

        //widths of the columns
        const widths: number[] = [5];
        widths.push(...fieldNames.map(c => c.length));

        //give the necessary with to columns that contain bigger values. max 50 (`em` later added)
        function updateCellWith(widthIndex: number, valueString: string | null) {
            if (valueString != null) {
                const currentWidth = widths[widthIndex];
                if (valueString?.length && valueString.length > currentWidth) {
                    widths[widthIndex] = Math.min(50, valueString.length);
                }
            }
        }

        //
        const results: any[] = response[0];
        let rowNumber = 1;
        for (let resultIndex = 0; resultIndex < results.length; resultIndex++) {

            const cells = [];
            cells.push(preact.h('vscode-data-grid-cell', { 'style': headerCellStyle, 'grid-column': '1' }, (rowNumber++).toString()));

            const result: any = results[resultIndex];
            for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex++) {
                const field = fields[fieldIndex];

                let value: any = result[field.name || ''];

                switch (field.type || 'STRING') {
                    case 'DATE':
                        if (value != null) {
                            const date = value as BigQueryDate;
                            value = date.value;
                        }
                        break;
                    default:
                        console.info(`field ${field.name} has type ${field.type}`);
                        break;
                }

                updateCellWith(fieldIndex + 1, value);

                const cell = preact.h('vscode-data-grid-cell', { 'grid-column': (fieldIndex + 2).toString() }, value || '');
                cells.push(cell);
            }

            updateCellWith(0, (rowNumber).toString());

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        return preact.h('vscode-data-grid', { 'generate-header': 'sticky', 'grid-template-columns': widths.map(c => `${c}em`).join(' ') }, rows);
    }

    override toString(): string {
        return p.render(this.render());
    }

}
