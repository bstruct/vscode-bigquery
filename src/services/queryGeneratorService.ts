import { TableMetadata } from "@google-cloud/bigquery";
import { BigqueryTreeItem } from "../activitybar/treeItem";


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

    public static generateDdlQuery(item: BigqueryTreeItem): string {
        
        return `SELECT ddl
FROM \`${item.projectId}\`.${item.datasetId}.INFORMATION_SCHEMA.ROUTINES
WHERE routine_name = '${item.tableId}';`;

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