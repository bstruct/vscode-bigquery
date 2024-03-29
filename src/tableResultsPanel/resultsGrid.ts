import bigquery from '@google-cloud/bigquery/build/src/types';
import * as preact from 'preact';
import * as p from 'preact-render-to-string';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';

//---------------- @vscode/webview-ui-toolkit ----------------
//https://github.com/microsoft/vscode-webview-ui-toolkit
//https://microsoft.github.io/vscode-webview-ui-toolkit/?path=/story/library-badge--default


//---------------- @vscode/codicons ----------------
//https://microsoft.github.io/vscode-codicons/dist/codicon.html

export class ResultsGrid extends Object {

    // private queryRowsResponse: QueryRowsResponse;
    private request: ResultsGridRenderRequest;
    private schema: bigquery.ITableSchema;
    private rows: any[];
    private totalRows: number;
    // private jobReferences: JobReference[] | undefined;
    // private startIndex: number;
    // private maxResults: number;
    // private queryCount: number;
    // private queryIndex: number;
    // private openInTabVisible: boolean;

    /**
     *
     */
    constructor(
        request: ResultsGridRenderRequest,
        schema: bigquery.ITableSchema,
        rows: any[],
        totalRows: number) {

        super();
        // this.queryRowsResponse = queryRowsResponse;
        this.request = request;
        this.schema = schema;
        this.rows = rows;
        this.totalRows = totalRows;
        // this.jobReferences = jobReferences;
        // this.startIndex = startIndex;
        // this.maxResults = maxResults;
        // this.queryCount = queryCount;
        // this.queryIndex = queryIndex;
        // this.openInTabVisible = openInTabVisible;
    }

    private render(): preact.VNode[] {

        const resultsSize: number = this.rows.length;

        //array of elements to create
        const elements: preact.VNode[] = [];

        elements.push(this.getControls(this.request.startIndex, this.request.maxResults, this.totalRows, resultsSize));

        elements.push(preact.h('vscode-divider', {}, []));

        const [gridNode, _] = this.getGrid(this.schema, this.rows, this.request.startIndex + 1, false);

        elements.push(gridNode);

        //bundle all under a div
        // return preact.h('div', {}, elements);
        return elements;
    }

    private getControls(startIndex: number, maxResults: number, totalRows: number, resultsSize: number): preact.VNode {

        //array of elements to create
        const elements: preact.VNode[] = [];

        if (this.request && this.request.jobReferences && this.request.jobReferences.length > 1) {

            const options = [];
            for (let index = 0; index < this.request.jobReferences.length; index++) {
                options.push(preact.h('vscode-option', { selected: this.request.jobIndex === index, value: index }, `Result query ${index + 1}`));
            }

            const parametersSerialized = JSON.stringify([this.request.jobReferences, this.request.tableReference, this.request.startIndex, this.request.maxResults, totalRows, "$$$", this.request.openInTabVisible]).replace('"$$$"', 'this.value');

            elements.push(preact.h('vscode-dropdown', { 'onchange': `vscode.postMessage({"command":"query_index_change", parameters: ${parametersSerialized}})` }, options));

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

        const parameters = [this.request.jobReferences, this.request.tableReference, this.request.startIndex, this.request.maxResults, totalRows, this.request.jobIndex, this.request.openInTabVisible];

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "first_page", parameters: ${JSON.stringify(parameters)}})`, disabled: !firstAndPreviousPageEnabled }, [
            'First page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "previous_page", parameters: ${JSON.stringify(parameters)}})`, disabled: !firstAndPreviousPageEnabled }, [
            'Previous page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-left' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        const nextAndLastPageEnabled = startIndex + resultsSize < totalRows;

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "next_page", parameters: ${JSON.stringify(parameters)}})`, disabled: !nextAndLastPageEnabled }, [
            'Next page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-small-right' }, [])
        ]));

        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "last_page", parameters: ${JSON.stringify(parameters)}})`, disabled: !nextAndLastPageEnabled }, [
            'Last page',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-arrow-circle-right' }, [])
        ]));

        if (this.request.openInTabVisible) {

            elements.push(preact.h('span', {}, ' '));

            elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "open_in_tab", parameters: ${JSON.stringify(parameters)}})` }, [
                'Open in tab',
                preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-open-preview' }, [])
            ]));

        }

        //download csv
        elements.push(preact.h('span', {}, ' '));

        elements.push(preact.h('vscode-button', { 'appearance': 'secondary', 'onclick': `vscode.postMessage({"command": "download_csv", parameters: ${JSON.stringify(parameters)}})` }, [
            'Download CSV',
            preact.h('span', { 'slot': 'start', 'class': 'codicon codicon-cloud-download' }, [])
        ]));

        return preact.h('div', {}, elements);
    }

    /**
     * 
     * @param schema 
     * @param results 
     * @param startRowNumber 
     * @param innerGrid
     * If this grid to be generated is meant to be inside another grid. Basically, at the moment, to put a max height on the wrapping div 
     * @returns 
     */
    private getGrid(schema: bigquery.ITableSchema, results: any[], startRowNumber: number, innerGrid: boolean): [preact.VNode, number] {

        const headerCellStyle = 'background-color: var(--list-hover-background);';

        const fields: bigquery.ITableFieldSchema[] = schema.fields || [];
        const fieldNames: string[] = fields?.map(c => c.name || '') || [];

        //cells that contain the top level schema column names
        function getHeaderCells() {
            // eslint-disable-next-line @typescript-eslint/naming-convention
            const cells: preact.VNode[] = [preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '1' }, 'Row')];
            for (let fieldIndex = 0; fieldIndex < fieldNames.length; fieldIndex++) {
                const fieldName = fieldNames[fieldIndex];
                // eslint-disable-next-line @typescript-eslint/naming-convention
                const cell = preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': (fieldIndex + 2).toString() }, fieldName);
                cells.push(cell);
            }
            return cells;
        }

        //initialize rows array with the header column row already
        const rows = [];
        // eslint-disable-next-line @typescript-eslint/naming-convention
        rows.push(preact.h('vscode-data-grid-row', { 'row-type': 'header', 'style': 'position:sticky;top:0' }, getHeaderCells()));

        //widths of the columns
        const widths: number[] = [5];
        widths.push(...fieldNames.map(c => Math.max(c.length, 6)));//min with of 6

        //give the necessary with to columns that contain bigger values. max 80 (`.8 * x em` later)
        function updateCellWith(widthIndex: number, valueString: string | null) {
            if (valueString !== null) {
                const currentWidth = widths[widthIndex];
                if (valueString?.length && valueString.length > currentWidth) {
                    widths[widthIndex] = Math.min(80, valueString.length);
                }
            }
        }

        //
        for (let resultIndex = 0; resultIndex < results.length; resultIndex++) {

            const cells = [];
            // eslint-disable-next-line @typescript-eslint/naming-convention
            cells.push(preact.h('vscode-data-grid-cell', { 'style': headerCellStyle, 'cell-type': 'columnheader', 'grid-column': '1' }, (startRowNumber++).toString()));

            const result: any = results[resultIndex];
            for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex++) {
                const field = fields[fieldIndex];

                let value: any = null;

                // eslint-disable-next-line @typescript-eslint/naming-convention
                let cellProperties: any = { 'grid-column': (fieldIndex + 2).toString() };

                if (field.mode === 'REPEATED' || field.type === 'RECORD') {

                    let innerSchema: bigquery.ITableSchema = {};
                    let innerResults: any[] = [];

                    if (field.type === 'RECORD') {

                        innerSchema = { fields: field.fields || [] };
                        if (field.mode === 'REPEATED') {
                            innerResults = result ? result[field.name || ''] : '';
                        } else {
                            innerResults = [result ? result[field.name || ''] : ''];
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
                    [value, totalWidth] = this.getGrid(innerSchema, innerResults || [], 1, true);

                    cellProperties.class = 'disableFocus';

                    widths[fieldIndex + 1] = Math.max(widths[fieldIndex + 1], totalWidth + 4);

                } else {

                    value = result ? result[field.name || ''] : '';

                    switch (field.type || 'STRING') {
                        case 'DATETIME':
                        case 'DATE':
                        case 'TIME':
                        case 'TIMESTAMP':
                        case 'GEOGRAPHY':
                            if (value !== null && value !== undefined) {
                                value = value.value;
                            }
                            break;
                        case 'NUMERIC':
                        case 'FLOAT':
                        case 'INTEGER':
                            if (value !== null && value !== undefined) {
                                value = value.toString();
                            }
                            break;
                        case 'BYTES':
                            if (value !== null && value !== undefined) {
                                value = value.toString('base64');
                            }
                            break;
                        case 'BOOLEAN':
                            if (value !== null && value !== undefined) {
                                value = value ? 'true' : 'false';
                            }
                            break;
                        case 'STRING':
                        case 'INTERVAL':
                        case 'JSON':
                            break;
                        default:
                            console.info(`field ${field.name} has type ${field.type}`);
                            break;
                    }

                    updateCellWith(fieldIndex + 1, value);
                }

                const valueDisplay = value === '' ? '' : value || preact.h('span', { class: 'nullValue' }, 'null');

                const cell = preact.h('vscode-data-grid-cell', cellProperties, valueDisplay);
                cells.push(cell);
            }

            updateCellWith(0, (startRowNumber).toString());

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        // eslint-disable-next-line @typescript-eslint/naming-convention
        const table = preact.h('vscode-data-grid', { 'sgenerate-header': 'sticky', 'grid-template-columns': widths.map(c => `${Math.ceil(c * .85)}em`).join(' ') }, rows);
        let props: any = {};
        if (innerGrid) {
            props.style = 'max-height: 20em;overflow-y:scroll;overflow-x:visible;';
        }

        const wrappingDiv = preact.h('div', props, table);

        const totalWidth: number = widths.reduce((previous, current, index) => previous + current);

        return [wrappingDiv, totalWidth + 2];
    }

    override toString(): string {
        return this.render().map(c => p.render(c)).join('');
    }

}