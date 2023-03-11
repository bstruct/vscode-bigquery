import { WebviewPanel } from "vscode";
import { QueryResultsVisualizationType } from "./queryResultsVisualizationType";

interface JobReference {
    projectId: string,
    jobId: string,
    location: string,
}

interface QueryResultsMapping {
    uuid: string,
    visualizationType: QueryResultsVisualizationType,
    textEditorUriString: string,
    jobReferences: JobReference[] | undefined,
    jobIndex: number | undefined,
    webviewPanel: WebviewPanel | undefined,
}