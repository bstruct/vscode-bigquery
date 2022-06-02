import { BigQuery, Job, JobResponse, Query } from '@google-cloud/bigquery';

export class BigQueryQueryRunner {

	public runQuery(queryText: string): Promise<Job[]> {

		const bqclient = new BigQuery();

		const query: Query = {
			dryRun: false,
			query: queryText,
			useLegacySql: false,
		};

		return bqclient
			.createQueryJob(query)
			.then(async (jobResponse) => {

				const jobId = jobResponse[1].jobReference?.jobId || '';

				const statementType: string = jobResponse[1].statistics?.query?.statementType || '';

				//If the query is a 'SCRIPT', means that there's multiple jobs involved.
				// Can be multiple select statements, but also declaring variables is another `job`
				if (statementType === 'SCRIPT') {

					// in this case, only after the parent jobs is 'DONE', it constains the list 
					// of all the jobs involved

					// jobResponse[0].get()
					// const j = bqclient.job('');
					
					const jobsResponse = await bqclient.getJobs({ parentJobId: jobId });

					const jobs: Job[] = jobsResponse[0];

					return jobs;

				} else {
					return [jobResponse[0]];
				}

			});

	}

}