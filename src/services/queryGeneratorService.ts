import { TableMetadata } from "@google-cloud/bigquery";


export class QueryGeneratorService {

    static generateSelectQuerySimple(projectId: string, datasetId: string, tableId: string): string {
        return `SELECT
\t
FROM \`${projectId}.${datasetId}.${tableId}\`
LIMIT 10;`;
    }


    public static generateSelectQuery(metadata: TableMetadata): string {

        return `SELECT
\t
FROM \`${metadata.tableReference?.projectId}.${metadata.tableReference?.datasetId}.${metadata.tableReference?.tableId}\` ${QueryGeneratorService.generateTimepartitionClause(metadata)}
LIMIT 10;`;

    }

    public static generateModelQuery(projectId: string, datasetId: string, modelId: string): string {
        const fqn = `\`${projectId}.${datasetId}.${modelId}\``;
        return (
            `-- ML.PREDICT — classification, regression, recommendation, clustering, etc.\n` +
            `SELECT *\n` +
            `FROM ML.PREDICT(\n` +
            `\tMODEL ${fqn},\n` +
            `\t(\n` +
            `\t\tSELECT *\n` +
            `\t\tFROM \`project.dataset.input_table\`\n` +
            `\t\tLIMIT 100\n` +
            `\t)\n` +
            `);\n\n` +
            `-- ML.FORECAST — ARIMA_PLUS / time-series models\n` +
            `-- SELECT *\n` +
            `-- FROM ML.FORECAST(\n` +
            `-- \tMODEL ${fqn},\n` +
            `-- \tSTRUCT(5 AS horizon, 0.8 AS confidence_level)\n` +
            `-- );`
        );
    }

    private static generateTimepartitionClause(metadata: TableMetadata): string {

        if (metadata.timePartitioning) {
            if (metadata.timePartitioning.type === 'DAY') {
                return `\nWHERE DATE(${metadata.timePartitioning.field}) = CURRENT_DATE()`;
            }
        }

        return '';
    }

}