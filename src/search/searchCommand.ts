import * as vscode from 'vscode';
import { SearchPanel } from './searchPanel';
import { DataplexSearchService } from '../services/dataplexSearchService';
import { getBigQueryClient, SETTING_PINNED_PROJECTS, SETTING_PROJECTS, SETTING_TABLES } from '../extensionCommands';
import { Authentication } from '../services/authentication';
import { getExtensionUri } from '../extension';
import { BigqueryTreeItemType } from '../activitybar/bigqueryTreeItem';
import { QueryGeneratorService } from '../services/queryGeneratorService';

export const COMMAND_SEARCH = 'vscode-bigquery.search';

/** Cancellation handle for in-flight searches */
let _currentSearchSignal: { cancelled: boolean } | null = null;

/** Pagination state — kept across load-more requests */
let _currentTerm     = '';
let _nextPageToken: string | undefined;
let _currentProjects: string[] = [];

/**
 * Opens (or reveals) the BigQuery Search editor-tab panel.
 * Wires all callbacks on the panel.
 */
export const commandSearch = async function (..._args: any[]) {
    const panel = SearchPanel.open(getExtensionUri());

    panel.onSearch = async (term: string) => {
        _currentTerm  = term;
        _nextPageToken = undefined;
        _currentProjects = await resolveProjectIds();
        await runSearch(panel, term, _currentProjects, undefined);
    };

    panel.onLoadMore = async () => {
        if (!_nextPageToken) { return; }
        await loadMore(panel);
    };

    panel.onOpenTable = async (projectId, datasetId, tableId, treeItemType) => {
        if (treeItemType === BigqueryTreeItemType.model) {
            // Open ALTER MODEL template as a new editor
            const content =
                `ALTER MODEL \`${projectId}.${datasetId}.${tableId}\`\n` +
                `SET OPTIONS (\n` +
                `\t-- friendly_name = 'My Model',\n` +
                `\t-- expiration_timestamp = TIMESTAMP '2030-01-01 00:00:00 UTC'\n` +
                `);`;
            const doc = await vscode.workspace.openTextDocument({ language: 'bqsql', content });
            await vscode.commands.executeCommand('vscode.open', doc.uri);
        } else {
            await vscode.commands.executeCommand(
                'vscode-bigquery.view-table',
                { treeItemType, projectId, datasetId, tableId, label: tableId }
            );
        }
    };

    panel.onCreateQuery = async (projectId, datasetId, tableId, treeItemType) => {
        if (treeItemType === BigqueryTreeItemType.model) {
            const content = QueryGeneratorService.generateModelQuery(projectId, datasetId, tableId);
            const doc = await vscode.workspace.openTextDocument({ language: 'bqsql', content });
            await vscode.commands.executeCommand('vscode.open', doc.uri);
        } else {
            await vscode.commands.executeCommand(
                'vscode-bigquery.create-table-default-query',
                { treeItemType, projectId, datasetId, tableId, label: tableId }
            );
        }
    };
};

// ── Internal ────────────────────────────────────────────────────────────────

async function runSearch(
    panel: SearchPanel,
    term: string,
    projectIds: string[],
    pageToken: string | undefined,
): Promise<void> {
    // Cancel any previous in-flight search
    if (_currentSearchSignal) { _currentSearchSignal.cancelled = true; }
    const signal = { cancelled: false };
    _currentSearchSignal = signal;

    panel.renderSearching(term);

    try {
        if (projectIds.length === 0) {
            panel.renderError(term, 'No GCP projects found. Make sure you are authenticated and have at least one project configured.');
            return;
        }

        const bqClient = await getBigQueryClient();
        const token = await bqClient.getToken();
        if (!token) {
            panel.renderError(term, 'Not authenticated. Please sign in via the BigQuery extension.');
            return;
        }

        const page = await DataplexSearchService.search(term, projectIds[0], token, pageToken, 10, signal);

        if (signal.cancelled) { return; }

        _nextPageToken = page.nextPageToken;
        panel.renderResults(term, page.results, page.nextPageToken);

    } catch (err) {
        if (signal.cancelled) { return; }
        const msg = (err as any)?.message ?? String(err);
        // Surface a helpful hint if the API is simply not enabled
        const hint = msg.includes('disabled') || msg.includes('403')
            ? ' (Hint: enable the Dataplex API at console.cloud.google.com/apis/library/dataplex.googleapis.com)'
            : '';
        panel.renderError(term, msg + hint);
    }
}

async function loadMore(panel: SearchPanel): Promise<void> {
    if (!_nextPageToken) { return; }

    if (_currentSearchSignal) { _currentSearchSignal.cancelled = true; }
    const signal = { cancelled: false };
    _currentSearchSignal = signal;

    panel.renderLoadingMore();

    try {
        const bqClient = await getBigQueryClient();
        const token = await bqClient.getToken();
        if (!token) { panel.renderError(_currentTerm, 'Not authenticated. Please sign in via the BigQuery extension.'); return; }

        const page = await DataplexSearchService.search(
            _currentTerm, _currentProjects[0], token, _nextPageToken, 10, signal
        );

        if (signal.cancelled) { return; }

        _nextPageToken = page.nextPageToken;
        panel.renderMoreResults(page.results, page.nextPageToken);

    } catch (err) {
        if (signal.cancelled) { return; }
        const msg = (err as any)?.message ?? String(err);
        panel.renderError(_currentTerm, msg);
    }
}

/**
 * Assembles the full list of project IDs the extension knows about.
 */
async function resolveProjectIds(): Promise<string[]> {

    const seen = new Set<string>();
    const ids: string[] = [];

    const add = (id: string) => {
        const lower = id.toLowerCase().trim();
        if (lower && !seen.has(lower)) { seen.add(lower); ids.push(lower); }
    };

    try {
        const defaultId = await Authentication.getDefaultProjectId();
        if (defaultId) { add(defaultId); }
    } catch (_) { /* ok */ }

    const configuredProjects = (vscode.workspace.getConfiguration().get(SETTING_PROJECTS) as string[] ?? []);
    configuredProjects.forEach(p => add(p));

    const configuredTables = (vscode.workspace.getConfiguration().get(SETTING_TABLES) as string[] ?? []);
    configuredTables.map(t => t.split('.')[0]).filter(Boolean).forEach(p => add(p));

    try {
        const bqClient = await getBigQueryClient();
        const projectList = await bqClient.getProjects();
        for (const project of projectList.projects ?? []) {
            if (project.id) { add(project.id); }
        }
    } catch (_) { /* no additional projects */ }

    const pinned = (vscode.workspace.getConfiguration().get(SETTING_PINNED_PROJECTS) as string[] ?? [])
        .map(p => p.toLowerCase());

    ids.sort((a, b) => {
        const aP = pinned.indexOf(a) >= 0;
        const bP = pinned.indexOf(b) >= 0;
        if (aP && !bP) { return -1; }
        if (!aP && bP) { return 1; }
        return a.localeCompare(b);
    });

    return ids;
}
