interface TableMetadata {

    creationTime: string;
    // '1642435677331'
    etag: string;
    // '1GXriUdGTuQwsMyvm81arw=='
    id: string;
    // 'damiao-project-1:PvhTest.MetapackProofOfDeliveryWithICSInformation'
    kind: string;
    // 'bigquery#table'
    lastModifiedTime: string;
    // '1642435677331'
    location: string;
    // 'EU'
    numActiveLogicalBytes: string;
    // '0'
    numBytes: string;
    // '33190086'
    numLongTermBytes: string;
    // '33190086'
    numLongTermLogicalBytes: string;
    // '33190086'
    numRows: string;
    // '115621'
    numTotalLogicalBytes: string;
    // '33190086'
    schema: { fields: SchemaField[] };
    // {fields: Array(19)}
    selfLink: string;
    // 'https://bigquery.googleapis.com/bigquery/v2/projects/damiao-project-1/datasets/PvhTest/tables/MetapackProofOfDeliveryWithICSInformation'
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
    collationName: string;
    description: string;
    fields: SchemaField[];
}