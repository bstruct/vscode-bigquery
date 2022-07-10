import { Job } from "@google-cloud/bigquery";

export interface ResultsGridRenderRequest {
    jobsPromise: Promise<Job[]>;
    startIndex: number;
    maxResults: number;
    jobIndex: number;
    //true if results should have a button to open in another tab
    openInTabVisible: boolean;
}