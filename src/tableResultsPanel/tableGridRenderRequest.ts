import { Table } from "@google-cloud/bigquery";

export interface TableGridRenderRequest {
    table: Table;
    startIndex: number;
    maxResults: number;
    jobIndex: number;
    //true if results should have a button to open in another tab
    openInTabVisible: boolean;
}