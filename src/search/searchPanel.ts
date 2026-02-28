import * as vscode from 'vscode';
import { CatalogEntry } from '../services/dataplexSearchService';

export const SEARCH_PANEL_VIEW_TYPE = 'bigquery-search-panel';

type InboundMessage =
    | { command: 'search'; term: string }
    | { command: 'loadMore' }
    | { command: 'openTable';   projectId: string; datasetId: string; tableId: string; treeItemType: number }
    | { command: 'createQuery'; projectId: string; datasetId: string; tableId: string; treeItemType: number };

type OutboundMessage =
    | { command: 'searching';   term: string }
    | { command: 'results';     term: string; results: CatalogEntry[]; nextPageToken?: string }
    | { command: 'loadingMore' }
    | { command: 'moreResults'; results: CatalogEntry[]; nextPageToken?: string }
    | { command: 'error';       term: string; message: string };

export class SearchPanel {

    private static _instance: SearchPanel | undefined;

    private readonly _panel: vscode.WebviewPanel;
    private _disposed = false;

    public onSearch:      ((term: string) => void) | undefined;
    public onLoadMore:    (() => void) | undefined;
    public onOpenTable:   ((projectId: string, datasetId: string, tableId: string, treeItemType: number) => void) | undefined;
    public onCreateQuery: ((projectId: string, datasetId: string, tableId: string, treeItemType: number) => void) | undefined;

    // ── Static open/reveal ─────────────────────────────────────────────────

    static open(extensionUri: vscode.Uri): SearchPanel {
        if (SearchPanel._instance && !SearchPanel._instance._disposed) {
            SearchPanel._instance._panel.reveal(vscode.ViewColumn.One);
            return SearchPanel._instance;
        }

        const panel = vscode.window.createWebviewPanel(
            SEARCH_PANEL_VIEW_TYPE,
            'BigQuery Search',
            { viewColumn: vscode.ViewColumn.One, preserveFocus: false },
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [extensionUri],
            }
        );

        SearchPanel._instance = new SearchPanel(panel);
        return SearchPanel._instance;
    }

    // ── Constructor ────────────────────────────────────────────────────────

    private constructor(panel: vscode.WebviewPanel) {
        this._panel = panel;
        this._panel.webview.html = this._buildHtml();

        this._panel.webview.onDidReceiveMessage((msg: InboundMessage) => {
            switch (msg.command) {
                case 'search':       this.onSearch?.(msg.term); return;
                case 'loadMore':     this.onLoadMore?.(); return;
                case 'openTable':    this.onOpenTable?.(msg.projectId, msg.datasetId, msg.tableId, msg.treeItemType); return;
                case 'createQuery':  this.onCreateQuery?.(msg.projectId, msg.datasetId, msg.tableId, msg.treeItemType); return;
            }
        });

        this._panel.onDidDispose(() => {
            this._disposed = true;
            SearchPanel._instance = undefined;
        });
    }

    // ── Public API ─────────────────────────────────────────────────────────

    renderSearching(term: string): void {
        this._post({ command: 'searching', term });
    }

    renderResults(term: string, results: CatalogEntry[], nextPageToken?: string): void {
        this._post({ command: 'results', term, results, nextPageToken });
    }

    renderLoadingMore(): void {
        this._post({ command: 'loadingMore' });
    }

    renderMoreResults(results: CatalogEntry[], nextPageToken?: string): void {
        this._post({ command: 'moreResults', results, nextPageToken });
    }

    renderError(term: string, message: string): void {
        this._post({ command: 'error', term, message });
    }

    private _post(msg: OutboundMessage): void {
        if (!this._disposed) {
            this._panel.webview.postMessage(msg);
        }
    }

    // ── HTML ───────────────────────────────────────────────────────────────

    private _buildHtml(): string {
        const nonce = getNonce();
        return /* html */`<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta http-equiv="Content-Security-Policy"
      content="default-src 'none'; style-src 'nonce-${nonce}'; script-src 'nonce-${nonce}';" />
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<title>BigQuery Search</title>
<style nonce="${nonce}">
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: var(--vscode-font-family);
    font-size: var(--vscode-font-size);
    color: var(--vscode-foreground);
    background: var(--vscode-editor-background);
    height: 100vh; display: flex; flex-direction: column; overflow: hidden;
  }
  .search-bar {
    display: flex; align-items: center; gap: 8px;
    padding: 16px 20px 12px;
    border-bottom: 1px solid var(--vscode-panel-border); flex-shrink: 0;
  }
  .search-bar input {
    flex: 1; height: 28px; padding: 0 8px;
    background: var(--vscode-input-background);
    color: var(--vscode-input-foreground);
    border: 1px solid var(--vscode-input-border, transparent);
    outline: none; font-family: inherit; font-size: inherit;
  }
  .search-bar input:focus { border-color: var(--vscode-focusBorder); }
  .search-bar input::placeholder { color: var(--vscode-input-placeholderForeground); }
  .search-bar button {
    height: 28px; padding: 0 14px;
    background: var(--vscode-button-background);
    color: var(--vscode-button-foreground);
    border: none; cursor: pointer; font-family: inherit; font-size: inherit;
    white-space: nowrap; flex-shrink: 0;
  }
  .search-bar button:hover:not(:disabled) { background: var(--vscode-button-hoverBackground); }
  .search-bar button:disabled { opacity: 0.5; cursor: default; }

  .status-bar {
    padding: 5px 20px; font-size: 11px;
    color: var(--vscode-descriptionForeground);
    border-bottom: 1px solid var(--vscode-panel-border);
    flex-shrink: 0; min-height: 26px;
    display: flex; align-items: center; gap: 8px;
  }
  .status-bar.error  { color: var(--vscode-errorForeground); }
  .status-bar.hidden { visibility: hidden; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .spinner {
    width: 12px; height: 12px;
    border: 2px solid var(--vscode-descriptionForeground);
    border-top-color: transparent; border-radius: 50%;
    animation: spin 0.7s linear infinite; flex-shrink: 0;
  }

  .results { flex: 1; overflow-y: auto; }

  .entry {
    display: flex; align-items: flex-start; gap: 12px;
    padding: 10px 20px;
    border-bottom: 1px solid var(--vscode-panel-border);
    cursor: pointer;
  }
  .entry:hover { background: var(--vscode-list-hoverBackground); }
  .entry-icon { flex-shrink: 0; margin-top: 2px; opacity: 0.75; }
  .entry-body { flex: 1; min-width: 0; }
  .entry-name {
    display: flex; align-items: center; gap: 8px; margin-bottom: 2px;
  }
  .entry-name-text {
    font-weight: 600; white-space: nowrap;
    overflow: hidden; text-overflow: ellipsis;
    color: var(--vscode-textLink-foreground);
  }
  .entry:hover .entry-name-text { text-decoration: underline; }
  .entry-path {
    font-size: 11px; color: var(--vscode-descriptionForeground);
    margin-bottom: 2px; white-space: nowrap;
    overflow: hidden; text-overflow: ellipsis;
  }
  .kind-badge {
    display: inline-block;
    background: var(--vscode-badge-background);
    color: var(--vscode-badge-foreground);
    border-radius: 3px; padding: 0 4px;
    font-size: 10px; margin-right: 6px;
    text-transform: uppercase; letter-spacing: 0.04em;
  }
  .entry-desc {
    font-size: 11px; color: var(--vscode-descriptionForeground);
    overflow: hidden; display: -webkit-box;
    -webkit-line-clamp: 2; -webkit-box-orient: vertical;
  }
  .entry-actions {
    display: flex; flex-direction: row; gap: 4px; flex-shrink: 0;
  }
  .action-btn {
    background: transparent;
    color: var(--vscode-foreground);
    border: 1px solid var(--vscode-button-border, var(--vscode-panel-border));
    padding: 1px 8px; font-size: 11px;
    cursor: pointer; font-family: inherit;
    white-space: nowrap; border-radius: 2px; opacity: 0.7;
  }
  .action-btn:hover { opacity: 1; background: var(--vscode-toolbar-hoverBackground); }

  .load-more-row { padding: 12px 20px; text-align: center; }
  .load-more-btn {
    background: none; color: var(--vscode-textLink-foreground);
    border: 1px solid var(--vscode-textLink-foreground);
    padding: 5px 20px; font-family: inherit; font-size: inherit;
    cursor: pointer; border-radius: 2px;
  }
  .load-more-btn:hover { opacity: 0.8; }
  .load-more-btn:disabled { opacity: 0.5; cursor: default; }

  .empty {
    padding: 48px 20px; text-align: center;
    color: var(--vscode-descriptionForeground); line-height: 1.9;
  }
  svg { display: block; }
</style>
</head>
<body>

<div class="search-bar">
  <input id="search-input" type="text"
         placeholder="Search datasets, tables, columns…"
         autocomplete="off" spellcheck="false" />
  <button id="search-btn">Search</button>
</div>

<div id="status-bar" class="status-bar hidden"></div>

<div id="results" class="results">
  <div class="empty">
    Enter a search term and click <strong>Search</strong><br/>
    to find BigQuery resources via the Dataplex catalog.
  </div>
</div>

<script nonce="${nonce}">
  const vscode = acquireVsCodeApi();
  const input     = document.getElementById('search-input');
  const searchBtn = document.getElementById('search-btn');
  const statusBar = document.getElementById('status-bar');
  const results   = document.getElementById('results');

  searchBtn.addEventListener('click', () => doSearch());
  input.addEventListener('keydown', e => { if (e.key === 'Enter') doSearch(); });

  function doSearch() {
    const term = input.value.trim();
    if (!term) { input.focus(); return; }
    vscode.postMessage({ command: 'search', term });
  }

  window.addEventListener('message', e => {
    const msg = e.data;
    switch (msg.command) {
      case 'searching':   onSearching(msg.term); break;
      case 'results':     onResults(msg.term, msg.results, msg.nextPageToken); break;
      case 'loadingMore': onLoadingMore(); break;
      case 'moreResults': onMoreResults(msg.results, msg.nextPageToken); break;
      case 'error':       onError(msg.term, msg.message); break;
    }
  });

  function onSearching(term) {
    searchBtn.disabled = true;
    setStatus('<span class="spinner"></span> Searching for <em>' + esc(term) + '</em>…', '');
    results.innerHTML = '';
  }

  function onResults(term, items, nextPageToken) {
    searchBtn.disabled = false;
    if (!items || items.length === 0) {
      setStatus('No results for <em>' + esc(term) + '</em>.', '');
      results.innerHTML = '<div class="empty">No results found for <strong>' + esc(term) + '</strong>.</div>';
      return;
    }
    const moreHint = nextPageToken ? ' — scroll for more' : '';
    setStatus(items.length + ' result' + (items.length === 1 ? '' : 's') + ' for <em>' + esc(term) + '</em>' + moreHint, '');
    results.innerHTML = renderEntries(items);
    appendLoadMoreIfNeeded(nextPageToken);
    attachHandlers();
  }

  function onLoadingMore() {
    const btn = document.getElementById('load-more-btn');
    if (btn) { btn.disabled = true; btn.textContent = 'Loading…'; }
  }

  function onMoreResults(items, nextPageToken) {
    const existing = document.getElementById('load-more-row');
    if (existing) { existing.remove(); }
    if (items && items.length > 0) {
      const tmp = document.createElement('div');
      tmp.innerHTML = renderEntries(items);
      while (tmp.firstChild) { results.appendChild(tmp.firstChild); }
      results.querySelectorAll('.entry:not([data-bound])').forEach(attachEntryHandlers);
    }
    appendLoadMoreIfNeeded(nextPageToken);
  }

  function onError(term, message) {
    searchBtn.disabled = false;
    setStatus(message, 'error');
    results.innerHTML = '<div class="empty">Search failed. Check the status bar for details.</div>';
  }

  function setStatus(html, cls) {
    statusBar.className = 'status-bar ' + cls;
    statusBar.innerHTML = html;
  }

  function renderEntries(items) {
    return items.map(item => {
      const treeItemType = item.kind === 'view' ? 5 : item.kind === 'model' ? 8 : 3;
      const canOpen = !!item.tableId;
      let html = '<div class="entry"';
      if (canOpen) {
        html += ' data-project="' + esc(item.projectId) + '"';
        html += ' data-dataset="' + esc(item.datasetId) + '"';
        html += ' data-table="'   + esc(item.tableId)   + '"';
        html += ' data-kind="'    + esc(item.kind)      + '"';
        html += ' data-type="'    + treeItemType         + '"';
      }
      html += '>';
      html += '<span class="entry-icon">' + iconSvg(item.kind) + '</span>';
      html += '<div class="entry-body">';
      html += '<div class="entry-name">';
      html += '<span class="entry-name-text">' + esc(item.displayName || item.tableId || item.datasetId) + '</span>';
      if (canOpen) {
        html += '<div class="entry-actions">';
        html += '<button class="action-btn open-btn">' + (item.kind === 'model' ? 'DDL' : 'Open') + '</button>';
        html += '<button class="action-btn query-btn">Query</button>';
        html += '</div>';
      }
      html += '</div>';
      const path = item.projectId + ' · ' + item.datasetId;
      const modTime = item.modifyTime ? formatDate(item.modifyTime) : null;
      html += '<div class="entry-path">';
      html += '<span class="kind-badge">' + esc(item.kind) + '</span>';
      html += esc(path);
      if (modTime) { html += ' · modified ' + esc(modTime); }
      html += '</div>';
      if (item.description) {
        html += '<div class="entry-desc">' + esc(item.description) + '</div>';
      }
      html += '</div>';
      html += '</div>';
      return html;
    }).join('');
  }

  function appendLoadMoreIfNeeded(nextPageToken) {
    if (!nextPageToken) { return; }
    const row = document.createElement('div');
    row.id = 'load-more-row';
    row.className = 'load-more-row';
    row.innerHTML = '<button class="load-more-btn" id="load-more-btn">Load more results</button>';
    results.appendChild(row);
    document.getElementById('load-more-btn').addEventListener('click', () => {
      vscode.postMessage({ command: 'loadMore' });
    });
  }

  function attachHandlers() { results.querySelectorAll('.entry').forEach(attachEntryHandlers); }

  function attachEntryHandlers(entry) {
    if (entry.dataset.bound) { return; }
    entry.dataset.bound = '1';
    if (!entry.dataset.table) { return; }
    entry.addEventListener('click', e => {
      if (e.target.closest('.action-btn')) { return; }
      fireAction(entry, 'openTable');
    });
    const openBtn = entry.querySelector('.open-btn');
    if (openBtn) { openBtn.addEventListener('click', e => { e.stopPropagation(); fireAction(entry, 'openTable'); }); }
    const queryBtn = entry.querySelector('.query-btn');
    if (queryBtn) { queryBtn.addEventListener('click', e => { e.stopPropagation(); fireAction(entry, 'createQuery'); }); }
  }

  function fireAction(entry, command) {
    vscode.postMessage({
      command,
      projectId:    entry.dataset.project,
      datasetId:    entry.dataset.dataset,
      tableId:      entry.dataset.table,
      treeItemType: parseInt(entry.dataset.type, 10),
    });
  }

  function formatDate(iso) {
    try {
      return new Date(iso).toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
    } catch (_) { return iso; }
  }

  function esc(s) {
    if (s == null) { return ''; }
    return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
  }

  function iconSvg(kind) {
    const s = 'stroke="currentColor" stroke-width="1.3" fill="none"';
    const ns = 'xmlns="http://www.w3.org/2000/svg"';
    const open = '<svg width="16" height="16" viewBox="0 0 16 16" ' + ns + '>';
    switch (kind) {
      case 'dataset':
        // Folder icon
        return open
          + '<path d="M1.5 5.5h13v7.5a1 1 0 0 1-1 1h-11a1 1 0 0 1-1-1V5.5z" ' + s + '/>'
          + '<path d="M1.5 5.5V4a1 1 0 0 1 1-1h3.5l1.5 2h6.5a1 1 0 0 1 1 1" ' + s + '/></svg>';
      case 'view':
        // Eye icon
        return open
          + '<path d="M1.5 8s2.5-5 6.5-5 6.5 5 6.5 5-2.5 5-6.5 5-6.5-5-6.5-5z" ' + s + '/>'
          + '<circle cx="8" cy="8" r="2" ' + s + '/></svg>';
      case 'model':
        // Neural-net / connected nodes
        return open
          + '<circle cx="3" cy="8" r="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" fill-opacity="0.3"/>'
          + '<circle cx="8" cy="3" r="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" fill-opacity="0.3"/>'
          + '<circle cx="8" cy="13" r="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" fill-opacity="0.3"/>'
          + '<circle cx="13" cy="8" r="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" fill-opacity="0.3"/>'
          + '<line x1="4.4" y1="7.4" x2="6.6" y2="4" stroke="currentColor" stroke-width="1.1"/>'
          + '<line x1="4.4" y1="8.6" x2="6.6" y2="12" stroke="currentColor" stroke-width="1.1"/>'
          + '<line x1="9.4" y1="4" x2="11.6" y2="7.4" stroke="currentColor" stroke-width="1.1"/>'
          + '<line x1="9.4" y1="12" x2="11.6" y2="8.6" stroke="currentColor" stroke-width="1.1"/></svg>';
      default:
        // Table/grid icon
        return open
          + '<rect x="1.5" y="2.5" width="13" height="11" rx="1" ' + s + '/>'
          + '<line x1="1.5" y1="6" x2="14.5" y2="6" stroke="currentColor" stroke-width="1.3"/>'
          + '<line x1="6" y1="6" x2="6" y2="13.5" stroke="currentColor" stroke-width="1.3"/>'
          + '<line x1="10" y1="6" x2="10" y2="13.5" stroke="currentColor" stroke-width="1.3"/></svg>';
    }
  }

  input.focus();
</script>
</body>
</html>`;
    }
}


function getNonce(): string {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}
