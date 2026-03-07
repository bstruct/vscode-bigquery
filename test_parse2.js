const { BigQuery } = require('@google-cloud/bigquery');

// Simulate the job object from @google-cloud/bigquery
const simulatedJob = {
  metadata: {
    kind: "bigquery#job",
    etag: "some-etag",
    id: "some-id",
    selfLink: "https://bigquery.googleapis.com/bigquery/v2/...",
    user_email: "test@example.com",
    configuration: {
       jobType: "QUERY",
       query: { query: "SELECT 1" }
    },
    jobReference: {
      projectId: "my-project",
      jobId: "some-job-id",
      location: "US"
    },
    statistics: {
      creationTime: "1234567890", // Usually string, sometimes might be number
      startTime: "1234567891",
      endTime: "1234567892",
      totalBytesProcessed: "0",
      query: {
         statementType: "SELECT"
      }
    },
    status: {
      state: "DONE"
    }
  }
};

const externalRequest = {
  requestType: "execute_query",
  projectId: "my-project",
  token: "my-token",
  job: simulatedJob.metadata,
  error: null
};

console.log(JSON.stringify(externalRequest, null, 2));
