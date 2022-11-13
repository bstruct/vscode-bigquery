import { Job } from "@google-cloud/bigquery";

export interface DownloadCsvRequest {
    jobsPromise: Promise<Job[]>;
    jobIndex: number;
}