import { getBigQueryClient } from "../extensionCommands";
import { BqsqlDocumentItem } from "../language/bqsqlDocument";
import { Authentication } from "./authentication";
import { BigqueryTableSchema } from "./bigqueryTableSchema";

export class BigqueryTableSchemaService {

    private schemas: BigqueryTableSchema[] = [];
    private defaultProjectId: string | null = null;

    public async preLoadSchemaToCache(bqsql: & string, tableIdentifier: BqsqlDocumentItem): Promise<boolean> {

        if (this.defaultProjectId === null) {
            this.defaultProjectId = await Authentication.getDefaultProjectId();
        }

        let table = this.resolveTableIdentifier(bqsql, tableIdentifier);
        if (table !== null) {

            let projectId = table[0];
            let datasetName = table[1];
            let tableName = table[2];

            let q = this.schemas.filter(c => c.project_id === projectId && c.dataset_name === datasetName && c.table_name === tableName);
            if (q.length === 0) {
                let tableSchema: BigqueryTableSchema[] = await getBigQueryClient().getTableSchema(projectId, datasetName, tableName);
                this.schemas.push(...tableSchema);

                return true;
            }
        }

        return false;
    }

    public getSchemaFromCache(bqsql: & string, tableIdentifier: BqsqlDocumentItem): BigqueryTableSchema[] {

        let table = this.resolveTableIdentifier(bqsql, tableIdentifier);
        if (table !== null) {

            let projectId = table[0];
            let datasetName = table[1];
            let tableName = table[2];

            return this.schemas.filter(c => c.project_id === projectId && c.dataset_name === datasetName && c.table_name === tableName);
        }

        return [];
    }

    private resolveTableIdentifier(bqsql: & string, tableIdentifier: BqsqlDocumentItem): [project_id: string, dataset_name: string, table_name: string] | null {

        if (tableIdentifier.item_type === "TableIdentifier" && tableIdentifier.items.length > 0) {

            let projectId: string | null = null;
            let datasetName: string | null = null;
            let tableName: string | null = null;

            for (let index = 0; index < tableIdentifier.items.length; index++) {
                const element = tableIdentifier.items[index];

                let text = this.getText(bqsql, element.range);

                if (text !== null) {
                    if (text.startsWith('`')) { text = text.substring(1); }
                    if (text.endsWith('`')) { text = text.substring(0, text.length - 1); }

                    if (element.item_type === "TableIdentifierProjectId") {
                        projectId = text;
                    }

                    if (element.item_type === "TableIdentifierDatasetIdTableId") {
                        const split = text.split('.');
                        datasetName = split[0];
                        tableName = split[1];
                        break;
                    }

                    if (element.item_type === "TableIdentifierProjectIdDatasetId") {
                        const split = text.split('.');
                        projectId = split[0];
                        datasetName = split[1];
                    }

                    if (element.item_type === "TableIdentifierDatasetId") {
                        datasetName = text;
                    }
                    if (element.item_type === "TableIdentifierTableId") {
                        tableName = text;
                    }
                    if (element.item_type === "TableIdentifierProjectIdDatasetIdTableId") {
                        const split = text.split('.');
                        projectId = split[0];
                        datasetName = split[1];
                        tableName = split[2];
                        break;
                    }
                }
            }

            if (projectId === null && datasetName !== null && tableName !== null) {
                projectId = this.defaultProjectId;
            }

            if (projectId !== null && datasetName !== null && tableName !== null) {
                return [projectId, datasetName, tableName];
            }

        }

        return null;
    }

    private getText(bqsql: string, range: number[]): string | null {

        try {
            const lines = bqsql.split('\n');
            return lines[range[0]].substring(range[1], range[2]);
        } catch (ex) { }

        return null;
    }

}
