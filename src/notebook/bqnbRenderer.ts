// This file is built as a separate entry point by webpack, targeting 'web'.
// It runs inside the VS Code notebook renderer iframe (ES module context).
// We use import.meta.url to locate sibling assets (WASM, CSS, grid_render.js).

import { Chart, registerables } from 'chart.js';
Chart.register(...registerables);

let initializedPromise: Promise<void> | null = null;
let gridRenderModule: any = null;

function resolveAssetUrl(filename: string): string {
    // import.meta.url points to the bqnbRenderer.js file inside dist/
    const base = (import.meta as any).url as string;
    return new URL(filename, base).href;
}

async function ensureInitialized(): Promise<void> {
    if (!initializedPromise) {
        initializedPromise = (async () => {
            const wasmUrl = resolveAssetUrl('grid_render_bg.wasm');
            const gridRenderUrl = resolveAssetUrl('grid_render.js');

            console.log('[bqnb-renderer] Loading grid_render from:', gridRenderUrl);
            console.log('[bqnb-renderer] Loading WASM from:', wasmUrl);

            // Dynamic import that Webpack won't try to bundle
            const dynamicImport = new Function('url', 'return import(url)');
            gridRenderModule = await dynamicImport(gridRenderUrl);

            // Initialize the WASM module
            await gridRenderModule.default(wasmUrl);

            // Register all custom web components from grid_render
            const webComponentsList = gridRenderModule.get_web_components_list();
            for (let index = 0; index < webComponentsList.length; index++) {
                const elementName = webComponentsList[index];
                if (!customElements.get(elementName)) {
                    const gridRender = gridRenderModule; // capture in closure
                    window.customElements.define(elementName, class extends HTMLElement {
                        connectedCallback() {
                            gridRender.register_custom_element(elementName, this);
                        }
                    });
                }
            }

            // Wire up the external_message listener (same pattern as grid.js)
            window.addEventListener('external_message', gridRenderModule.on_window_message_received);

            // Provide dummy setState if not present – Rust code calls document.setState(...)
            if (!(document as any).setState) {
                (document as any).setState = function (str: string) {
                    console.log('[bqnb-renderer] setState:', str);
                };
            }

            // Load grid.css
            const cssUrl = resolveAssetUrl('../resources/grid.css');
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = cssUrl;
            document.head.appendChild(link);

            // Inject tab styles once
            if (!document.getElementById('bqnb-tab-styles')) {
                const style = document.createElement('style');
                style.id = 'bqnb-tab-styles';
                style.textContent = `
                    .bqnb-tabs { display: flex; flex-direction: column; min-height: 100px; }
                    .bqnb-tab-bar {
                        display: flex;
                        background: var(--vscode-editorGroupHeader-tabsBackground, #1e1e1e);
                        border-bottom: 1px solid var(--vscode-panel-border, #444);
                    }
                    .bqnb-tab-btn {
                        padding: 5px 14px;
                        background: none;
                        border: none;
                        border-bottom: 2px solid transparent;
                        color: var(--vscode-foreground, #ccc);
                        cursor: pointer;
                        font-size: 12px;
                        font-family: var(--vscode-font-family, sans-serif);
                    }
                    .bqnb-tab-btn:hover { background: var(--vscode-toolbar-hoverBackground, #2a2d2e); }
                    .bqnb-tab-btn.active {
                        color: var(--vscode-tab-activeForeground, #fff);
                        border-bottom-color: var(--vscode-focusBorder, #007acc);
                    }
                    .bqnb-tab-pane { display: none; }
                    .bqnb-tab-pane.active { display: block; overflow-y: auto; }
                    .bqnb-viz-controls { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; padding: 8px 0; }
                    .bqnb-viz-controls label { font-size: 12px; color: var(--vscode-foreground, #ccc); }
                    .bqnb-viz-controls select {
                        background: var(--vscode-dropdown-background, #3c3c3c);
                        color: var(--vscode-dropdown-foreground, #ccc);
                        border: 1px solid var(--vscode-dropdown-border, #555);
                        padding: 3px 6px;
                        font-size: 12px;
                        border-radius: 2px;
                    }
                    .bqnb-viz-controls button {
                        background: var(--vscode-button-background, #0e639c);
                        color: var(--vscode-button-foreground, #fff);
                        border: none;
                        padding: 4px 12px;
                        font-size: 12px;
                        border-radius: 2px;
                        cursor: pointer;
                    }
                    .bqnb-viz-controls button:hover { background: var(--vscode-button-hoverBackground, #1177bb); }
                    .bqnb-chart-wrap { position: relative; height: 400px; }
                    .bqnb-viz-msg { font-size: 12px; color: var(--vscode-descriptionForeground, #888); padding: 4px 0; }
                    .bqnb-jobinfo-table { width: 100%; border-collapse: collapse; font-size: 12px; margin-top: 8px; }
                    .bqnb-jobinfo-table th, .bqnb-jobinfo-table td {
                        text-align: left; padding: 3px 8px;
                        border-bottom: 1px solid var(--vscode-panel-border, #333);
                        vertical-align: top; word-break: break-all;
                    }
                    .bqnb-jobinfo-table th { color: var(--vscode-descriptionForeground, #888); width: 200px; white-space: nowrap; }
                    .bqnb-jobinfo-table td { color: var(--vscode-foreground, #ccc); }
                `;
                document.head.appendChild(style);
            }

            console.log('[bqnb-renderer] Initialization complete');
        })();
    }
    return initializedPromise;
}

function esc(s: string): string {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

function flattenObj(obj: any, prefix: string): [string, string][] {
    const rows: [string, string][] = [];
    for (const key of Object.keys(obj || {})) {
        const val = obj[key];
        const label = prefix ? prefix + '.' + key : key;
        if (val !== null && typeof val === 'object' && !Array.isArray(val)) {
            rows.push(...flattenObj(val, label));
        } else {
            rows.push([label, Array.isArray(val) ? JSON.stringify(val) : String(val ?? '')]);
        }
    }
    return rows;
}

interface RendererContext {
    readonly workspace: unknown;
    postMessage?(message: unknown): void;
}

interface OutputItem {
    readonly id: string;
    json(): any;
    text(): string;
}

export function activate(_ctx: RendererContext) {
    // The Rust WASM generates click handlers that call `vscode.postMessage(...)`.
    // In the notebook renderer iframe, acquireVsCodeApi() is unavailable, so we
    // provide a shim that forwards messages via the renderer context.
    if (!(window as any).vscode) {
        (window as any).vscode = {
            postMessage: (msg: unknown) => { _ctx.postMessage?.(msg); },
            getState: () => undefined,
            setState: (_s: unknown) => {},
        };
    }

    return {
        async renderOutputItem(outputItem: OutputItem, element: HTMLElement) {

            const payload = outputItem.json();

            element.innerHTML = `
                <div style="min-height: 100px; padding: 4px;">
                    <div style="color: var(--vscode-descriptionForeground, #999); font-size: 12px;">Loading BigQuery results…</div>
                </div>
            `;

            try {
                await ensureInitialized();

                const id = `bqnb-${outputItem.id.replace(/[^a-z0-9]/gi, '')}`;

                // Build tab structure
                const root = document.createElement('div');
                root.className = 'bqnb-tabs';

                root.innerHTML = `
                    <div class="bqnb-tab-bar">
                        <button class="bqnb-tab-btn active" data-tab="${id}-results">Results</button>
                        <button class="bqnb-tab-btn" data-tab="${id}-viz">Visualization</button>
                        <button class="bqnb-tab-btn" data-tab="${id}-jobinfo">Job Information</button>
                    </div>
                    <div id="${id}-results" class="bqnb-tab-pane active">
                        <div id="${id}-grid" style="height:500px;overflow:auto;"></div>
                    </div>
                    <div id="${id}-viz" class="bqnb-tab-pane" style="max-height:500px;overflow-y:auto;padding:12px;">
                        <div class="bqnb-viz-controls">
                            <label>Chart type:
                                <select id="${id}-vtype">
                                    <option value="bar">Bar</option>
                                    <option value="line">Line</option>
                                    <option value="scatter">Scatter</option>
                                    <option value="pie">Pie</option>
                                    <option value="doughnut">Doughnut</option>
                                </select>
                            </label>
                            <label>X axis:
                                <select id="${id}-vx"><option value="">— select —</option></select>
                            </label>
                            <label>Y axis:
                                <select id="${id}-vy"><option value="">— select —</option></select>
                            </label>
                            <button id="${id}-vbtn">Render</button>
                        </div>
                        <div id="${id}-vmsg" class="bqnb-viz-msg">Select columns and click Render.</div>
                        <div class="bqnb-chart-wrap" id="${id}-cwrap" style="display:none;">
                            <canvas id="${id}-canvas"></canvas>
                        </div>
                    </div>
                    <div id="${id}-jobinfo" class="bqnb-tab-pane" style="max-height:500px;overflow-y:auto;padding:12px;">
                        <table class="bqnb-jobinfo-table"><tbody id="${id}-jbody"></tbody></table>
                    </div>
                `;

                element.innerHTML = '';
                element.appendChild(root);

                // Render the WASM grid in the Results tab
                const gridContainer = document.getElementById(`${id}-grid`)!;
                const bqQuery = document.createElement('bq-query');
                bqQuery.setAttribute('be_id', `grid-${outputItem.id}`);
                bqQuery.setAttribute('job_id', payload.jobId || '');
                bqQuery.setAttribute('project_id', payload.projectId || '');
                bqQuery.setAttribute('location', payload.location || 'US');
                bqQuery.setAttribute('token', payload.token || '');
                bqQuery.setAttribute('statement_type', payload.statementType || 'SELECT');
                bqQuery.setAttribute('page_size', '50');
                bqQuery.setAttribute('page_start_index', '0');
                gridContainer.appendChild(bqQuery);
                setTimeout(() => { bqQuery.dispatchEvent(new Event('render_table')); }, 100);

                // Populate job info tab
                const jbody = document.getElementById(`${id}-jbody`)!;
                const meta = payload.jobMetadata || {};
                const metaRows = flattenObj(meta, '');
                metaRows.forEach(([k, v]) => {
                    const tr = document.createElement('tr');
                    tr.innerHTML = `<th>${esc(k)}</th><td>${esc(v)}</td>`;
                    jbody.appendChild(tr);
                });

                // Viz state
                const xSel = document.getElementById(`${id}-vx`) as HTMLSelectElement;
                const ySel = document.getElementById(`${id}-vy`) as HTMLSelectElement;
                const vmsg = document.getElementById(`${id}-vmsg`)!;
                let chartInstance: Chart | null = null;
                let cachedData: any[] | null = null;
                let loadingData = false;

                async function loadVizData() {
                    if (loadingData || cachedData) { return; }
                    loadingData = true;
                    vmsg.textContent = 'Loading data…';
                    try {
                        const result = await fetchBqData(payload);
                        cachedData = result.rows;
                        xSel.innerHTML = '<option value="">— select —</option>';
                        ySel.innerHTML = '<option value="">— select —</option>';
                        result.fields.forEach((name: string) => {
                            const o1 = document.createElement('option');
                            o1.value = name; o1.textContent = name;
                            xSel.appendChild(o1);
                            const o2 = document.createElement('option');
                            o2.value = name; o2.textContent = name;
                            ySel.appendChild(o2);
                        });
                        vmsg.textContent = result.fields.length > 0 ? 'Select columns and click Render.' : 'No columns available.';
                    } catch (err: any) {
                        vmsg.textContent = 'Error loading data: ' + (err.message || err);
                    } finally {
                        loadingData = false;
                    }
                }

                // Tab switching — auto-load viz data on tab open
                root.querySelectorAll('.bqnb-tab-btn').forEach(btn => {
                    btn.addEventListener('click', () => {
                        const tab = (btn as HTMLElement).dataset.tab!;
                        root.querySelectorAll('.bqnb-tab-btn').forEach(b => b.classList.remove('active'));
                        root.querySelectorAll('.bqnb-tab-pane').forEach(p => p.classList.remove('active'));
                        btn.classList.add('active');
                        document.getElementById(tab)!.classList.add('active');
                        if (tab === `${id}-viz` && !cachedData) { loadVizData(); }
                    });
                });

                // Chart render button
                document.getElementById(`${id}-vbtn`)!.addEventListener('click', async () => {
                    if (!cachedData) {
                        await loadVizData();
                        if (!cachedData) { return; }
                    }
                    const xCol = xSel.value;
                    const yCol = ySel.value;
                    if (!xCol || !yCol) {
                        vmsg.textContent = 'Please select X and Y columns.';
                        return;
                    }
                    const chartType = (document.getElementById(`${id}-vtype`) as HTMLSelectElement).value;
                    const labels = cachedData.map((r: any) => r[xCol]);
                    const values = cachedData.map((r: any) => parseFloat(r[yCol]));

                    const cwrap = document.getElementById(`${id}-cwrap`)!;
                    cwrap.style.display = '';

                    if (chartInstance) { chartInstance.destroy(); }
                    const canvas = document.getElementById(`${id}-canvas`) as HTMLCanvasElement;
                    chartInstance = new Chart(canvas, {
                        type: chartType as any,
                        data: {
                            labels,
                            datasets: [{
                                label: yCol,
                                data: chartType === 'scatter'
                                    ? cachedData.map((r: any) => ({ x: parseFloat(r[xCol]), y: parseFloat(r[yCol]) }))
                                    : values,
                                backgroundColor: 'rgba(0, 122, 204, 0.6)',
                                borderColor: 'rgba(0, 122, 204, 1)',
                                borderWidth: 1
                            }]
                        },
                        options: {
                            responsive: true,
                            maintainAspectRatio: false,
                            plugins: { legend: { display: true } }
                        }
                    });
                    vmsg.textContent = '';
                });

            } catch (err: any) {
                console.error('[bqnb-renderer] renderOutputItem error:', err);
                element.innerHTML = `<div style="color: var(--vscode-errorForeground, red); padding: 8px;">Failed to render: ${err.message || err}</div>`;
            }
        }
    };
}

async function fetchBqData(payload: any): Promise<{ rows: any[]; fields: string[] }> {
    const { jobId, projectId, location, token } = payload;
    if (!jobId || !projectId || !token) { throw new Error('Missing job/project/token in payload'); }
    let url = `https://bigquery.googleapis.com/bigquery/v2/projects/${encodeURIComponent(projectId)}/queries/${encodeURIComponent(jobId)}?maxResults=10000`;
    if (location) { url += `&location=${encodeURIComponent(location)}`; }
    const resp = await fetch(url, { headers: { 'Authorization': `Bearer ${token}` } });
    if (!resp.ok) { throw new Error(`HTTP ${resp.status}`); }
    const body = await resp.json();
    const fieldDefs: any[] = (body.schema && body.schema.fields) || [];
    const fields = fieldDefs.map((f: any) => f.name);
    const rawRows: any[] = body.rows || [];
    const rows = rawRows.map(row => {
        const obj: any = {};
        (row.f || []).forEach((cell: any, i: number) => {
            obj[fields[i] || i] = cell.v;
        });
        return obj;
    });
    return { rows, fields };
}
