import { JobReference } from "../services/queryResultsMapping";
import { TableReference } from "../services/tableMetadata";

export interface ResultsGridRenderRequest {
    jobReferences: JobReference[] | undefined;
    tableReference: TableReference | undefined;
    startIndex: number;
    maxResults: number;
    jobIndex: number;
    //true if results should have a button to open in another tab
    openInTabVisible: boolean;
    token: string | null
}