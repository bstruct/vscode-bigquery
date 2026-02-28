import { BigQuery, Job, JobResponse, Query, Table } from '@google-cloud/bigquery';
import { BigqueryJobError } from './bigqueryJobError';
import { BigqueryTableSchema } from './bigqueryTableSchema';
import { JobReference } from './queryResultsMapping';
import { SchemaField, TableMetadata } from './tableMetadata';

export class BigQueryClient {

	private bqclient: BigQuery;

	/**
	 *
	 */
	constructor(projId: string | undefined) {
		this.bqclient = new BigQuery({ 'projectId': projId });
	}

	getToken(): Promise<string | null> {
		return this.bqclient.authClient.getAccessToken()
			.then(value => {
				return value || null;
			});
	}
	getProjectId(): Promise<string | null> {
		return this.bqclient.getProjectId()
			.then(value => {
				return value || null;
			});
	}

	public async runQuery(queryText: string): Promise<Job> {

		const query: Query = {
			dryRun: false,
			query: queryText,
			useLegacySql: false,
			useQueryCache: true
		};

		const jobResponse: JobResponse = await this.bqclient.createQueryJob(query);

		const job = jobResponse[0];

		return job;

		// return new Promise((resolve, reject) => {

		// 	job.on('complete', (metadata) => {

		// 		const jobMeta = jobResponse[1];
		// 		const statementType: string = jobMeta.statistics?.query?.statementType || '';

		// 		//If the query is a 'SCRIPT', means that there's multiple jobs involved.
		// 		// Can be multiple select statements, but also declaring variables is another `job`
		// 		if (statementType === 'SCRIPT') {

		// 			const jobId = jobMeta.jobReference?.jobId || '';

		// 			// in this case, only after the parent jobs is 'DONE', it constains the list 
		// 			// of all the jobs involved.
		// 			// jobs will have id's postfixed
		// 			this.bqclient
		// 				.getJobs({ parentJobId: jobId })
		// 				.then((getJobsResponse) => {

		// 					const jobs: Job[] = getJobsResponse[0];

		// 					const sortedJobs = jobs.sort((a: Job, b: Job) => {

		// 						const id1 = a.id || '';
		// 						const id2 = b.id || '';

		// 						const n1 = Number(id1.substring(id1.lastIndexOf('_') + 1));
		// 						const n2 = Number(id2.substring(id2.lastIndexOf('_') + 1));

		// 						return n1 > n2 ? 1 : -1;
		// 					});

		// 					resolve(sortedJobs);
		// 				})
		// 				.catch((err) => { reject(err); });

		// 		} else {
		// 			resolve([job]);
		// 		}
		// 	});

		// 	job.on('error', (error) => {
		// 		reject(error);
		// 	});

		// });

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

		/**
		 * Previously this method ran a BigQuery job against
		 * INFORMATION_SCHEMA.COLUMN_FIELD_PATHS to obtain per-field descriptions
		 * and collation names.  The BigQuery REST API (tables.get) already embeds
		 * descriptions on every field – including deeply-nested RECORD fields –
		 * and collation under `collationSpec`.  We read that data directly here,
		 * avoiding a billable query job.
		 *
		 * API reference:
		 *   GET https://bigquery.googleapis.com/bigquery/v2/projects/{p}/datasets/{d}/tables/{t}
		 */
		return this.bqclient
			.dataset(datasetId, { projectId: projectId })
			.table(tableId)
			.getMetadata()
			.then(([rawMetadata]) => {
				const metadata = rawMetadata as TableMetadata;
				// Normalise collationSpec → collation so the rest of the codebase
				// keeps working with the existing SchemaField.collation property.
				BigQueryClient.normaliseCollation(metadata.schema?.fields ?? []);
				return metadata;
			});

	}

	public async getTableSchema(projectId: string, datasetName: string, tableName: string): Promise<BigqueryTableSchema[]> {

		/**
		 * Previously this ran two INFORMATION_SCHEMA queries (COLUMNS +
		 * COLUMN_FIELD_PATHS) to build the column list.  The BigQuery REST API
		 * tables.get endpoint already returns the complete schema tree, including
		 * descriptions and collation, at no query cost.  We flatten it here into
		 * the same BigqueryTableSchema[] shape the rest of the codebase expects.
		 *
		 * API reference:
		 *   GET https://bigquery.googleapis.com/bigquery/v2/projects/{p}/datasets/{d}/tables/{t}
		 */
		const [rawMetadata] = await this.bqclient
			.dataset(datasetName, { projectId })
			.table(tableName)
			.getMetadata();

		const metadata = rawMetadata as TableMetadata;
		const partitioningField: string | null = metadata.timePartitioning?.field ?? null;

		const result: BigqueryTableSchema[] = [];
		BigQueryClient.flattenSchemaFields(
			projectId,
			datasetName,
			tableName,
			metadata.schema?.fields ?? [],
			partitioningField,
			result,
			null,
		);
		return result;

	}

	public getJob(jobReference: JobReference): Job {
		return this.bqclient.job(jobReference.jobId, { location: jobReference.location, projectId: jobReference.projectId });
	}

	// ─── DDL via BigQuery & Routines REST APIs (no INFORMATION_SCHEMA) ──────────

	/**
	 * Return the DDL text for a table, view, materialized view, routine, or ML
	 * model by calling the appropriate BigQuery REST API endpoint.
	 *
	 * Replaces INFORMATION_SCHEMA.TABLES / .ROUTINES / .MODELS queries that
	 * previously ran a billable job.
	 *
	 * REST references:
	 *   tables.get    → GET …/bigquery/v2/projects/{p}/datasets/{d}/tables/{t}
	 *   routines.get  → GET …/bigquery/v2/projects/{p}/datasets/{d}/routines/{r}
	 *   models.get    → GET …/bigquery/v2/projects/{p}/datasets/{d}/models/{m}
	 */
	public async getDdl(
		projectId: string,
		datasetId: string,
		resourceId: string,
		resourceType: 'table' | 'routine' | 'model',
	): Promise<string> {

		const base = 'https://bigquery.googleapis.com/bigquery/v2/projects';

		if (resourceType === 'routine') {
			const uri = `${base}/${encodeURIComponent(projectId)}/datasets/${encodeURIComponent(datasetId)}/routines/${encodeURIComponent(resourceId)}`;
			const raw = await this.makeAuthenticatedGet(uri);
			const routine = JSON.parse(raw);
			return BigQueryClient.buildRoutineDdl(projectId, datasetId, routine);
		}

		if (resourceType === 'model') {
			const uri = `${base}/${encodeURIComponent(projectId)}/datasets/${encodeURIComponent(datasetId)}/models/${encodeURIComponent(resourceId)}`;
			const raw = await this.makeAuthenticatedGet(uri);
			const model = JSON.parse(raw);
			return BigQueryClient.buildModelDdl(projectId, datasetId, resourceId, model);
		}

		// table / view / materialized-view
		const uri = `${base}/${encodeURIComponent(projectId)}/datasets/${encodeURIComponent(datasetId)}/tables/${encodeURIComponent(resourceId)}`;
		const raw = await this.makeAuthenticatedGet(uri);
		const tableJson = JSON.parse(raw);
		return BigQueryClient.buildTableDdl(projectId, datasetId, resourceId, tableJson);
	}

	// ─── DDL reconstruction helpers ─────────────────────────────────────────────

	/** Reconstruct CREATE TABLE / VIEW / MATERIALIZED VIEW DDL from a tables.get response. */
	private static buildTableDdl(projectId: string, datasetId: string, tableId: string, t: any): string {
		const fqn = `\`${projectId}.${datasetId}.${tableId}\``;

		if (t.type === 'VIEW') {
			const query: string = t.view?.query ?? '';
			return `CREATE OR REPLACE VIEW ${fqn}\nAS ${query}`;
		}

		if (t.type === 'MATERIALIZED_VIEW') {
			const query: string = t.materializedView?.query ?? '';
			return `CREATE OR REPLACE MATERIALIZED VIEW ${fqn}\nAS ${query}`;
		}

		// Regular TABLE (including EXTERNAL)
		const fields: SchemaField[] = t.schema?.fields ?? [];
		const columnsDdl = BigQueryClient.buildColumnsDdl(fields, 0);
		let ddl = `CREATE TABLE ${fqn}\n(\n${columnsDdl}\n)`;

		if (t.timePartitioning) {
			const tp = t.timePartitioning;
			ddl += tp.field
				? `\nPARTITION BY DATE(${tp.field})`
				: `\nPARTITION BY _PARTITIONDATE`;
		} else if (t.rangePartitioning) {
			const rp = t.rangePartitioning;
			const r = rp.range ?? {};
			ddl += `\nPARTITION BY RANGE_BUCKET(${rp.field}, GENERATE_ARRAY(${r.start}, ${r.end}, ${r.interval}))`;
		}

		if (t.clustering?.fields?.length > 0) {
			ddl += `\nCLUSTER BY ${(t.clustering.fields as string[]).join(', ')}`;
		}

		const options: string[] = [];
		if (t.description) {
			options.push(`description="${(t.description as string).replace(/\\/g, '\\\\').replace(/"/g, '\\"')}"`);
		}
		if (options.length > 0) {
			ddl += `\nOPTIONS(\n  ${options.join(',\n  ')}\n)`;
		}

		return ddl + ';';
	}

	/** Recursively emit column definitions, handling RECORD/STRUCT nesting. */
	private static buildColumnsDdl(fields: SchemaField[], depth: number): string {
		const pad = '  '.repeat(depth + 1);
		return fields.map(f => {
			let typeStr = (f.type ?? 'STRING').toUpperCase();

			if ((typeStr === 'RECORD' || typeStr === 'STRUCT') && f.fields?.length > 0) {
				const nested = BigQueryClient.buildColumnsDdl(f.fields, depth + 1);
				const closePad = '  '.repeat(depth + 1);
				typeStr = `STRUCT<\n${nested}\n${closePad}>`;
			}

			const isRepeated = (f.mode ?? '').toUpperCase() === 'REPEATED';
			const finalType = isRepeated ? `ARRAY<${typeStr}>` : typeStr;
			const notNull = (f.mode ?? '').toUpperCase() === 'REQUIRED' ? ' NOT NULL' : '';
			const descOpt = f.description
				? ` OPTIONS(description="${f.description.replace(/\\/g, '\\\\').replace(/"/g, '\\"')}")`
				: '';

			return `${pad}${f.name} ${finalType}${notNull}${descOpt}`;
		}).join(',\n');
	}

	/**
	 * Reconstruct DDL from a routines.get response.
	 *
	 * REST reference:
	 *   GET https://bigquery.googleapis.com/bigquery/v2/projects/{p}/datasets/{d}/routines/{r}
	 */
	private static buildRoutineDdl(projectId: string, datasetId: string, r: any): string {
		const routineId: string = r.routineReference?.routineId ?? r.routineId ?? '';
		const fqn = `\`${projectId}.${datasetId}.${routineId}\``;

		const routineType: string = (r.routineType ?? 'SCALAR_FUNCTION').toUpperCase();
		const language: string = (r.language ?? 'SQL').toUpperCase();
		const body: string = r.definitionBody ?? '';
		const args: any[] = r.arguments ?? [];

		const argList = args.map((a: any) => {
			const argMode: string = a.argumentKind === 'ANY_TYPE' ? '' : (a.mode ? `${a.mode} ` : '');
			const argType: string = a.dataType?.typeKind ?? 'ANY TYPE';
			return `${argMode}${a.name ?? ''} ${argType}`.trim();
		}).join(',\n    ');

		if (routineType === 'PROCEDURE') {
			return `CREATE OR REPLACE PROCEDURE ${fqn}(\n    ${argList}\n)\nBEGIN\n${body}\nEND;`;
		}

		if (routineType === 'TABLE_VALUED_FUNCTION') {
			return `CREATE OR REPLACE TABLE FUNCTION ${fqn}(\n    ${argList}\n)\nRETURNS TABLE<...>\nAS (\n${body}\n);`;
		}

		// SCALAR_FUNCTION (and other function-like types)
		const retTypeKind: string = r.returnType?.typeKind ?? r.returnTableType?.columns ? '' : 'ANY TYPE';
		const langClause = language !== 'SQL' ? `\nLANGUAGE ${language}\nAS r"""\n${body}\n"""` : `\nAS (\n${body}\n)`;
		return `CREATE OR REPLACE FUNCTION ${fqn}(\n    ${argList}\n)\nRETURNS ${retTypeKind}${langClause};`;
	}

	/**
	 * Build a descriptive representation for a BigQuery ML model using a
	 * models.get response.  The REST API does not expose DDL text, so we emit
	 * a best-effort CREATE MODEL stub with the model type and training options
	 * as they would appear in `INFORMATION_SCHEMA.MODELS`.
	 *
	 * REST reference:
	 *   GET https://bigquery.googleapis.com/bigquery/v2/projects/{p}/datasets/{d}/models/{m}
	 */
	private static buildModelDdl(projectId: string, datasetId: string, modelId: string, m: any): string {
		const fqn = `\`${projectId}.${datasetId}.${modelId}\``;
		const modelType: string = m.modelType ?? 'UNKNOWN';
		const description: string = m.description ? `\n  description="${m.description}"` : '';
		const labelCols: string = (m.labelColumns ?? []).map((c: any) => c.name).join(', ');
		const featureCols: string = (m.featureColumns ?? []).map((c: any) => c.name).join(', ');

		const lines: string[] = [
			`-- Model type : ${modelType}`,
			`-- Label cols : ${labelCols || '(none)'}`,
			`-- Feature cols: ${featureCols || '(none)'}`,
			`-- Fetched via BigQuery models.get REST API (no billable job).`,
			``,
			`CREATE OR REPLACE MODEL ${fqn}`,
			`OPTIONS(`,
			`  model_type='${modelType}'${description}`,
			`)`,
			`AS`,
			`-- Re-run your original training query here.`,
			`;`,
		];
		return lines.join('\n');
	}

	// ─── REST API helpers ────────────────────────────────────────────────────────

	/** Issue an authenticated GET request and return the response body as a string. */
	private makeAuthenticatedGet(uri: string): Promise<string> {
		const request = this.bqclient.makeAuthenticatedRequest({ uri, method: 'GET' });
		return new Promise((resolve, reject) => {
			const chunks: Uint8Array[] = [];
			request.on('data', (chunk: Uint8Array) => { chunks.push(chunk); });
			request.on('end', () => { resolve(Buffer.concat(chunks).toString('utf-8')); });
			request.on('error', reject);
		});
	}

	// ─── Schema normalisation helpers ───────────────────────────────────────────

	/**
	 * The BigQuery REST API returns field collation under `collationSpec`.
	 * Normalise it to `collation` (the property the rest of the codebase uses)
	 * for every field in the tree in-place.
	 */
	private static normaliseCollation(fields: SchemaField[]): void {
		for (const f of fields) {
			if ((f as any).collationSpec !== undefined) {
				f.collation = (f as any).collationSpec ?? '';
			}
			if (f.fields?.length > 0) {
				BigQueryClient.normaliseCollation(f.fields);
			}
		}
	}

	/**
	 * Recursively flatten a schema field tree into the `BigqueryTableSchema[]`
	 * row format expected by the completion / schema service.
	 *
	 * Nested RECORD fields are expanded using dot-separated paths, mirroring
	 * what INFORMATION_SCHEMA.COLUMN_FIELD_PATHS returned.
	 */
	private static flattenSchemaFields(
		projectId: string,
		datasetName: string,
		tableName: string,
		fields: SchemaField[],
		partitioningField: string | null,
		result: BigqueryTableSchema[],
		prefix: string | null,
		ordinalBase: number = 0,
	): void {
		for (let i = 0; i < fields.length; i++) {
			const f = fields[i];
			const columnName = prefix ? `${prefix}.${f.name}` : f.name;

			result.push({
				project_id: projectId,
				dataset_name: datasetName,
				table_name: tableName,
				column_name: columnName,
				ordinal_position: String(ordinalBase + i + 1),
				data_type: f.type ?? '',
				is_partitioning_column: columnName === partitioningField ? 'YES' : 'NO',
				description: f.description ?? '',
			});

			if (f.fields?.length > 0) {
				BigQueryClient.flattenSchemaFields(
					projectId,
					datasetName,
					tableName,
					f.fields,
					partitioningField,
					result,
					columnName,
					0,
				);
			}
		}
	}

	//GET https://bigquery.googleapis.com/bigquery/v2/projects
	public async getProjects(): Promise<gapi.client.bigquery.ProjectList> {

		const request = this.bqclient.makeAuthenticatedRequest({ uri: 'https://bigquery.googleapis.com/bigquery/v2/projects', method: 'GET' });

		return new Promise((resolve, reject) => {

			let responseBody: Uint8Array[] = [];

			request.on('data', (chunk) => {
				responseBody.push(chunk);
			});

			request.on('end', () => {
				const responseBodyString = Buffer.concat(responseBody).toString('utf-8');
				resolve(JSON.parse(responseBodyString) as gapi.client.bigquery.ProjectList);
			});

			request.on('error', (error) => {
				console.log(error);
				reject(error);
			});

		});

	}

}
