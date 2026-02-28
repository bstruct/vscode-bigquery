import { BigQuery, Dataset, Table } from '@google-cloud/bigquery';

export interface SearchResultItem {
    kind: 'dataset' | 'table' | 'view' | 'model';
    projectId: string;
    datasetId: string;
    tableId?: string;
    description?: string;
    /** Matching column names/descriptions, only populated for column-level hits */
    matchingColumns?: ColumnHit[];
}

export interface ColumnHit {
    fieldPath: string;
    description?: string;
}

export class BigQuerySearchService {

    /**
     * Search datasets, tables, views, models and column names across all projects
     * known to the current user via the BigQuery REST API (no INFORMATION_SCHEMA
     * queries — no billable jobs created).
     *
     * Strategy:
     *   1. datasets.list  → filter dataset id / description client-side
     *   2. tables.list    → filter table id / description client-side
     *   3. tables.get     → for each matching table, pull full schema to search column names/descriptions
     *
     * All three calls use the @google-cloud/bigquery client which routes through
     * bigquery.googleapis.com (BigQuery API v2). No Data Catalog API needed.
     */
    public static async search(
        projectIds: string[],
        term: string,
        signal?: { cancelled: boolean }
    ): Promise<SearchResultItem[]> {

        if (!term || term.trim().length < 1) {
            return [];
        }

        const lower = term.trim().toLowerCase();
        const results: SearchResultItem[] = [];

        for (const projectId of projectIds) {
            if (signal?.cancelled) { break; }

            const bq = new BigQuery({ projectId });

            let datasets: Dataset[] = [];
            try {
                const [ds] = await bq.getDatasets({ all: true });
                datasets = ds.filter(d => d.id && !d.id.startsWith('_'));
            } catch (err) {
                // API disabled or permission denied — skip this project
                console.warn(`[search] datasets.list failed for ${projectId}:`, (err as any)?.message);
                continue;
            }

            for (const dataset of datasets) {
                if (signal?.cancelled) { break; }

                const datasetId = dataset.id ?? '';

                // Fetch dataset metadata for description
                let datasetDescription = '';
                try {
                    const [meta] = await dataset.getMetadata();
                    datasetDescription = meta?.description ?? '';
                } catch (_) { /* no description access — ok */ }

                // Dataset name or description matches
                if (
                    datasetId.toLowerCase().includes(lower) ||
                    datasetDescription.toLowerCase().includes(lower)
                ) {
                    results.push({
                        kind: 'dataset',
                        projectId,
                        datasetId,
                        description: datasetDescription || undefined,
                    });
                }

                // Enumerate tables in this dataset
                let tables: Table[] = [];
                try {
                    const [tbls] = await dataset.getTables();
                    tables = tbls.filter(t => t.id && !t.id.startsWith('_'));
                } catch (err) {
                    console.warn(`[search] tables.list failed for ${projectId}.${datasetId}:`, (err as any)?.message);
                    continue;
                }

                for (const table of tables) {
                    if (signal?.cancelled) { break; }

                    const tableId = table.id ?? '';
                    const tableType = table.metadata?.type ?? 'TABLE';

                    // Quick check on table ID before making an extra API call
                    const tableIdMatches = tableId.toLowerCase().includes(lower);

                    // Get full metadata (includes description + schema with field descriptions)
                    let tableDescription = '';
                    let matchingColumns: ColumnHit[] = [];

                    try {
                        const [meta] = await table.getMetadata();
                        tableDescription = meta?.description ?? '';

                        // Walk schema fields for column name / description matches
                        if (meta?.schema?.fields) {
                            matchingColumns = BigQuerySearchService.searchFields(
                                meta.schema.fields,
                                null,
                                lower
                            );
                        }
                    } catch (_) { /* no column info — still report the table hit */ }

                    const tableDescMatches = tableDescription.toLowerCase().includes(lower);
                    const columnMatches = matchingColumns.length > 0;

                    if (tableIdMatches || tableDescMatches || columnMatches) {
                        let kind: SearchResultItem['kind'] = 'table';
                        if (tableType === 'VIEW') { kind = 'view'; }
                        else if (tableType === 'MODEL') { kind = 'model'; }

                        results.push({
                            kind,
                            projectId,
                            datasetId,
                            tableId,
                            description: tableDescription || undefined,
                            matchingColumns: columnMatches ? matchingColumns : undefined,
                        });
                    }
                }
            }
        }

        return results;
    }

    private static searchFields(
        fields: any[],
        prefix: string | null,
        lower: string
    ): ColumnHit[] {
        const hits: ColumnHit[] = [];

        for (const field of fields) {
            const fieldPath = prefix ? `${prefix}.${field.name}` : field.name;
            const nameMatch = fieldPath.toLowerCase().includes(lower);
            const descMatch = (field.description ?? '').toLowerCase().includes(lower);

            if (nameMatch || descMatch) {
                hits.push({
                    fieldPath,
                    description: field.description || undefined,
                });
            }

            // Recurse into RECORD/STRUCT fields
            if (field.fields && field.fields.length > 0) {
                hits.push(...BigQuerySearchService.searchFields(field.fields, fieldPath, lower));
            }
        }

        return hits;
    }
}
