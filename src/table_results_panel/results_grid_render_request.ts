import { Job } from "@google-cloud/bigquery";

export interface ResultsGridRenderRequest {
    jobsPromise: Promise<Job[]>;
    startIndex: number;
    maxResults: number;
    jobIndex: number;
    openInTabVisible: boolean;
}