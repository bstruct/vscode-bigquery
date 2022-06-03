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
			.then((jobResponse: JobResponse) => {

				const job = jobResponse[0];

				return new Promise((resolve, reject) => {

					job.on('complete', (metadata) => {

						const jobMeta = jobResponse[1];
						const statementType: string = jobMeta.statistics?.query?.statementType || '';

						//If the query is a 'SCRIPT', means that there's multiple jobs involved.
						// Can be multiple select statements, but also declaring variables is another `job`
						if (statementType === 'SCRIPT') {

							const jobId = jobMeta.jobReference?.jobId || '';

							// in this case, only after the parent jobs is 'DONE', it constains the list 
							// of all the jobs involved
							const _ = bqclient.getJobs({ parentJobId: jobId })
								.then((getJobsResponse) => {

									const jobs: Job[] = getJobsResponse[0];

									const sortedJobs = jobs.sort((a: Job, b: Job) => {

										const id1 = a.id || '';
										const id2 = b.id || '';

										const n1 = Number(id1.substring(id1.lastIndexOf('_') + 1));
										const n2 = Number(id2.substring(id2.lastIndexOf('_') + 1));

										return n1 > n2 ? 1 : -1;
									});

									resolve(sortedJobs);
								})
								.catch((err) => { reject(err); });

						} else {
							resolve([job]);
						}
					});

					job.on('error', (error) => {
						reject(error);
					});

				});

			});

	}

}