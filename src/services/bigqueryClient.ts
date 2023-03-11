import { BigQuery, Job, JobResponse, Query, Table } from '@google-cloud/bigquery';
import { BigqueryJobError } from './bigqueryJobError';
import { BigqueryTableSchema } from './bigqueryTableSchema';
import { JobReference } from './queryResultsMapping';
import { SchemaField, TableMetadata } from './tableMetadata';

export class BigQueryClient {

	private bqclient = new BigQuery();

	public async runQuery(queryText: string): Promise<Job[]> {

		const query: Query = {
			dryRun: false,
			query: queryText,
			useLegacySql: false,
			useQueryCache: true
		};

		const jobResponse: JobResponse = await this.bqclient.createQueryJob(query);

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
					this.bqclient
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

	}

	public async validateQuery(queryText: string): Promise<[number | null, BigqueryJobError | null]> {

		const query: Query = {
			dryRun: true,
			query: queryText,
			useLegacySql: false,
			useQueryCache: true
		};

		let error: BigqueryJobError | null = null;

		try {
			const queryJob = await this.bqclient.createQueryJob(query);

			if (queryJob[1] && queryJob[1].statistics && queryJob[1].statistics.totalBytesProcessed) {
				const totalBytesProcessed = queryJob[1].statistics.totalBytesProcessed;
				if (Number.parseInt(totalBytesProcessed)) {
					return [Number.parseInt(totalBytesProcessed), null];
				}
			}

		} catch (err) {
			error = err as BigqueryJobError;
		}

		return [null, error];
	}

	public getTable(projectId: string, datasetId: string, tableId: string): Table {
		return this.bqclient.dataset(datasetId, { projectId: projectId }).table(tableId);
	}

	public getMetadata(projectId: string, datasetId: string, tableId: string): Promise<TableMetadata> {

		const metadataPromise = this.bqclient
			.dataset(datasetId, { projectId: projectId })
			.table(tableId)
			.getMetadata();

		const fullSchema = this.runQuery(`
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
			.then(this.onfulfilled);

	}

	public async getTableSchema(projectId: string, datasetName: string, tableName: string): Promise<BigqueryTableSchema[]> {

		const query = `
SELECT 
	colums.table_catalog AS project_id,
	colums.table_schema AS dataset_name,
	colums.table_name,
	colums.column_name,
	colums.ordinal_position,
	colums.data_type,
  	colums.is_partitioning_column,
  	paths.description,
FROM \`${projectId}.${datasetName}\`.INFORMATION_SCHEMA.COLUMNS colums
  LEFT JOIN \`${projectId}.${datasetName}\`.INFORMATION_SCHEMA.COLUMN_FIELD_PATHS paths USING(table_catalog, table_schema, table_name, column_name)
WHERE table_name = '${tableName}' AND is_hidden = 'NO';
`;

		const q = await this.runQuery(query);

		const results = await q[0].getQueryResults();

		return results[0].map(c => c as BigqueryTableSchema);

	}

	public getJob(jobReference: JobReference): Job {
		return this.bqclient.job(jobReference.jobId, { location: jobReference.location, projectId: jobReference.projectId });
	}

	private onfulfilled(value: [any, any]): TableMetadata {

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
				schemaItem.collation = extra.collationName === 'NULL' ? '' : extra.collationName;
				schemaItem.description = extra.description;
			}

			if (schemaItem.fields && schemaItem.fields.length > 0) {
				schemaItem.fields = this.schemaEnrich(fieldPath, schemaItem.fields, extraInformation);
			}

			newSchemaItems.push(schemaItem);
		}

		return newSchemaItems;
	}

	//GET https://bigquery.googleapis.com/bigquery/v2/projects
	public async getProjects(): Promise<gapi.client.bigquery.ProjectList> {

		const request = this.bqclient.makeAuthenticatedRequest({ uri: 'https://bigquery.googleapis.com/bigquery/v2/projects', method: 'GET' });

		return new Promise((resolve, reject) => {

			request.on('data', (stream) => {
				const response = stream.toString('utf-8');
				resolve(JSON.parse(response) as gapi.client.bigquery.ProjectList);
			});

			request.on('error', (error) => {
				console.log(error);
				reject(error);
			});

		});

	}

}
