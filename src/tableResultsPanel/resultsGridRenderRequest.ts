import { JobReference } from "../services/queryResultsMapping";

export interface ResultsGridRenderRequest {
    jobReferences: JobReference[];
    startIndex: number;
    maxResults: number;
    jobIndex: number;
    //true if results should have a button to open in another tab
    openInTabVisible: boolean;
}