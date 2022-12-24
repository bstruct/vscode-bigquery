import { TextEditor } from 'vscode';
import { ResultsGridRenderRequest } from './tableResultsPanel/resultsGridRenderRequest';

interface JobReference {
    projectId: string,
    jobId: string,
    location: string,
}

interface QueryResultsMapping {
    uuid: string,
    textEditor: TextEditor,
    jobReferences: JobReference[] | undefined,
    jobIndex: number | undefined,
}