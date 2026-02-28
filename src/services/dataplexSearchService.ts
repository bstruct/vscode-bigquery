/**
 * DataplexSearchService
 *
 * Uses the Dataplex Universal Catalog REST API (dataplex.googleapis.com)
 * `projects.locations.searchEntries` endpoint, which powers the Dataplex
 * Universal Catalog portal.  Results are served from a pre-built full-text
 * index — no BigQuery jobs are needed.
 *
 * Reference:
 *   POST https://dataplex.googleapis.com/v1/{name=projects/{p}/locations/{l}}:searchEntries
 *   https://cloud.google.com/dataplex/docs/reference/rest/v1/projects.locations/searchEntries
 */

import * as https from 'https';

// ── Public types ─────────────────────────────────────────────────────────────

export interface CatalogEntry {
    kind: 'dataset' | 'table' | 'view' | 'model' | 'other';
    projectId: string;
    datasetId: string;
    tableId?: string;
    displayName: string;
    description?: string;
    /** ISO-8601 timestamp from the catalog index */
    modifyTime?: string;
    fullyQualifiedName?: string;
}

export interface CatalogPage {
    results: CatalogEntry[];
    /** Pass back to fetch the next page; `undefined` means no more pages. */
    nextPageToken?: string;
}

// ── Service ──────────────────────────────────────────────────────────────────

export class DataplexSearchService {

    private static readonly HOSTNAME = 'dataplex.googleapis.com';

    /**
     * Search the Dataplex Universal Catalog.
     *
     * @param query      Free-text search term (same syntax as the GCP portal)
     * @param projectId  Project used as the API path context (`name` param);
     *                   results scope defaults to the project's organisation
     * @param token      OAuth2 Bearer token (from BigQueryClient.getToken())
     * @param pageToken  Token returned by a previous call; omit for first page
     * @param pageSize   Number of results per page (default: 10)
     * @param signal     Cancellation handle
     */
    public static async search(
        query: string,
        projectId: string,
        token: string,
        pageToken?: string,
        pageSize = 10,
        signal?: { cancelled: boolean },
    ): Promise<CatalogPage> {

        if (signal?.cancelled) { return { results: [] }; }

        const params = new URLSearchParams({
            query: `system=BIGQUERY ${query}`,
            pageSize: String(pageSize),
            orderBy: 'relevance',
        });
        if (pageToken) { params.set('pageToken', pageToken); }

        const path = `/v1/projects/${encodeURIComponent(projectId)}/locations/global:searchEntries?${params.toString()}`;

        const raw = await DataplexSearchService.request(path, token);

        if (signal?.cancelled) { return { results: [] }; }

        return DataplexSearchService.parse(raw);
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /** POST to the Dataplex HTTPS endpoint with an empty body and query params in the path. */
    private static request(path: string, bearerToken: string): Promise<string> {
        return new Promise((resolve, reject) => {
            const options: https.RequestOptions = {
                hostname: DataplexSearchService.HOSTNAME,
                path,
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${bearerToken}`,
                    'Content-Length': 0,
                },
            };

            const req = https.request(options, res => {
                let data = '';
                res.on('data', (chunk: string) => { data += chunk; });
                res.on('end', () => {
                    if (res.statusCode && res.statusCode >= 400) {
                        reject(new Error(
                            `Dataplex API returned HTTP ${res.statusCode}: ${data}`
                        ));
                    } else {
                        resolve(data);
                    }
                });
            });

            req.on('error', reject);
            req.end();
        });
    }

    /** Parse the raw API response JSON into our CatalogPage type. */
    private static parse(raw: string): CatalogPage {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const json: any = JSON.parse(raw);

        const results: CatalogEntry[] = (json.results ?? []).map(
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            (r: any) => DataplexSearchService.parseEntry(r)
        );

        return {
            results,
            nextPageToken: json.nextPageToken ?? undefined,
        };
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    private static parseEntry(r: any): CatalogEntry {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const entry: any = r.dataplexEntry ?? {};

        // fullyQualifiedName: "bigquery:project.dataset" or "bigquery:project.dataset.table"
        const fqn: string = entry.fullyQualifiedName ?? '';
        const bqPart = fqn.startsWith('bigquery:') ? fqn.slice('bigquery:'.length) : '';
        const parts = bqPart.split('.');
        const projectId = parts[0] ?? '';
        const datasetId = parts[1] ?? '';
        const tableId   = parts[2] || undefined;

        // entryType is a resource name ending in e.g. "bigquery-table", "bigquery-view", etc.
        const entryType: string = (entry.entryType ?? '').toLowerCase();

        let kind: CatalogEntry['kind'] = 'other';
        if      (entryType.includes('view'))    { kind = 'view'; }
        else if (entryType.includes('table'))   { kind = 'table'; }
        else if (entryType.includes('dataset')) { kind = 'dataset'; }
        else if (entryType.includes('model'))   { kind = 'model'; }

        return {
            kind,
            projectId,
            datasetId,
            tableId,
            displayName:        entry.displayName ?? tableId ?? datasetId ?? '',
            description:        entry.description || undefined,
            modifyTime:         entry.updateTime  || undefined,
            fullyQualifiedName: fqn               || undefined,
        };
    }
}
