import { WebviewPanel } from "vscode";

interface JobReference {
    projectId: string,
    jobId: string,
    location: string,
}

interface QueryResultsMapping {
    uuid: string,
    textEditorUriString: string,
    jobReferences: JobReference[] | undefined,
    jobIndex: number | undefined,
    webviewPanel: WebviewPanel | undefined,
}