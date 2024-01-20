import { Job } from "@google-cloud/bigquery";

export enum ResultsGridRenderRequestV2Type {
  clear = "clear",
  executeQuery = "execute_query",
  previewTable = "preview_table",
  error = "error"
}

export interface ResultsGridRenderRequestV2Error {
  message: String,
  reason: String | null
}

export interface ResultsGridRenderRequestV2 {
  requestType: String;
  projectId: String | null;
  token: String | null;
  job: Job | null;
  error: ResultsGridRenderRequestV2Error | null;
}