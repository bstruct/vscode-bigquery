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
    
    type: string;

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
    collation: string;
    description: string;
    fields: SchemaField[];
}