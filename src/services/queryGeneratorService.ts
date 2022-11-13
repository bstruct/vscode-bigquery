import { TableMetadata } from "@google-cloud/bigquery";


export class QueryGeneratorService {


    public static generateSelectQuery(metadata: TableMetadata): string {

        return `SELECT
\t
FROM \`${metadata.tableReference?.projectId}.${metadata.tableReference?.datasetId}.${metadata.tableReference?.tableId}\` ${QueryGeneratorService.generateTimepartitionClause(metadata)}
LIMIT 10;`;

    }

    private static generateTimepartitionClause(metadata: TableMetadata): string {

        if(metadata.timePartitioning){
            if(metadata.timePartitioning.type === 'DAY'){
                return `\nWHERE DATE(${metadata.timePartitioning.field}) = CURRENT_DATE()`;
            }
        }

        return '';
    }

}