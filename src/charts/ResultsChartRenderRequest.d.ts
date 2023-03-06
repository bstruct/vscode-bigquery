import { JobReference } from "../services/queryResultsMapping";
import { TableReference } from "../services/tableMetadata";

export interface ResultsChartRenderRequest {
    jobReferences: JobReference[] | undefined;
    tableReference: TableReference | undefined;
    jobIndex: number;
}