import { BigQuery, Query, QueryOptions, SimpleQueryRowsResponse } from '@google-cloud/bigquery';

export class BigQueryQueryRunner {

	constructor() {
	}

	public runQuery(queryText: string): Promise<SimpleQueryRowsResponse> {

		const bqclient = new BigQuery();

		const query: Query = {
			dryRun: false,
			query: queryText
		};

		const options: QueryOptions = {
			autoPaginate: false,
			wrapIntegers: true,
			//this query should only carry the information about the location of the result
			maxResults: 10
		};

		return bqclient.query(query, options);
	}

}