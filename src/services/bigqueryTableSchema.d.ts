export interface BigqueryTableSchema {
    project_id: string,
    dataset_name: string,
    table_name: string,
    column_name: string,
    ordinal_position: string,
    data_type: string,
    is_partitioning_column: string,
    description: string
}