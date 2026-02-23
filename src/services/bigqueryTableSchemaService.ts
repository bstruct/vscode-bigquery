import { getBigQueryClient } from "../extensionCommands";
import { BqsqlDocumentItem } from "../language/bqsqlDocument";
import { Authentication } from "./authentication";
import { BigqueryTableSchema } from "./bigqueryTableSchema";
import { outputChannel } from "../logger";

export class BigqueryTableSchemaService {

    private schemas: BigqueryTableSchema[] = [];
    private defaultProjectId: string | null = null;

    private normalizeProjectId(projectId: string | null | undefined): string | null {
        const value = (projectId ?? '').trim();
        if (!value) { return null; }
        const lower = value.toLowerCase();
        if (lower === '(unset)' || lower === '(not set)' || lower === 'unset' || lower === 'null' || lower === 'undefined') {
            return null;
        }
        return value;
    }

    public async preLoadSchemaToCache(bqsql: & string, tableIdentifier: BqsqlDocumentItem): Promise<boolean> {

        if (this.defaultProjectId === null) {
            this.defaultProjectId = this.normalizeProjectId(await Authentication.getDefaultProjectId());
        }

        let table = this.resolveTableIdentifier(bqsql, tableIdentifier);
        if (table !== null) {

            let projectId = table[0];
            let datasetName = table[1];
            let tableName = table[2];

            let q = this.schemas.filter(c => c.project_id === projectId && c.dataset_name === datasetName && c.table_name === tableName);
            if (q.length === 0) {
                const bqClient = await getBigQueryClient();
                let tableSchema: BigqueryTableSchema[] = await bqClient.getTableSchema(projectId, datasetName, tableName);
                this.schemas.push(...tableSchema);

                return true;
            }
        }

        return false;
    }

    // -----------------------------------------------------------------------
    // New helpers – work directly with a full table name string
    // -----------------------------------------------------------------------

    /**
     * Resolve a full table name like 'project.dataset.table' (or 'dataset.table')
     * into its [projectId, datasetName, tableName] tuple, falling back to the
     * default project when the project part is absent.
     */
    private resolveFullName(fullName: string): [string, string, string] | null {
        const parts = fullName.replace(/`/g, '').split('.');
        let projectId: string | null = null;
        let datasetName: string | null = null;
        let tableName: string | null = null;

        if (parts.length === 3) {
            [projectId, datasetName, tableName] = parts;
        } else if (parts.length === 2) {
            projectId = this.normalizeProjectId(this.defaultProjectId);
            [datasetName, tableName] = parts;
        }

        if (projectId && datasetName && tableName) {
            return [projectId, datasetName, tableName];
        }
        return null;
    }

    /**
     * Pre-load the schema for a table identified by its full name string.
     * Safe to call multiple times – returns immediately if already cached.
     */
    public async preLoadSchemaByFullName(fullName: string): Promise<boolean> {
        if (this.defaultProjectId === null) {
            this.defaultProjectId = this.normalizeProjectId(await Authentication.getDefaultProjectId());
        }

        const resolved = this.resolveFullName(fullName);
        if (!resolved) {
            outputChannel.appendLine(`[schema] preLoad FAILED to resolve: "${fullName}"  defaultProject="${this.defaultProjectId}"`);
            return false;
        }
        const [projectId, datasetName, tableName] = resolved;

        const existing = this.schemas.filter(
            c => c.project_id === projectId && c.dataset_name === datasetName && c.table_name === tableName
        );
        if (existing.length === 0) {
            try {
                outputChannel.appendLine(`[schema] fetching ${projectId}.${datasetName}.${tableName} ...`);
                const bqClient = await getBigQueryClient();
                const tableSchema: BigqueryTableSchema[] = await bqClient.getTableSchema(projectId, datasetName, tableName);
                this.schemas.push(...tableSchema);
                outputChannel.appendLine(`[schema] loaded ${tableSchema.length} columns for ${projectId}.${datasetName}.${tableName}`);
                return true;
            } catch (ex) {
                outputChannel.appendLine(`[schema] ERROR loading ${projectId}.${datasetName}.${tableName}: ${ex}`);
            }
        } else {
            outputChannel.appendLine(`[schema] cache hit: ${existing.length} columns for ${projectId}.${datasetName}.${tableName}`);
        }
        return false;
    }

    /**
     * Return the cached schema columns for a table identified by its full name.
     * May return an empty array if the schema has not been loaded yet.
     */
    public getSchemaByFullName(fullName: string): BigqueryTableSchema[] {
        const resolved = this.resolveFullName(fullName);
        if (!resolved) {
            outputChannel.appendLine(`[schema] getSchema FAILED to resolve: "${fullName}"`);
            return [];
        }
        const [projectId, datasetName, tableName] = resolved;
        const result = this.schemas.filter(
            c => c.project_id === projectId && c.dataset_name === datasetName && c.table_name === tableName
        );
        outputChannel.appendLine(`[schema] getSchema "${fullName}" → ${result.length} columns (resolved: ${projectId}.${datasetName}.${tableName})`);
        return result;
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
