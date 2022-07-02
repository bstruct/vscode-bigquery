import { BigQuery, Job, JobResponse, Query, Table } from '@google-cloud/bigquery';

export class BigQueryClient {

	public static runQuery(queryText: string): Promise<Job[]> {

		const bqclient = new BigQuery();

		// bqclient.dataset('', {projectId:''}).table('').createReadStream({})

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
							// of all the jobs involved.
							// jobs will have id's postfixed
							bqclient
								.getJobs({ parentJobId: jobId })
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

	public static getTable(projectId: string, datasetId: string, tableId: string): Table {

		const bqclient = new BigQuery();

		return bqclient.dataset(datasetId, { projectId: projectId }).table(tableId);
	}

	public static getMetadata(projectId: string, datasetId: string, tableId: string): Promise<TableMetadata> {

		const bqclient = new BigQuery();

		const metadataPromise = bqclient
			.dataset(datasetId, { projectId: projectId })
			.table(tableId)
			.getMetadata();

		const fullSchema = BigQueryClient.runQuery(`
		SELECT 
			field_path AS fieldPath, 
			collation_name AS collationName, 
			description 
		FROM \`${projectId}.${datasetId}\`.INFORMATION_SCHEMA.COLUMN_FIELD_PATHS 
		WHERE table_name = '${tableId}';
		`).then(jobs => {
			return jobs[0].getQueryResults();
		});

		return Promise.all([metadataPromise, fullSchema])
			.then(BigQueryClient.onfulfilled);

	}

	private static onfulfilled(value: [any, any]): TableMetadata {

		const metadata = value[0][0] as TableMetadata;

		const extraInformation = value[1][0] as [{ fieldPath: string, collationName: string, description: string }];

		const fields = BigQueryClient.schemaEnrich(null, metadata.schema.fields, extraInformation);

		metadata.schema = { fields: fields };

		return metadata;
	}

	private static schemaEnrich(prefix: string | null, schemaItems: SchemaField[], extraInformation: [{ fieldPath: string, collationName: string, description: string }]): SchemaField[] {

		const newSchemaItems: SchemaField[] = [];

		for (let schemaItemIndex = 0; schemaItemIndex < schemaItems.length; schemaItemIndex++) {

			const schemaItem = schemaItems[schemaItemIndex];

			const fieldPath = `${prefix ? prefix : ''}${prefix ? '.' : ''}${schemaItem.name}`;
			const extra = extraInformation.find(c => c.fieldPath === fieldPath);
			if (extra) {
				schemaItem.collationName = extra.collationName === 'NULL' ? '' : extra.collationName;
				schemaItem.description = extra.description;
			}

			if (schemaItem.fields && schemaItem.fields.length > 0) {
				schemaItem.fields = BigQueryClient.schemaEnrich(fieldPath, schemaItem.fields, extraInformation);
			}

			newSchemaItems.push(schemaItem);
		}

		return newSchemaItems;
	}

}
