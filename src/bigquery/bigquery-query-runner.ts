import { BigQuery, Query, QueryOptions, SimpleQueryRowsResponse } from '@google-cloud/bigquery';

export class BigQueryQueryRunner {

	constructor() {
	}

	public runQuery(
		queryText: string,
		maxResults: number = 10,
		pageToken: string | null = null,
		startIndex: string = '0'
	): Promise<SimpleQueryRowsResponse> {

		const bqclient = new BigQuery();

		const query: Query = {
			dryRun: false,
			query: queryText
		};

		const options: QueryOptions = {
			autoPaginate: false,
			wrapIntegers: true,
			maxResults: maxResults,
			startIndex: startIndex
		};
		if (pageToken) { options.pageToken = pageToken; }

		return bqclient.query(query, options);
	}

}