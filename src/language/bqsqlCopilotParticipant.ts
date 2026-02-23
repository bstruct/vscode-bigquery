/**
 * BigQuery Copilot Chat Participant
 *
 * Registers a  @bigquery  chat participant that:
 *  – reads the currently active .bqsql document
 *  – extracts table references and enriches the prompt with live table schemas
 *  – forwards the enriched prompt to the GitHub Copilot language model
 *
 * Supported slash commands:
 *   /explain  – explain the current query in plain English
 *   /optimize – suggest optimisation opportunities
 *   /schema   – summarise the schemas of the referenced tables
 */

import * as vscode from 'vscode';
import { bigqueryTableSchemaService } from '../extension';
import { BqsqlTsParser } from './bqsqlTsParser';

const PARTICIPANT_ID = 'bigquery.sql';

// ---------------------------------------------------------------------------
// Registration (called from extension.ts activate)
// ---------------------------------------------------------------------------

export function registerBqsqlCopilotParticipant(context: vscode.ExtensionContext): void {
    if (!vscode.chat) {
        // Chat API not available in this VS Code build
        console.warn('[bigquery] vscode.chat API not available – skipping chat participant registration.');
        return;
    }

    const participant = vscode.chat.createChatParticipant(PARTICIPANT_ID, handleChatRequest);
    participant.iconPath = new vscode.ThemeIcon('database');
    context.subscriptions.push(participant);
    console.log('[bigquery] Chat participant @bigquery registered.');
}

// ---------------------------------------------------------------------------
// Request handler
// ---------------------------------------------------------------------------

async function handleChatRequest(
    request: vscode.ChatRequest,
    _context: vscode.ChatContext,
    stream: vscode.ChatResponseStream,
    token: vscode.CancellationToken,
): Promise<vscode.ChatResult> {

    // ── Gather current editor context ────────────────────────────────────
    const editor = vscode.window.activeTextEditor;
    let currentSql = '';
    let schemaMarkdown = '';

    if (editor && editor.document.languageId === 'bqsql') {
        currentSql = editor.document.getText();

        // Pre-load any uncached schemas then build the markdown block
        if (currentSql.trim()) {
            const refs = BqsqlTsParser.extractTableRefs(currentSql);
            for (const ref of refs) {
                await bigqueryTableSchemaService.preLoadSchemaByFullName(ref.fullName).catch(() => undefined);
                const schema = bigqueryTableSchemaService.getSchemaByFullName(ref.fullName);
                if (schema.length > 0) {
                    const aliasNote = ref.alias ? ` (alias: \`${ref.alias}\`)` : '';
                    schemaMarkdown += `\n### Table \`${ref.fullName}\`${aliasNote}\n`;
                    schemaMarkdown += '| Column | Type | Notes |\n|--------|------|-------|\n';
                    for (const col of schema) {
                        const notes = [
                            col.is_partitioning_column === 'YES' ? '🔑 partition' : '',
                            col.description ?? '',
                        ].filter(Boolean).join(' · ');
                        schemaMarkdown += `| \`${col.column_name}\` | \`${col.data_type}\` | ${notes} |\n`;
                    }
                }
            }
        }
    }

    // ── Choose slash-command behaviour ───────────────────────────────────
    let userInstruction = request.prompt;

    if (request.command === 'explain') {
        userInstruction = `Explain what the following BigQuery SQL query does in plain English, step by step:\n\n\`\`\`sql\n${currentSql}\n\`\`\``;
    } else if (request.command === 'optimize') {
        userInstruction = `Suggest concrete optimisations for this BigQuery SQL query. ` +
            `Focus on cost reduction (bytes processed), partition pruning, and avoiding full scans. ` +
            `Provide the improved query where applicable.\n\n\`\`\`sql\n${currentSql}\n\`\`\``;
    } else if (request.command === 'schema') {
        if (!schemaMarkdown) {
            stream.markdown('No table schemas found for the current query (schemas are loaded when the file is opened and the user is authenticated).');
            return {};
        }
        stream.markdown('## Referenced Table Schemas\n' + schemaMarkdown);
        return {};
    }

    // ── Build system prompt ───────────────────────────────────────────────
    let systemPrompt =
        'You are an expert BigQuery Standard SQL assistant. ' +
        'Always use Standard SQL syntax, never legacy SQL. ' +
        'Prefer cost-efficient patterns: partition pruning, column selection over SELECT *, use of clustering. ' +
        'Be concise and accurate.';

    if (schemaMarkdown) {
        systemPrompt +=
            '\n\nThe user\'s current query references the following BigQuery table schemas. ' +
            'Use this information to give column-accurate suggestions and explanations:' +
            schemaMarkdown;
    }

    if (currentSql && !request.command) {
        systemPrompt +=
            '\n\nThe user\'s current SQL query (for context):\n```sql\n' + currentSql + '\n```';
    }

    // ── Select model and stream response ─────────────────────────────────
    let models: vscode.LanguageModelChat[] = [];
    try {
        models = await vscode.lm.selectChatModels({ vendor: 'copilot', family: 'gpt-4o' });
    } catch {
        // selectChatModels may throw when Copilot is not available
    }

    if (models.length === 0) {
        // Try any available model
        try {
            models = await vscode.lm.selectChatModels();
        } catch { /* ignore */ }
    }

    if (models.length === 0) {
        stream.markdown(
            '⚠️ No AI model available. Please ensure **GitHub Copilot** is installed and you are signed in.',
        );
        return { metadata: { command: request.command } };
    }

    const model = models[0];
    const messages: vscode.LanguageModelChatMessage[] = [
        vscode.LanguageModelChatMessage.User(systemPrompt + '\n\n---\n\n' + userInstruction),
    ];

    try {
        const response = await model.sendRequest(messages, {}, token);
        for await (const chunk of response.text) {
            if (token.isCancellationRequested) { break; }
            stream.markdown(chunk);
        }
    } catch (err: unknown) {
        if (err instanceof vscode.LanguageModelError) {
            stream.markdown(`\n\n⚠️ Model error: ${err.message} (${err.code})`);
        } else {
            stream.markdown(`\n\n⚠️ Unexpected error: ${String(err)}`);
        }
    }

    return { metadata: { command: request.command } };
}
