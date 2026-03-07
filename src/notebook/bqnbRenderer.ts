// This file is built as a separate entry point by webpack, targeting 'web'.
// It runs inside the VS Code notebook renderer iframe (ES module context).
// We use import.meta.url to locate sibling assets (WASM, CSS, grid_render.js).

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

            console.log('[bqnb-renderer] Initialization complete');
        })();
    }
    return initializedPromise;
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

            // The JSON payload from our controller
            const payload = outputItem.json();

            element.innerHTML = `
                <div style="min-height: 100px; max-height: 600px; overflow: auto; padding: 4px;">
                    <div style="color: var(--vscode-descriptionForeground, #999); font-size: 12px;">Loading BigQuery results…</div>
                </div>
            `;

            try {
                await ensureInitialized();

                const container = element.querySelector('div')!;
                container.innerHTML = ''; // clear placeholder

                // Create the bq-query custom element exactly like the panel webview does
                const bqQuery = document.createElement('bq-query');
                bqQuery.setAttribute('be_id', `grid-${outputItem.id}`);
                bqQuery.setAttribute('job_id', payload.jobId || '');
                bqQuery.setAttribute('project_id', payload.projectId || '');
                bqQuery.setAttribute('location', payload.location || 'US');
                bqQuery.setAttribute('token', payload.token || '');
                bqQuery.setAttribute('statement_type', payload.statementType || 'SELECT');
                bqQuery.setAttribute('page_size', '50');
                bqQuery.setAttribute('page_start_index', '0');

                container.appendChild(bqQuery);

                // Give the custom element time to connect, then trigger data fetch
                setTimeout(() => {
                    const event = new Event('render_table');
                    bqQuery.dispatchEvent(event);
                }, 100);

            } catch (err: any) {
                console.error('[bqnb-renderer] renderOutputItem error:', err);
                element.innerHTML = `<div style="color: var(--vscode-errorForeground, red); padding: 8px;">Failed to render grid: ${err.message || err}</div>`;
            }
        }
    };
}
