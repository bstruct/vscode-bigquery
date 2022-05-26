import { SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import bigquery from '@google-cloud/bigquery/build/src/types';
import * as preact from 'preact';
import * as p from 'preact-render-to-string';

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

        //remove type
        const response: any = this.simpleQueryRowsResponse;

        const queryResults: bigquery.IGetQueryResultsResponse = response[2];

        const schema: bigquery.ITableSchema | null = queryResults.schema || null;
        if (!schema) { throw Error('Unexpected query result'); }

        const fields: bigquery.ITableFieldSchema[] = schema.fields || [];


        const headerCells = [];
        for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex++) {
            const field: bigquery.ITableFieldSchema = fields[fieldIndex];
            const fieldName = field.name || '';
            const cell = preact.h('vscode-data-grid-cell', { "cell-type": "columnheader", "grid-column": (fieldIndex + 1).toString() }, [fieldName]);
            headerCells.push(cell);
        }
        const headerRow = preact.h('vscode-data-grid-row', { "row-type": "header" }, headerCells);;

        return preact.h('vscode-data-grid', {}, headerRow);

    }

    override toString(): string {
        return p.render(this.render());
    }

}