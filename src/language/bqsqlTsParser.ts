/**
 * Pure-TypeScript BigQuery SQL parser.
 *
 * Replaces the Rust/WASM @bstruct/bqsql-parser for the purposes needed by the extension:
 *  - Keyword / token positions (semantic tokens, highlighting)
 *  - Table-reference extraction (column completions, schema pre-loading)
 *  - Column-completion context: dot-trigger (alias.) AND SELECT-clause context
 *  - CTE column extraction (no API call needed)
 *
 * The parser is intentionally permissive – it is used for IDE support, not execution.
 */

// ---------------------------------------------------------------------------
// Token types
// ---------------------------------------------------------------------------

export type TokenType =
    | 'keyword'
    | 'identifier'
    | 'backtick'    // `project.dataset.table`
    | 'string'      // '…' or "…"
    | 'number'
    | 'operator'
    | 'comment';

export interface Token {
    type: TokenType;
    value: string;
    /** 0-based line index */
    line: number;
    /** 0-based start column (inclusive) */
    startChar: number;
    /** 0-based end column (exclusive) */
    endChar: number;
}

// ---------------------------------------------------------------------------
// BigQuery keywords (used for semantic token highlighting)
// ---------------------------------------------------------------------------

const BQ_KEYWORDS = new Set<string>([
    // DML
    'SELECT', 'FROM', 'WHERE', 'INSERT', 'INTO', 'UPDATE', 'DELETE', 'MERGE',
    'USING', 'MATCHED', 'THEN',
    // DDL
    'CREATE', 'TABLE', 'VIEW', 'REPLACE', 'MATERIALIZED', 'DROP', 'ALTER',
    // Query clauses
    'WITH', 'AS', 'DISTINCT', 'ALL', 'UNION', 'INTERSECT', 'EXCEPT',
    'JOIN', 'INNER', 'LEFT', 'RIGHT', 'OUTER', 'FULL', 'CROSS', 'LATERAL',
    'ON', 'USING',
    'GROUP', 'ORDER', 'BY', 'HAVING', 'LIMIT', 'OFFSET', 'QUALIFY',
    // Window
    'OVER', 'PARTITION', 'ROWS', 'RANGE', 'BETWEEN', 'PRECEDING', 'FOLLOWING',
    'CURRENT', 'ROW', 'UNBOUNDED',
    // Conditionals / expressions
    'CASE', 'WHEN', 'ELSE', 'END', 'IF', 'AND', 'OR', 'NOT', 'IN',
    'EXISTS', 'IS', 'NULL', 'LIKE', 'BETWEEN', 'ANY', 'SOME', 'ALL',
    'TRUE', 'FALSE',
    // Types / casts
    'CAST', 'SAFE_CAST', 'ARRAY', 'STRUCT', 'INTERVAL',
    // Functions used as keywords
    'EXTRACT', 'UNNEST', 'COALESCE', 'NULLIF', 'IFNULL',
    // Analytics extensions
    'PIVOT', 'UNPIVOT', 'TABLESAMPLE', 'SYSTEM',
    // Misc
    'SET', 'AT', 'TIME', 'ZONE', 'SAFE', 'IGNORE', 'RESPECT',
    'NULLS', 'FIRST', 'LAST',
    'VALUES', 'DEFAULT',
    'FETCH', 'NEXT', 'ONLY', 'ROWS',
    'WINDOW',
]);

// Keywords that cannot be table aliases
const ALIAS_STOP_KEYWORDS = new Set<string>([
    'WHERE', 'ON', 'SET', 'INNER', 'LEFT', 'RIGHT', 'OUTER',
    'FULL', 'CROSS', 'GROUP', 'ORDER', 'HAVING', 'LIMIT', 'UNION',
    'INTERSECT', 'EXCEPT', 'JOIN', 'SELECT', 'WITH', 'INSERT',
    'UPDATE', 'DELETE', 'MERGE', 'USING', 'LATERAL', 'QUALIFY',
    'WINDOW',
]);

// Clause keywords that end a SELECT's column list
const SELECT_CLAUSE_ENDERS = new Set<string>([
    'FROM', 'WHERE', 'GROUP', 'ORDER', 'HAVING', 'LIMIT', 'QUALIFY',
    'WINDOW', 'SET', 'INTO',
]);

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

export function tokenize(sql: string): Token[] {
    const tokens: Token[] = [];
    let i = 0;
    let line = 0;
    let lineStart = 0;

    while (i < sql.length) {
        const ch = sql[i];

        // ── newline ──
        if (ch === '\n') {
            line++;
            lineStart = i + 1;
            i++;
            continue;
        }

        // ── whitespace ──
        if (ch === ' ' || ch === '\r' || ch === '\t') {
            i++;
            continue;
        }

        const colStart = i - lineStart;

        // ── line comment  -- ──
        if (ch === '-' && sql[i + 1] === '-') {
            let value = '';
            while (i < sql.length && sql[i] !== '\n') {
                value += sql[i++];
            }
            tokens.push({ type: 'comment', value, line, startChar: colStart, endChar: colStart + value.length });
            continue;
        }

        // ── block comment  /* ... */ ──
        if (ch === '/' && sql[i + 1] === '*') {
            const startLine = line;
            const startCol = colStart;
            let value = '/*';
            i += 2;
            while (i < sql.length && !(sql[i] === '*' && sql[i + 1] === '/')) {
                if (sql[i] === '\n') { line++; lineStart = i + 1; }
                value += sql[i++];
            }
            value += '*/';
            if (i < sql.length) { i += 2; }
            tokens.push({ type: 'comment', value, line: startLine, startChar: startCol, endChar: startCol + value.length });
            continue;
        }

        // ── backtick identifier  `…` ──
        if (ch === '`') {
            let value = '`';
            i++;
            while (i < sql.length && sql[i] !== '`') {
                value += sql[i++];
            }
            value += '`';
            i++;
            tokens.push({ type: 'backtick', value, line, startChar: colStart, endChar: colStart + value.length });
            continue;
        }

        // ── single or double-quoted string ──
        if (ch === "'" || ch === '"') {
            const quote = ch;
            let value = quote;
            i++;
            while (i < sql.length && sql[i] !== quote) {
                if (sql[i] === '\\') { value += sql[i++]; }   // escape
                if (i < sql.length) { value += sql[i++]; }
            }
            value += quote;
            i++;
            tokens.push({ type: 'string', value, line, startChar: colStart, endChar: colStart + value.length });
            continue;
        }

        // ── number ──
        if (/[0-9]/.test(ch) || (ch === '.' && sql[i + 1] && /[0-9]/.test(sql[i + 1]))) {
            let value = '';
            while (i < sql.length && /[0-9.eE+\-]/.test(sql[i])) {
                value += sql[i++];
            }
            tokens.push({ type: 'number', value, line, startChar: colStart, endChar: colStart + value.length });
            continue;
        }

        // ── identifier or keyword ──
        if (/[a-zA-Z_$]/.test(ch)) {
            let value = '';
            while (i < sql.length && /[a-zA-Z0-9_$]/.test(sql[i])) {
                value += sql[i++];
            }
            const type: TokenType = BQ_KEYWORDS.has(value.toUpperCase()) ? 'keyword' : 'identifier';
            tokens.push({ type, value, line, startChar: colStart, endChar: colStart + value.length });
            continue;
        }

        // ── operator / punctuation ──
        tokens.push({ type: 'operator', value: ch, line, startChar: colStart, endChar: colStart + 1 });
        i++;
    }

    return tokens;
}

// ---------------------------------------------------------------------------
// Table reference extraction (used by semantic tokens + Copilot participant)
// ---------------------------------------------------------------------------

export interface TableRef {
    /** Full table name without backticks, e.g. 'project.dataset.table' */
    fullName: string;
    alias: string | null;
    startLine: number;
    startChar: number;
    endChar: number;
}

/**
 * Walk the token stream and return all real table references (FROM/JOIN clauses).
 * CTE self-references are excluded.
 */
export function extractTableRefs(tokens: Token[]): TableRef[] {
    const refs: TableRef[] = [];
    const cteMap = buildCteMap(tokens);

    for (let i = 0; i < tokens.length; i++) {
        const tok = tokens[i];
        if (tok.type !== 'keyword') { continue; }
        const upper = tok.value.toUpperCase();
        if (upper !== 'FROM' && !upper.endsWith('JOIN')) { continue; }

        let j = i + 1;
        if (j >= tokens.length) { continue; }

        const result = readTableRefAt(tokens, j, cteMap);
        if (!result || result.source.kind !== 'table') { continue; }

        refs.push({
            fullName: result.source.fullName,
            alias: result.source.alias,
            startLine: tokens[j].line,
            startChar: tokens[j].startChar,
            endChar: tokens[j].endChar,
        });
    }

    return refs;
}

// ---------------------------------------------------------------------------
// CTE extraction
// ---------------------------------------------------------------------------

export interface CteDefinition {
    /** Lower-cased CTE name */
    name: string;
    /** Column names exposed by this CTE (aliases from its SELECT list) */
    columns: string[];
}

/**
 * Builds a map of  cte_name_lower → CteDefinition  for every WITH block in the token stream.
 * Used both for table-ref filtering and SELECT-context completions.
 */
function buildCteMap(tokens: Token[]): Map<string, CteDefinition> {
    const result = new Map<string, CteDefinition>();

    for (let i = 0; i < tokens.length; i++) {
        if (!(tokens[i].type === 'keyword' && tokens[i].value.toUpperCase() === 'WITH')) { continue; }

        let j = i + 1;
        // Parse:  name AS (body) [, name AS (body) ]*
        while (j < tokens.length) {
            // Expect: identifier
            if (tokens[j]?.type !== 'identifier') { break; }
            const cteName = tokens[j].value.toLowerCase();
            j++;

            // Expect: AS
            if (!(tokens[j]?.type === 'keyword' && tokens[j].value.toUpperCase() === 'AS')) { break; }
            j++;

            // Expect: '('
            if (!(tokens[j]?.type === 'operator' && tokens[j].value === '(')) { break; }
            j++;

            // Find matching ')' and remember body range
            let depth = 1;
            const bodyStart = j;
            while (j < tokens.length && depth > 0) {
                if (tokens[j].value === '(') { depth++; }
                if (tokens[j].value === ')') { depth--; }
                if (depth > 0) { j++; } else { break; }
            }
            const bodyEnd = j - 1;  // last token inside the body (before closing ')') 
            j++;                     // move past ')'

            const columns = extractOuterSelectColumns(tokens, bodyStart, bodyEnd);
            result.set(cteName, { name: cteName, columns });

            // Continue to next CTE if comma follows
            if (tokens[j]?.type === 'operator' && tokens[j].value === ',') {
                j++;
            } else {
                break;
            }
        }
    }

    return result;
}

/**
 * Given a token sub-range (inclusive), find the outermost SELECT and return
 * its column names (the effective output column names — aliases or last identifiers).
 */
function extractOuterSelectColumns(tokens: Token[], startIdx: number, endIdx: number): string[] {
    // Find first SELECT at depth 0 within the range
    let selectIdx = -1;
    let depth = 0;
    for (let i = startIdx; i <= endIdx; i++) {
        const v = tokens[i].value;
        if (v === '(') { depth++; }
        if (v === ')') { depth--; }
        if (depth === 0 && tokens[i].type === 'keyword' && tokens[i].value.toUpperCase() === 'SELECT') {
            selectIdx = i;
            break;
        }
    }
    if (selectIdx < 0) { return []; }

    // Collect token groups separated by top-level commas, stopping at FROM
    depth = 0;
    let currentItem: Token[] = [];
    const items: Token[][] = [];

    for (let i = selectIdx + 1; i <= endIdx; i++) {
        const t = tokens[i];
        if (t.value === '(') { depth++; }
        if (t.value === ')') { depth--; }

        if (depth === 0) {
            if (t.type === 'keyword') {
                const u = t.value.toUpperCase();
                if (SELECT_CLAUSE_ENDERS.has(u) || u === 'FROM') {
                    items.push(currentItem);
                    currentItem = [];
                    break;
                }
                if (u === 'DISTINCT' || u === 'ALL') { continue; }
            }
            if (t.type === 'operator' && t.value === ',') {
                items.push(currentItem);
                currentItem = [];
                continue;
            }
        }
        currentItem.push(t);
    }
    if (currentItem.length > 0) { items.push(currentItem); }

    return items.map(inferColumnName).filter((c): c is string => c !== null);
}

/**
 * Given the tokens of a single SELECT-list item (e.g. `SUM(amount) AS total`),
 * infer the output column name: alias if AS is present, otherwise last plain identifier.
 */
function inferColumnName(item: Token[]): string | null {
    // Collect depth-0 tokens only
    const flat: Token[] = [];
    let d = 0;
    for (const t of item) {
        if (t.value === '(') { d++; }
        if (t.value === ')') { d--; }
        if (d === 0) { flat.push(t); }
    }

    // AS alias  →  use the alias
    for (let i = 0; i < flat.length - 1; i++) {
        if (flat[i].type === 'keyword' && flat[i].value.toUpperCase() === 'AS') {
            const next = flat[i + 1];
            if (next?.type === 'identifier') { return next.value; }
        }
    }

    // No AS: find the last identifier that's not immediately followed by '('
    for (let i = flat.length - 1; i >= 0; i--) {
        const t = flat[i];
        if (t.type === 'identifier') {
            // Skip if it's a function name (followed by '(')
            if (i + 1 < flat.length && flat[i + 1].value === '(') { continue; }
            return t.value;
        }
        if (t.type === 'backtick') {
            // `schema.column` → return 'column'
            const parts = t.value.slice(1, -1).split('.');
            return parts[parts.length - 1];
        }
    }

    return null;
}

// ---------------------------------------------------------------------------
// SELECT context: what columns are available at the cursor position?
// ---------------------------------------------------------------------------

export type SelectSourceKind = 'table' | 'cte';

export interface SelectSource {
    kind: SelectSourceKind;
    /** For 'table': 'project.dataset.table'. For 'cte': the cte name. */
    fullName: string;
    alias: string | null;
    /** Populated only for kind === 'cte' */
    cteColumns: string[];
}

export interface SelectContext {
    /** All FROM/JOIN sources visible at the cursor (in insertion order) */
    sources: SelectSource[];
}

/**
 * Determines whether the cursor is inside the column-list of a SELECT statement
 * and returns all data sources (real tables + CTEs) that are referenced by
 * that SELECT's FROM / JOIN clauses.
 *
 * Returns null when the cursor is not in a SELECT column list.
 */
export function getSelectContext(sql: string, cursorLine: number, cursorCol: number): SelectContext | null {
    const tokens = tokenize(sql);
    const cteMap = buildCteMap(tokens);

    // ── 1. Find the last token index that is strictly before the cursor ──
    let cursorIdx = -1;
    for (let i = 0; i < tokens.length; i++) {
        const t = tokens[i];
        if (t.line < cursorLine || (t.line === cursorLine && t.startChar < cursorCol)) {
            cursorIdx = i;
        } else {
            break;
        }
    }
    // cursor is before all tokens – nothing to do
    if (cursorIdx < 0) { return null; }

    // (diagnostics logged by caller via outputChannel)

    // ── 2. Walk backwards to find the enclosing SELECT ──────────────────
    //
    // While walking backwards:
    //   ')' → depth++ (entering a subquery from the right)
    //   '(' → depth-- (leaving the current scope)
    //   If depth < 0: cursor is inside parens but we've left without finding SELECT → null
    //   At depth 0: if SELECT → found it; if clause-ender keyword → cursor is not in col list
    //
    let depth = 0;
    let selectIdx = -1;

    for (let i = cursorIdx; i >= 0; i--) {
        const t = tokens[i];
        if (t.type === 'operator') {
            if (t.value === ')') { depth++; }
            if (t.value === '(') {
                depth--;
                if (depth < 0) { return null; }
            }
        }
        if (depth !== 0) { continue; }
        if (t.type === 'keyword') {
            const u = t.value.toUpperCase();
            if (SELECT_CLAUSE_ENDERS.has(u)) {
                // Cursor is past the column list (in WHERE / FROM / etc.)
                return null;
            }
            if (u === 'SELECT') {
                selectIdx = i;
                break;
            }
        }
    }

    if (selectIdx < 0) { return null; }

    // ── 3. Scan forward from SELECT to collect all FROM/JOIN sources ─────
    //
    // Track depth: '(' → depth++, ')' → depth-- (depth < 0 means we left our scope)
    // Only process FROM/JOIN tokens at depth 0.
    // Stop at UNION / INTERSECT / EXCEPT / end-of-scope.
    //
    const sources: SelectSource[] = [];
    depth = 0;
    const SCOPE_ENDERS = new Set(['UNION', 'INTERSECT', 'EXCEPT']);

    for (let i = selectIdx + 1; i < tokens.length; i++) {
        const t = tokens[i];
        if (t.type === 'operator') {
            if (t.value === '(') { depth++; }
            if (t.value === ')') {
                depth--;
                if (depth < 0) { break; }   // left our scope
            }
        }
        if (depth !== 0) { continue; }
        if (t.type !== 'keyword') { continue; }

        const u = t.value.toUpperCase();
        if (SCOPE_ENDERS.has(u)) { break; }

        // Collect FROM / all JOIN variants
        if (u === 'FROM' || u.endsWith('JOIN')) {
            const result = readTableRefAt(tokens, i + 1, cteMap);
            if (result) { sources.push(result.source); }
        }
    }

    if (sources.length === 0) { return null; }
    return { sources };
}

/**
 * Read a table or CTE reference starting at token index `j`.
 * Returns { source, nextIdx } or null if no recognisable reference.
 */
function readTableRefAt(
    tokens: Token[],
    j: number,
    cteMap: Map<string, CteDefinition>,
): { source: SelectSource; nextIdx: number } | null {
    if (j >= tokens.length) { return null; }

    const t = tokens[j];
    let fullName: string | null = null;
    let nextJ = j + 1;

    if (t.type === 'backtick') {
        fullName = t.value.slice(1, -1);
    } else if (t.type === 'identifier') {
        let name = t.value;
        nextJ = j + 1;
        // Consume dotted chain:  dataset.table  or  project.dataset.table
        while (
            nextJ < tokens.length &&
            tokens[nextJ].type === 'operator' && tokens[nextJ].value === '.' &&
            nextJ + 1 < tokens.length &&
            (tokens[nextJ + 1].type === 'identifier' || tokens[nextJ + 1].type === 'backtick')
        ) {
            const part = tokens[nextJ + 1];
            name += '.' + (part.type === 'backtick' ? part.value.slice(1, -1) : part.value);
            nextJ += 2;
        }
        fullName = name;
    } else if (t.type === 'operator' && t.value === '(') {
        return null;  // subquery — skip
    } else {
        return null;
    }

    if (!fullName) { return null; }

    // Parse optional alias: [AS] identifier
    let alias: string | null = null;
    let k = nextJ;
    if (k < tokens.length) {
        if (tokens[k].type === 'keyword' && tokens[k].value.toUpperCase() === 'AS') {
            k++;
            if (k < tokens.length && tokens[k].type === 'identifier') {
                alias = tokens[k].value;
                nextJ = k + 1;
            }
        } else if (
            tokens[k].type === 'identifier' &&
            !ALIAS_STOP_KEYWORDS.has(tokens[k].value.toUpperCase())
        ) {
            alias = tokens[k].value;
            nextJ = k + 1;
        }
    }

    // CTE?
    const cteDef = cteMap.get(fullName.toLowerCase());
    if (cteDef) {
        return {
            source: { kind: 'cte', fullName, alias, cteColumns: cteDef.columns },
            nextIdx: nextJ,
        };
    }

    // Real table must have at least one dot (project.dataset.table or dataset.table)
    if (!fullName.includes('.')) {
        // Bare identifier that's not a known CTE — skip
        return null;
    }

    return {
        source: { kind: 'table', fullName, alias, cteColumns: [] },
        nextIdx: nextJ,
    };
}

// ---------------------------------------------------------------------------
// Dot-trigger context  (alias. → column list for that specific table)
// ---------------------------------------------------------------------------

/**
 * Given the SQL text and cursor position, returns the full table name whose
 * columns should be suggested when the user typed "alias." or "tablename."
 *
 * Returns null when the cursor is not in a "qualifier." position.
 */
export function getColumnSuggestionContext(
    sql: string,
    line: number,
    col: number,
    refs: TableRef[],
): string | null {
    const lines = sql.split('\n');
    const currentLine = lines[line] ?? '';
    const textBefore = currentLine.substring(0, col);

    // Match  alias.  at the end of the typed text
    const m = /([a-zA-Z_$][a-zA-Z0-9_$]*)\.$/u.exec(textBefore);
    if (!m) { return null; }

    const qualifier = m[1];

    // Exact alias match
    const byAlias = refs.find(r => r.alias === qualifier);
    if (byAlias) { return byAlias.fullName; }

    // Match by last part of the table name
    const byLastPart = refs.find(
        r => r.alias === null && r.fullName.split('.').pop()?.toLowerCase() === qualifier.toLowerCase()
    );
    if (byLastPart) { return byLastPart.fullName; }

    return null;
}

// ---------------------------------------------------------------------------
// Higher-level static helpers used by the VS Code providers
// ---------------------------------------------------------------------------

export class BqsqlTsParser {

    static tokenize(sql: string): Token[] {
        return tokenize(sql);
    }

    static extractTableRefs(sql: string): TableRef[] {
        const tokens = tokenize(sql);
        return extractTableRefs(tokens);
    }

    /**
     * Dot-trigger: "alias." → columns for that table.
     */
    static getColumnSuggestionContext(sql: string, line: number, col: number): string | null {
        const tokens = tokenize(sql);
        const refs = extractTableRefs(tokens);
        return getColumnSuggestionContext(sql, line, col, refs);
    }

    /**
     * SELECT-clause context: cursor is inside a SELECT column list →
     * returns all tables/CTEs that SELECT draws from, so we can offer
     * their columns as completions.
     */
    static getSelectContext(sql: string, line: number, col: number): SelectContext | null {
        return getSelectContext(sql, line, col);
    }
}
