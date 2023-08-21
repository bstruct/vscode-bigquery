pub struct Jobs {
    token: String,
}

pub struct GetQueryResultsRequest {
    pub project_id: String,
    pub job_id: String,
    pub start_index: Option<String>,
    pub page_token: Option<String>,
    pub max_results: Option<u8>,
    pub timeout_ms: Option<u8>,
    pub location: Option<String>,
    // formatOptions: DataFormatOptions;
}

pub struct GetQueryResultsResponse {
    // kind: string;
    // etag: string;
    // schema: object;
    // jobReference: object;
    // totalRows: string;
    // pageToken: string;
    // rows: object;
    // totalBytesProcessed: string;
    // jobComplete: boolean;
    // errors: object;
    // cacheHit: boolean;
    // numDmlAffectedRows: string;
}

impl Jobs {
    pub fn new(token: &str) -> Jobs {
        Jobs {
            token: String::from(token),
        }
    }

    /*
    https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/getQueryResults#http-request
    */
    pub fn get_query_results(
        self: &Self,
        _request: GetQueryResultsRequest,
    ) -> GetQueryResultsResponse {
        GetQueryResultsResponse {

        // const url = new URL(`https://bigquery.googleapis.com/bigquery/v2/projects/${request.projectId}/queries/${request.jobId}`);

        // if (request.location) { url.searchParams.append("location", request.location); }
        // if (request.maxResults !== null) { url.searchParams.append("maxResults", request.maxResults.toString()); }
        // if (request.pageToken) { url.searchParams.append("pageToken", request.pageToken); }
        // if (request.startIndex) { url.searchParams.append("startIndex", request.startIndex); }
        // if (request.timeoutMs !== null) { url.searchParams.append("timeoutMs", request.timeoutMs.toString()); }

        // const response = fetch(url.href, {
        //     method: "GET",
        //     headers: {
        //         // eslint-disable-next-line @typescript-eslint/naming-convention
        //         "Content-Type": "application/json",
        //         // eslint-disable-next-line @typescript-eslint/naming-convention
        //         "Authorization": `Bearer ${this.token}`,
        //     }
        // });

        // debugger;


        // throw new Error();
        }
    }
}
