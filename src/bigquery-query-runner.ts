import { BigQuery, Query, QueryOptions, RowMetadata, SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import bigquery from '@google-cloud/bigquery/build/src/types';

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

		// const queryResult: any = await bqclient.query(query, options);
		// // const rows: RowMetadata[] = queryResult[0];
		// const job: bigquery.IGetQueryResultsResponse = queryResult[2];

		return bqclient.query(query, options);
	}

}