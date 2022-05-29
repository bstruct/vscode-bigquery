import { BigQuery, JobResponse, Query } from '@google-cloud/bigquery';

export class BigQueryQueryRunner {

	public runQuery(queryText: string): Promise<JobResponse> {

		const bqclient = new BigQuery();

		const query: Query = {
			dryRun: false,
			query: queryText
		};

		// const options: QueryOptions = {
		// 	autoPaginate: false,
		// 	wrapIntegers: true,
		// 	maxResults: maxResults,
		// 	startIndex: startIndex
		// };
		// if (pageToken) { options.pageToken = pageToken; }

		// return bqclient.query(query, options);

		return bqclient.createQueryJob(query);
	}

}