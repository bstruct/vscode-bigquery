export interface TableMetadata {

    // '1642435677331'
    creationTime: string;
    // '1GXriUdGTuQwsMyvm81arw=='
    etag: string;
    // 'damiao-project-1:PvhTest.MetapackProofOfDeliveryWithICSInformation'
    id: string;
    // 'bigquery#table'
    kind: string;
    // '1642435677331'
    lastModifiedTime: string;
    // 'EU'
    location: string;
    // '0'
    numActiveLogicalBytes: string;
    // '33190086'
    numBytes: string;
    // '33190086'
    numLongTermBytes: string;
    // '33190086'
    numLongTermLogicalBytes: string;
    // '115621'
    numRows: string;
    // '33190086'
    numTotalLogicalBytes: string;
    // {fields: Array(19)}
    schema: { fields: SchemaField[] };
    // 'https://bigquery.googleapis.com/bigquery/v2/projects/damiao-project-1/datasets/PvhTest/tables/MetapackProofOfDeliveryWithICSInformation'
    selfLink: string;

    tableReference: TableReference;

    // 'TABLE' | 'VIEW' | 'MATERIALIZED_VIEW' | 'EXTERNAL'
    type: string;

    description?: string;

    /** Present for VIEW tables (type === 'VIEW'). */
    view?: {
        query?: string;
        useLegacySql?: boolean;
    };

    /** Present for MATERIALIZED_VIEW tables. */
    materializedView?: {
        query?: string;
        enableRefresh?: boolean;
        refreshIntervalMs?: string;
    };

    /** Column-based time partitioning (DAY, HOUR, MONTH, YEAR). */
    timePartitioning?: {
        type?: string;
        field?: string;
        expirationMs?: string;
    };

    /** Integer-range partitioning. */
    rangePartitioning?: {
        field?: string;
        range?: {
            start?: string;
            end?: string;
            interval?: string;
        };
    };

    /** Clustering specification. */
    clustering?: {
        fields?: string[];
    };

}

interface TableReference {

    projectId: string;

    datasetId: string;

    tableId: string;

}

interface SchemaField {
    name: string;
    mode: string;
    type: string;
    /** Populated by our REST-API path (bigquery.googleapis.com tables.get). */
    collationSpec?: string;
    /** Legacy alias kept for backward compat; populated by the enrichment helper. */
    collation: string;
    description: string;
    fields: SchemaField[];
}