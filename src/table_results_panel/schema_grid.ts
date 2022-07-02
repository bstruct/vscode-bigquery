import * as preact from 'preact';
import * as p from 'preact-render-to-string';

export class SchemaGrid extends Object {

    private tableMetadata: TableMetadata;

    constructor(tableMetadata: TableMetadata) {
        super();
        this.tableMetadata = tableMetadata;
    }

    private render(): preact.VNode[] {

        //array of elements to create
        const elements: preact.VNode[] = [];

        const [gridNode, _] = this.getGrid(this.tableMetadata.schema.fields);

        elements.push(gridNode);

        //bundle all under a div
        // return preact.h('div', {}, elements);
        return elements;
    }

    private getGrid(schemaFields: SchemaField[]): [preact.VNode, number] {

        const headerCellStyle = 'background-color: var(--list-hover-background);';

        //initialize rows array with the header column row already
        const rows = [];
        rows.push(preact.h('vscode-data-grid-row', { 'row-type': 'header' }, [
            preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '1' }, 'Field name'),
            preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '2' }, 'Type'),
            preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '3' }, 'Mode'),
            preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '4' }, 'Collation'),
            preact.h('vscode-data-grid-cell', { 'cell-type': 'columnheader', 'style': headerCellStyle, 'grid-column': '5' }, 'Description'),
        ]));

        //widths of the columns
        const widths: number[] = [10, 10, 10, 10, 35];

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
        for (let index = 0; index < schemaFields.length; index++) {

            
            const schemaField: SchemaField = schemaFields[index];
            
            const cells = [];
            cells.push(preact.h('vscode-data-grid-cell', { 'grid-column': '1' }, schemaField.name));
            cells.push(preact.h('vscode-data-grid-cell', { 'grid-column': '2' }, schemaField.type));
            cells.push(preact.h('vscode-data-grid-cell', { 'grid-column': '3' }, schemaField.mode));
            cells.push(preact.h('vscode-data-grid-cell', { 'grid-column': '4' }, schemaField.collation));
            cells.push(preact.h('vscode-data-grid-cell', { 'grid-column': '5' }, schemaField.description));

            updateCellWith(0, schemaField.name);

            rows.push(preact.h('vscode-data-grid-row', {}, cells));
        }

        const table = preact.h('vscode-data-grid', { 'generate-header': 'sticky', 'grid-template-columns': widths.map(c => `${Math.ceil(c * .85)}em`).join(' ') }, rows);
        let props: any = {};
        // if (innerGrid) {
        //     props.style = 'max-height: 20em;overflow-y:scroll;overflow-x:visible;';
        // }

        const wrappingDiv = preact.h('div', props, table);

        const totalWidth: number = widths.reduce((previous, current, index) => previous + current);

        return [wrappingDiv, totalWidth];
    }

    override toString(): string {
        return this.render().map(c => p.render(c)).join('');
    }

}