import { QueryRowsResponse } from '@google-cloud/bigquery';
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
    private startIndex: number;
    private maxResults: number;
    private queryCount: number;
    private queryIndex: number

    /**
     *
     */
    constructor(
        queryRowsResponse: QueryRowsResponse,
        startIndex: number,
        maxResults: number,
        queryCount: number,
        queryIndex: number) {

        super();
        this.queryRowsResponse = queryRowsResponse;
        this.startIndex = startIndex;
        this.maxResults = maxResults;
        this.queryCount = queryCount;
        this.queryIndex = queryIndex;
    }

    private render(): preact.VNode {

        const results: any[] = this.queryRowsResponse[0];
        // const paging: BigqueryQueryPaging | null = (this.queryRowsResponse[1] as any || null);
        const queryResults: bigquery.IGetQueryResultsResponse = this.queryRowsResponse[2] || {};

        //array of elements to create
        const elements: preact.VNode[] = [];

        //is paging necessary?
        // if (queryResults.totalRows && Number(queryResults.totalRows) != results.length) 
        {

            const totalRows: number = Number(queryResults.totalRows || 0);

            const resultsSize: number = results.length;

            elements.push(this.getPagination(this.startIndex, this.maxResults, totalRows, resultsSize));

            elements.push(preact.h('vscode-divider', {}, []));

        }

        //grid
        const schema: bigquery.ITableSchema | null = queryResults.schema || null;
        //error unlikely to happen, that's why is lower in the code
        if (!schema) { throw Error('Unexpected query result'); }

        const [gridNode, _] = this.getGrid(schema, results, this.startIndex + 1);

        elements.push(gridNode);

        //bundle all under a div
        return preact.h('div', {}, elements);
    }

    private getPagination(startIndex: number, maxResults: number, totalRows: number, resultsSize: number): preact.VNode {

        //array of elements to create
        const elements: preact.VNode[] = [];

        if (this.queryCount > 1) {

            const options = [];
            for (let index = 0; index < this.queryCount; index++) {
                options.push(preact.h('vscode-option', { selected: this.queryIndex === index, value: index }, `Result query ${index + 1}`));
            }

            elements.push(preact.h('vscode-dropdown', { 'onchange': 'vscode.postMessage({"command":"query_index_change", "value": this.value})' }, options));

            elements.push(preact.h('span', {}, ' '));
        }

        elements.push(preact.h('span',
            { 'style': 'padding:5px 10px; display:inline-flex; vertical-align:top;color:var(--button-secondary-foreground);background:var(--button-secondary-background)' },
            `${Math.min(startIndex + 1, totalRows)} - ${startIndex + resultsSize} of ${totalRows}`));

        elements.push(preact.h('span', {}, ' '));

        //TODO: implement possibility to change page size
        // elements.push(preact.h('vscode-dropdown', {}, [
        //     preact.h('vscode-option', {}, 'Option Label #1'),
        //     preact.h('vscode-option', {}, 'Option Label #2'),
        //     preact.h('vscode-option', {}, 'Option Label #3')
        // ]));

        // elements.push(preact.h('span', {}, ' '));

        const firstAndPreviousPageEnabled = startIndex >= maxResults;

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("first_page")', disabled: !firstAndPreviousPageEnabled }, [
            'First page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("previous_page")', disabled: !firstAndPreviousPageEnabled }, [
            'Previous page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        const nextAndLastPageEnabled = startIndex + resultsSize < totalRows;

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("next_page")', disabled: !nextAndLastPageEnabled }, [
            'Next page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-right' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': 'vscode.postMessage("last_page")', disabled: !nextAndLastPageEnabled }, [
            'Last page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-right' }, [])
        ]));

        return preact.h('div', {}, elements);
    }

    private getGrid(schema: bigquery.ITableSchema, results: any[], startRowNumber: number): [preact.VNode, number] {

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
        widths.push(...fieldNames.map(c => Math.max(c.length, 6)));//min with of 5

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

                let value: any = null;

                if (field.mode == 'REPEATED' || field.type == 'RECORD') {

                    let innerSchema: bigquery.ITableSchema = {};
                    let innerResults: any[] = [];

                    if (field.type == 'RECORD') {

                        innerSchema = { fields: field.fields || [] };
                        if (field.mode == 'REPEATED') {
                            innerResults = result[field.name || ''];
                        } else {
                            innerResults = [result[field.name || '']];
                        }

                    } else {

                        const fieldName: string = field.name || '';

                        const field1: bigquery.ITableFieldSchema = { name: field.name, type: field.type, mode: 'NULLABLE' };
                        innerSchema = { fields: [field1] };
                        innerResults = (result[field.name || ''] as any[]).map(c => {
                            const item: any = {};
                            item[fieldName] = c;
                            return item;
                        });

                    }

                    let totalWidth: number = 0;
                    [value, totalWidth] = this.getGrid(innerSchema, innerResults, 1);

                    widths[fieldIndex + 1] = Math.max(widths[fieldIndex + 1], totalWidth);

                } else {

                    value = result[field.name || ''];

                    switch (field.type || 'STRING') {
                        case 'DATETIME':
                        case 'DATE':
                        case 'TIME':
                        case 'TIMESTAMP':
                        case 'GEOGRAPHY':
                            if (value != null) {
                                value = value.value;
                            }
                            break;
                        case 'NUMERIC':
                        case 'FLOAT':
                        case 'INTEGER':
                            if (value != null) {
                                value = value.toString();
                            }
                            break;
                        case 'BYTES':
                            if (value != null) {
                                value = value.toString('base64');
                            }
                            break;
                        case 'STRING':
                        case 'INTERVAL':
                            break;
                        default:
                            console.info(`field ${field.name} has type ${field.type}`);
                            break;
                    }

                    updateCellWith(fieldIndex + 1, value);
                }

                const cell = preact.h('vscode-data-grid-cell', { 'grid-column': (fieldIndex + 2).toString() }, value || '');
                cells.push(cell);
            }

            updateCellWith(0, (startRowNumber).toString());

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        const table = preact.h('vscode-data-grid', { 'generate-header': 'sticky', 'grid-template-columns': widths.map(c => `${c * .8}em`).join(' ') }, rows);
        const totalWidth: number = widths.reduce((previous, current, index) => previous + current);

        return [table, totalWidth + 2];
    }

    override toString(): string {
        return p.render(this.render());
    }

}