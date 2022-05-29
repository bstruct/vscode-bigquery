import { BigQueryDate, QueryRowsResponse, SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import bigquery from '@google-cloud/bigquery/build/src/types';
import * as preact from 'preact';
import * as p from 'preact-render-to-string';

//---------------- @vscode/webview-ui-toolkit ----------------
//https://github.com/microsoft/vscode-webview-ui-toolkit
//https://microsoft.github.io/vscode-webview-ui-toolkit/?path=/story/library-badge--default


//---------------- @vscode/codicons ----------------
//https://microsoft.github.io/vscode-codicons/dist/codicon.html

export class ResultsGrid extends Object {

    private queryRowsResponse: QueryRowsResponse;

    /**
     *
     */
    constructor(queryRowsResponse: QueryRowsResponse) {
        super();
        this.queryRowsResponse = queryRowsResponse;
    }

    private render(): preact.VNode {


        const results: any[] = this.queryRowsResponse[0];
        const startIndex: number = Number((this.queryRowsResponse[1] as any).startIndex);
        const maxResults: number = (this.queryRowsResponse[1] as any).maxResults;
        const queryResults: bigquery.IGetQueryResultsResponse = this.queryRowsResponse[2] || {};


        //array of elements to create
        const elements: preact.VNode[] = [];

        //is paging necessary?
        if (queryResults.totalRows && Number(queryResults.totalRows) != results.length) {

            elements.push(this.getPagination(results, queryResults));

            elements.push(preact.h('vscode-divider', {}, []));
        }

        //grid
        const schema: bigquery.ITableSchema | null = queryResults.schema || null;
        //error unlikely to happen, that's why is lower in the code
        if (!schema) { throw Error('Unexpected query result'); }

        elements.push(this.getGrid(schema, results, startIndex + 1));

        //bundle all under a div
        return preact.h('div', {}, elements);
    }

    private getPagination(results: any[], queryResults: bigquery.IGetQueryResultsResponse): preact.VNode {

        //array of elements to create
        const elements: preact.VNode[] = [];

        elements.push(preact.h('span',
            { 'style': 'padding:5px 10px; display:inline-flex; vertical-align:top;color:var(--button-secondary-foreground);background:var(--button-secondary-background)' },
            `1 - ${results.length} of ${queryResults.totalRows}`));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-dropdown', {}, [
            preact.h('vscode-option', {}, 'Option Label #1'),
            preact.h('vscode-option', {}, 'Option Label #2'),
            preact.h('vscode-option', {}, 'Option Label #3')
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("first_page")' }, [
            'First page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("previous_page")' }, [
            'Previous page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("next_page")' }, [
            'Next page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-right' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("last_page")' }, [
            'Last page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-right' }, [])
        ]));

        return preact.h('div', {}, elements);
    }

    private getGrid(schema: bigquery.ITableSchema, results: any[], startRowNumber: number): preact.VNode {

        const headerCellStyle = 'background-color: var(--list-hover-background);';

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

        //give the necessary with to columns that contain bigger values. max 80 (`.8 * x em` later)
        function updateCellWith(widthIndex: number, valueString: string | null) {
            if (valueString != null) {
                const currentWidth = widths[widthIndex];
                if (valueString?.length && valueString.length > currentWidth) {
                    widths[widthIndex] = Math.min(80, valueString.length);
                }
            }
        }

        //
        for (let resultIndex = 0; resultIndex < results.length; resultIndex++) {

            const cells = [];
            cells.push(preact.h('vscode-data-grid-cell', { 'style': headerCellStyle, 'grid-column': '1' }, (startRowNumber++).toString()));

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

            updateCellWith(0, (startRowNumber).toString());

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        return preact.h('vscode-data-grid', { 'generate-header': 'sticky', 'grid-template-columns': widths.map(c => `${c * .8}em`).join(' ') }, rows);
    }

    override toString(): string {
        return p.render(this.render());
    }

}
