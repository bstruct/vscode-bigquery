import { BigQuery, JobResponse, Query } from '@google-cloud/bigquery';

export class BigQueryQueryRunner {

	public runQuery(queryText: string): Promise<JobResponse> {

		const bqclient = new BigQuery();

		const query: Query = {
			dryRun: false,
			query: queryText
		};

		return bqclient.createQueryJob(query);
	}

}