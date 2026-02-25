import * as vscode from 'vscode';
import { CompletionItemProvider, CompletionItem, CancellationToken, CompletionContext, CompletionList, Position, TextDocument, CompletionItemKind } from 'vscode';
import { bigqueryTableSchemaService } from '../extension';
import { BqsqlTsParser, SelectSource } from './bqsqlTsParser';


export class BqsqlCompletionItemProvider implements CompletionItemProvider<CompletionItem> {

    async provideCompletionItems(
        document: TextDocument,
        position: Position,
        token: CancellationToken,
        _context: CompletionContext,
    ): Promise<vscode.CompletionList<vscode.CompletionItem> | vscode.CompletionItem[] | null | undefined> {

        const sql = document.getText();

        // ── 1. Dot-trigger:  alias.  →  columns for that specific table ───
        // IMPORTANT: in any "qualifier." context we must NEVER fall through to
        // the function list – that causes Copilot to suggest non-existent columns.
        const dotFullName = BqsqlTsParser.getColumnSuggestionContext(sql, position.line, position.character);
        if (dotFullName) {
            return this.columnsForTable(dotFullName, token);
        }

        // ── 2. SELECT clause context  (cursor inside SELECT … FROM block) ─
        // Column lookup is SYNCHRONOUS from cache so it never races against
        // VS Code's cancellation timeout.  A background load is kicked off
        // whenever the cache is empty; the next trigger will include columns.
        const selectCtx = BqsqlTsParser.getSelectContext(sql, position.line, position.character);
        if (selectCtx) {
            // Always warm schemas in the background for table sources.
            this.backgroundLoadSchemas(selectCtx.sources);

            let columnItems = this.columnItemsFromCache(selectCtx.sources);

            // Fast path for the common case: a single table source with cold cache.
            // Try one direct load on the same request so users can see columns
            // immediately (e.g. typing "pro" should surface "profile").
            if (columnItems.length === 0 && !token.isCancellationRequested) {
                const tableSources = selectCtx.sources.filter((s): s is SelectSource => s.kind === 'table');
                if (tableSources.length === 1) {
                    try {
                        await bigqueryTableSchemaService.preLoadSchemaByFullName(tableSources[0].fullName);
                        columnItems = this.columnItemsFromCache(selectCtx.sources);
                    } catch {
                        // Keep fallback behavior: return functions even if schema fetch fails.
                    }
                }
            }

            const baseList = this.getBaseCompletionList();
            for (const fn of baseList.items) {
                fn.sortText = 'z' + (fn.sortText ?? fn.label);
            }

            // Add a hint if we still have no columns (e.g. schema load failed or is taking too long)
            if (columnItems.length === 0) {
                const hint = new CompletionItem('(schema loading...)', CompletionItemKind.Text);
                hint.detail = 'Table schema is loading or unavailable';
                hint.insertText = '';
                hint.filterText = '\0';
                hint.sortText = '00000';
                columnItems.push(hint);
            }

            return new CompletionList([...columnItems, ...baseList.items], false);
        }

        // ── 3. Function / keyword completions (outside any column context) ─
        return this.getBaseCompletionList();
    }

    // -----------------------------------------------------------------------
    // Dot-trigger helper – single table, column list
    // -----------------------------------------------------------------------

    private async columnsForTable(
        fullName: string,
        token: CancellationToken,
    ): Promise<CompletionList> {

        let schema = bigqueryTableSchemaService.getSchemaByFullName(fullName);

        if (schema.length === 0 && !token.isCancellationRequested) {
            try {
                await bigqueryTableSchemaService.preLoadSchemaByFullName(fullName);
                schema = bigqueryTableSchemaService.getSchemaByFullName(fullName);
            } catch {
                return new CompletionList([], false);
            }
        }
        if (token.isCancellationRequested) { return new CompletionList([], false); }

        if (schema.length === 0) {
            const hint = new CompletionItem('(no schema available)', CompletionItemKind.Text);
            hint.detail = `Could not load schema for ${fullName}`;
            hint.insertText = '';
            hint.filterText = '\0';
            return new CompletionList([hint], false);
        }

        return new CompletionList(
            dedupeByName(schema).map((col, idx) => {
                const item = new CompletionItem(col.column_name, CompletionItemKind.Field);
                item.detail = col.data_type +
                    (col.is_partitioning_column === 'YES' ? '  [PARTITION KEY]' : '');
                item.documentation = buildColumnDoc(col.column_name, col.data_type,
                    col.is_partitioning_column === 'YES', col.description);
                item.sortText = String(idx).padStart(5, '0');
                item.commitCharacters = ['\t'];
                return item;
            }),
            false,
        );
    }

    // -----------------------------------------------------------------------
    // SELECT-clause context helpers – SYNCHRONOUS (cache reads only)
    // -----------------------------------------------------------------------

    /** Read columns from the in-memory cache – returns immediately, never blocks. */
    private columnItemsFromCache(sources: SelectSource[]): CompletionItem[] {
        const multiSource = sources.length > 1;
        const items: CompletionItem[] = [];
        let globalIdx = 0;

        for (const source of sources) {
            const qualifier = source.alias ?? source.fullName.split('.').pop() ?? source.fullName;

            if (source.kind === 'cte') {
                for (const col of source.cteColumns) {
                    const label = multiSource ? `${qualifier}.${col}` : col;
                    const item = new CompletionItem(label, CompletionItemKind.Field);
                    item.detail = `CTE column · ${qualifier}`;
                    item.documentation = new vscode.MarkdownString(
                        `**\`${col}\`** from CTE **\`${qualifier}\`**`
                    );
                    item.sortText = '0' + String(globalIdx++).padStart(5, '0');
                    item.commitCharacters = ['\t'];
                    items.push(item);
                }
            } else {
                const schema = dedupeByName(bigqueryTableSchemaService.getSchemaByFullName(source.fullName));
                for (const col of schema) {
                    const label = multiSource ? `${qualifier}.${col.column_name}` : col.column_name;
                    const item = new CompletionItem(label, CompletionItemKind.Field);
                    item.detail = col.data_type +
                        (col.is_partitioning_column === 'YES' ? '  [PARTITION KEY]' : '') +
                        (multiSource ? `  · ${qualifier}` : '');
                    item.documentation = buildColumnDoc(col.column_name, col.data_type,
                        col.is_partitioning_column === 'YES', col.description);
                    item.sortText = '0' + String(globalIdx++).padStart(5, '0');
                    item.commitCharacters = ['\t'];
                    items.push(item);
                }
            }
        }

        return items;
    }

    /** Fire-and-forget: load schemas for any uncached real tables. */
    private backgroundLoadSchemas(sources: SelectSource[]): void {
        for (const source of sources) {
            if (source.kind === 'table') {
                bigqueryTableSchemaService
                    .preLoadSchemaByFullName(source.fullName)
                    .catch(() => undefined);
            }
        }
    }

    getBaseCompletionList(): CompletionList<CompletionItem> {

        return new CompletionList<CompletionItem>(
            [

                this.getCompletionItem("COALESCE", CompletionItemKind.Function),
                this.getCompletionItem("IF", CompletionItemKind.Function),
                this.getCompletionItem("IFNULL", CompletionItemKind.Function),
                this.getCompletionItem("NULLIF", CompletionItemKind.Function),
                this.getCompletionItem("ANY_VALUE", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_AGG", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_CONCAT_AGG", CompletionItemKind.Function),
                this.getCompletionItem("AVG", CompletionItemKind.Function),
                this.getCompletionItem("BIT_AND", CompletionItemKind.Function),
                this.getCompletionItem("BIT_OR", CompletionItemKind.Function),
                this.getCompletionItem("BIT_XOR", CompletionItemKind.Function),
                this.getCompletionItem("COUNT", CompletionItemKind.Function),
                this.getCompletionItem("COUNTIF", CompletionItemKind.Function),
                this.getCompletionItem("GROUPING", CompletionItemKind.Function),
                this.getCompletionItem("LOGICAL_AND", CompletionItemKind.Function),
                this.getCompletionItem("LOGICAL_OR", CompletionItemKind.Function),
                this.getCompletionItem("MAX", CompletionItemKind.Function),
                this.getCompletionItem("MAX_BY", CompletionItemKind.Function),
                this.getCompletionItem("MIN", CompletionItemKind.Function),
                this.getCompletionItem("MIN_BY", CompletionItemKind.Function),
                this.getCompletionItem("STRING_AGG", CompletionItemKind.Function),
                this.getCompletionItem("SUM", CompletionItemKind.Function),
                this.getCompletionItem("CORR", CompletionItemKind.Function),
                this.getCompletionItem("COVAR_POP", CompletionItemKind.Function),
                this.getCompletionItem("COVAR_SAMP", CompletionItemKind.Function),
                this.getCompletionItem("STDDEV_POP", CompletionItemKind.Function),
                this.getCompletionItem("STDDEV_SAMP", CompletionItemKind.Function),
                this.getCompletionItem("STDDEV", CompletionItemKind.Function),
                this.getCompletionItem("VAR_POP", CompletionItemKind.Function),
                this.getCompletionItem("VAR_SAMP", CompletionItemKind.Function),
                this.getCompletionItem("VARIANCE", CompletionItemKind.Function),
                this.getCompletionItem("APPROX_COUNT_DISTINCT", CompletionItemKind.Function),
                this.getCompletionItem("APPROX_QUANTILES", CompletionItemKind.Function),
                this.getCompletionItem("APPROX_TOP_COUNT", CompletionItemKind.Function),
                this.getCompletionItem("APPROX_TOP_SUM", CompletionItemKind.Function),
                this.getCompletionItem("HLL_COUNT.INIT", CompletionItemKind.Function),
                this.getCompletionItem("HLL_COUNT.MERGE", CompletionItemKind.Function),
                this.getCompletionItem("HLL_COUNT.MERGE_PARTIAL", CompletionItemKind.Function),
                this.getCompletionItem("HLL_COUNT.EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.INIT_INT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.INIT_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.MERGE_INT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.MERGE_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.MERGE_PARTIAL", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.MERGE_POINT_INT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.MERGE_POINT_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.EXTRACT_INT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.EXTRACT_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.EXTRACT_POINT_INT64", CompletionItemKind.Function),
                this.getCompletionItem("KLL_QUANTILES.EXTRACT_POINT_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("RANK", CompletionItemKind.Function),
                this.getCompletionItem("DENSE_RANK", CompletionItemKind.Function),
                this.getCompletionItem("PERCENT_RANK", CompletionItemKind.Function),
                this.getCompletionItem("CUME_DIST", CompletionItemKind.Function),
                this.getCompletionItem("NTILE", CompletionItemKind.Function),
                this.getCompletionItem("ROW_NUMBER", CompletionItemKind.Function),
                this.getCompletionItem("BIT_COUNT", CompletionItemKind.Function),
                this.getCompletionItem("CAST", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_BIGNUMERIC", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_NUMERIC", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_CAST", CompletionItemKind.Function),
                this.getCompletionItem("TYPEOF", CompletionItemKind.Function),
                this.getCompletionItem("ABS", CompletionItemKind.Function),
                this.getCompletionItem("SIGN", CompletionItemKind.Function),
                this.getCompletionItem("IS_INF", CompletionItemKind.Function),
                this.getCompletionItem("IS_NAN", CompletionItemKind.Function),
                this.getCompletionItem("IEEE_DIVIDE", CompletionItemKind.Function),
                this.getCompletionItem("RAND", CompletionItemKind.Function),
                this.getCompletionItem("SQRT", CompletionItemKind.Function),
                this.getCompletionItem("POW", CompletionItemKind.Function),
                this.getCompletionItem("POWER", CompletionItemKind.Function),
                this.getCompletionItem("EXP", CompletionItemKind.Function),
                this.getCompletionItem("LN", CompletionItemKind.Function),
                this.getCompletionItem("LOG", CompletionItemKind.Function),
                this.getCompletionItem("LOG10", CompletionItemKind.Function),
                this.getCompletionItem("GREATEST", CompletionItemKind.Function),
                this.getCompletionItem("LEAST", CompletionItemKind.Function),
                this.getCompletionItem("DIV", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_DIVIDE", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_MULTIPLY", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_NEGATE", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_ADD", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_SUBTRACT", CompletionItemKind.Function),
                this.getCompletionItem("MOD", CompletionItemKind.Function),
                this.getCompletionItem("ROUND", CompletionItemKind.Function),
                this.getCompletionItem("TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("CEIL", CompletionItemKind.Function),
                this.getCompletionItem("CEILING", CompletionItemKind.Function),
                this.getCompletionItem("FLOOR", CompletionItemKind.Function),
                this.getCompletionItem("COS", CompletionItemKind.Function),
                this.getCompletionItem("COSH", CompletionItemKind.Function),
                this.getCompletionItem("ACOS", CompletionItemKind.Function),
                this.getCompletionItem("ACOSH", CompletionItemKind.Function),
                this.getCompletionItem("COT", CompletionItemKind.Function),
                this.getCompletionItem("COTH", CompletionItemKind.Function),
                this.getCompletionItem("CSC", CompletionItemKind.Function),
                this.getCompletionItem("CSCH", CompletionItemKind.Function),
                this.getCompletionItem("SEC", CompletionItemKind.Function),
                this.getCompletionItem("SECH", CompletionItemKind.Function),
                this.getCompletionItem("SIN", CompletionItemKind.Function),
                this.getCompletionItem("SINH", CompletionItemKind.Function),
                this.getCompletionItem("ASIN", CompletionItemKind.Function),
                this.getCompletionItem("ASINH", CompletionItemKind.Function),
                this.getCompletionItem("TAN", CompletionItemKind.Function),
                this.getCompletionItem("TANH", CompletionItemKind.Function),
                this.getCompletionItem("ATAN", CompletionItemKind.Function),
                this.getCompletionItem("ATANH", CompletionItemKind.Function),
                this.getCompletionItem("ATAN2", CompletionItemKind.Function),
                this.getCompletionItem("CBRT", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_BUCKET", CompletionItemKind.Function),
                this.getCompletionItem("RANGE", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_CONTAINS", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_END", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_INTERSECT", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_OVERLAPS", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_SESSIONIZE", CompletionItemKind.Function),
                this.getCompletionItem("RANGE_START", CompletionItemKind.Function),
                this.getCompletionItem("COSINE_DISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("EUCLIDEAN_DISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("EDIT_DISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("FIRST_VALUE", CompletionItemKind.Function),
                this.getCompletionItem("LAST_VALUE", CompletionItemKind.Function),
                this.getCompletionItem("NTH_VALUE", CompletionItemKind.Function),
                this.getCompletionItem("LEAD", CompletionItemKind.Function),
                this.getCompletionItem("LAG", CompletionItemKind.Function),
                this.getCompletionItem("PERCENTILE_CONT", CompletionItemKind.Function),
                this.getCompletionItem("PERCENTILE_DISC", CompletionItemKind.Function),
                this.getCompletionItem("FARM_FINGERPRINT", CompletionItemKind.Function),
                this.getCompletionItem("MD5", CompletionItemKind.Function),
                this.getCompletionItem("SHA1", CompletionItemKind.Function),
                this.getCompletionItem("SHA256", CompletionItemKind.Function),
                this.getCompletionItem("SHA512", CompletionItemKind.Function),
                this.getCompletionItem("ASCII", CompletionItemKind.Function),
                this.getCompletionItem("BYTE_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("CHAR_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("CHARACTER_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("CHR", CompletionItemKind.Function),
                this.getCompletionItem("CODE_POINTS_TO_BYTES", CompletionItemKind.Function),
                this.getCompletionItem("CODE_POINTS_TO_STRING", CompletionItemKind.Function),
                this.getCompletionItem("COLLATE", CompletionItemKind.Function),
                this.getCompletionItem("CONCAT", CompletionItemKind.Function),
                this.getCompletionItem("CONTAINS_SUBSTR", CompletionItemKind.Function),
                this.getCompletionItem("ENDS_WITH", CompletionItemKind.Function),
                this.getCompletionItem("FORMAT", CompletionItemKind.Function),
                this.getCompletionItem("FROM_BASE32", CompletionItemKind.Function),
                this.getCompletionItem("FROM_BASE64", CompletionItemKind.Function),
                this.getCompletionItem("FROM_HEX", CompletionItemKind.Function),
                this.getCompletionItem("INITCAP", CompletionItemKind.Function),
                this.getCompletionItem("INSTR", CompletionItemKind.Function),
                this.getCompletionItem("LEFT", CompletionItemKind.Function),
                this.getCompletionItem("LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("LPAD", CompletionItemKind.Function),
                this.getCompletionItem("LOWER", CompletionItemKind.Function),
                this.getCompletionItem("LTRIM", CompletionItemKind.Function),
                this.getCompletionItem("NORMALIZE", CompletionItemKind.Function),
                this.getCompletionItem("NORMALIZE_AND_CASEFOLD", CompletionItemKind.Function),
                this.getCompletionItem("OCTET_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_CONTAINS", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_EXTRACT_ALL", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_INSTR", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_REPLACE", CompletionItemKind.Function),
                this.getCompletionItem("REGEXP_SUBSTR", CompletionItemKind.Function),
                this.getCompletionItem("REPLACE", CompletionItemKind.Function),
                this.getCompletionItem("REPEAT", CompletionItemKind.Function),
                this.getCompletionItem("REVERSE", CompletionItemKind.Function),
                this.getCompletionItem("RIGHT", CompletionItemKind.Function),
                this.getCompletionItem("RPAD", CompletionItemKind.Function),
                this.getCompletionItem("RTRIM", CompletionItemKind.Function),
                this.getCompletionItem("SAFE_CONVERT_BYTES_TO_STRING", CompletionItemKind.Function),
                this.getCompletionItem("SOUNDEX", CompletionItemKind.Function),
                this.getCompletionItem("SPLIT", CompletionItemKind.Function),
                this.getCompletionItem("STARTS_WITH", CompletionItemKind.Function),
                this.getCompletionItem("STRPOS", CompletionItemKind.Function),
                this.getCompletionItem("SUBSTR", CompletionItemKind.Function),
                this.getCompletionItem("SUBSTRING", CompletionItemKind.Function),
                this.getCompletionItem("TO_BASE32", CompletionItemKind.Function),
                this.getCompletionItem("TO_BASE64", CompletionItemKind.Function),
                this.getCompletionItem("TO_CODE_POINTS", CompletionItemKind.Function),
                this.getCompletionItem("TO_HEX", CompletionItemKind.Function),
                this.getCompletionItem("TRANSLATE", CompletionItemKind.Function),
                this.getCompletionItem("TRIM", CompletionItemKind.Function),
                this.getCompletionItem("UNICODE", CompletionItemKind.Function),
                this.getCompletionItem("UPPER", CompletionItemKind.Function),
                this.getCompletionItem("JSON_EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("JSON_QUERY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_EXTRACT_SCALAR", CompletionItemKind.Function),
                this.getCompletionItem("JSON_VALUE", CompletionItemKind.Function),
                this.getCompletionItem("JSON_EXTRACT_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_QUERY_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_EXTRACT_STRING_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_VALUE_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("JSON_ARRAY_APPEND", CompletionItemKind.Function),
                this.getCompletionItem("JSON_ARRAY_INSERT", CompletionItemKind.Function),
                this.getCompletionItem("JSON_FLATTEN", CompletionItemKind.Function),
                this.getCompletionItem("JSON_KEYS", CompletionItemKind.Function),
                this.getCompletionItem("JSON_OBJECT", CompletionItemKind.Function),
                this.getCompletionItem("JSON_REMOVE", CompletionItemKind.Function),
                this.getCompletionItem("JSON_SET", CompletionItemKind.Function),
                this.getCompletionItem("JSON_STRIP_NULLS", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_JSON", CompletionItemKind.Function),
                this.getCompletionItem("TO_JSON", CompletionItemKind.Function),
                this.getCompletionItem("TO_JSON_STRING", CompletionItemKind.Function),
                this.getCompletionItem("STRING", CompletionItemKind.Function),
                this.getCompletionItem("BOOL", CompletionItemKind.Function),
                this.getCompletionItem("INT64", CompletionItemKind.Function),
                this.getCompletionItem("FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("JSON_TYPE", CompletionItemKind.Function),
                this.getCompletionItem("LAX_BOOL", CompletionItemKind.Function),
                this.getCompletionItem("LAX_FLOAT64", CompletionItemKind.Function),
                this.getCompletionItem("LAX_INT64", CompletionItemKind.Function),
                this.getCompletionItem("LAX_STRING", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_CONCAT", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_TO_STRING", CompletionItemKind.Function),
                this.getCompletionItem("GENERATE_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("GENERATE_DATE_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("GENERATE_TIMESTAMP_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("GENERATE_RANGE_ARRAY", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_REVERSE", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_FIRST", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_LAST", CompletionItemKind.Function),
                this.getCompletionItem("ARRAY_SLICE", CompletionItemKind.Function),
                this.getCompletionItem("CURRENT_DATE", CompletionItemKind.Function),
                this.getCompletionItem("EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("DATE", CompletionItemKind.Function),
                this.getCompletionItem("DATE_ADD", CompletionItemKind.Function),
                this.getCompletionItem("DATE_BUCKET", CompletionItemKind.Function),
                this.getCompletionItem("DATE_SUB", CompletionItemKind.Function),
                this.getCompletionItem("DATE_DIFF", CompletionItemKind.Function),
                this.getCompletionItem("DATE_TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("DATE_FROM_UNIX_DATE", CompletionItemKind.Function),
                this.getCompletionItem("FORMAT_DATE", CompletionItemKind.Function),
                this.getCompletionItem("LAST_DAY", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_DATE", CompletionItemKind.Function),
                this.getCompletionItem("UNIX_DATE", CompletionItemKind.Function),
                this.getCompletionItem("CURRENT_DATETIME", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME", CompletionItemKind.Function),
                this.getCompletionItem("EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME_ADD", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME_SUB", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME_DIFF", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME_BUCKET", CompletionItemKind.Function),
                this.getCompletionItem("DATETIME_TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("FORMAT_DATETIME", CompletionItemKind.Function),
                this.getCompletionItem("LAST_DAY", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_DATETIME", CompletionItemKind.Function),
                this.getCompletionItem("CURRENT_TIME", CompletionItemKind.Function),
                this.getCompletionItem("TIME", CompletionItemKind.Function),
                this.getCompletionItem("EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("TIME_ADD", CompletionItemKind.Function),
                this.getCompletionItem("TIME_SUB", CompletionItemKind.Function),
                this.getCompletionItem("TIME_DIFF", CompletionItemKind.Function),
                this.getCompletionItem("TIME_TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("FORMAT_TIME", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_TIME", CompletionItemKind.Function),
                this.getCompletionItem("CURRENT_TIMESTAMP", CompletionItemKind.Function),
                this.getCompletionItem("EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("STRING", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_ADD", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_BUCKET", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_SUB", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_DIFF", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("FORMAT_TIMESTAMP", CompletionItemKind.Function),
                this.getCompletionItem("PARSE_TIMESTAMP", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_SECONDS", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_MILLIS", CompletionItemKind.Function),
                this.getCompletionItem("TIMESTAMP_MICROS", CompletionItemKind.Function),
                this.getCompletionItem("UNIX_SECONDS", CompletionItemKind.Function),
                this.getCompletionItem("UNIX_MILLIS", CompletionItemKind.Function),
                this.getCompletionItem("UNIX_MICROS", CompletionItemKind.Function),
                this.getCompletionItem("MAKE_INTERVAL", CompletionItemKind.Function),
                this.getCompletionItem("EXTRACT", CompletionItemKind.Function),
                this.getCompletionItem("JUSTIFY_DAYS", CompletionItemKind.Function),
                this.getCompletionItem("JUSTIFY_HOURS", CompletionItemKind.Function),
                this.getCompletionItem("JUSTIFY_INTERVAL", CompletionItemKind.Function),
                this.getCompletionItem("S2_CELLIDFROMPOINT", CompletionItemKind.Function),
                this.getCompletionItem("S2_COVERINGCELLIDS", CompletionItemKind.Function),
                this.getCompletionItem("ST_ANGLE", CompletionItemKind.Function),
                this.getCompletionItem("ST_AREA", CompletionItemKind.Function),
                this.getCompletionItem("ST_ASBINARY", CompletionItemKind.Function),
                this.getCompletionItem("ST_ASGEOJSON", CompletionItemKind.Function),
                this.getCompletionItem("ST_ASTEXT", CompletionItemKind.Function),
                this.getCompletionItem("ST_AZIMUTH", CompletionItemKind.Function),
                this.getCompletionItem("ST_BOUNDARY", CompletionItemKind.Function),
                this.getCompletionItem("ST_BOUNDINGBOX", CompletionItemKind.Function),
                this.getCompletionItem("ST_BUFFER", CompletionItemKind.Function),
                this.getCompletionItem("ST_BUFFERWITHTOLERANCE", CompletionItemKind.Function),
                this.getCompletionItem("ST_CENTROID", CompletionItemKind.Function),
                this.getCompletionItem("ST_CENTROID_AGG", CompletionItemKind.Function),
                this.getCompletionItem("ST_CLOSESTPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_CLUSTERDBSCAN", CompletionItemKind.Function),
                this.getCompletionItem("ST_CONTAINS", CompletionItemKind.Function),
                this.getCompletionItem("ST_CONVEXHULL", CompletionItemKind.Function),
                this.getCompletionItem("ST_COVEREDBY", CompletionItemKind.Function),
                this.getCompletionItem("ST_COVERS", CompletionItemKind.Function),
                this.getCompletionItem("ST_DIFFERENCE", CompletionItemKind.Function),
                this.getCompletionItem("ST_DIMENSION", CompletionItemKind.Function),
                this.getCompletionItem("ST_DISJOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_DISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("ST_DUMP", CompletionItemKind.Function),
                this.getCompletionItem("ST_DWITHIN", CompletionItemKind.Function),
                this.getCompletionItem("ST_ENDPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_EQUALS", CompletionItemKind.Function),
                this.getCompletionItem("ST_EXTENT", CompletionItemKind.Function),
                this.getCompletionItem("ST_EXTERIORRING", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGFROM", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGFROMGEOJSON", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGFROMTEXT", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGFROMWKB", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOGPOINTFROMGEOHASH", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOHASH", CompletionItemKind.Function),
                this.getCompletionItem("ST_GEOMETRYTYPE", CompletionItemKind.Function),
                this.getCompletionItem("ST_HAUSDORFFDISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("ST_HAUSDORFFDWITHIN", CompletionItemKind.Function),
                this.getCompletionItem("ST_INTERIORRINGS", CompletionItemKind.Function),
                this.getCompletionItem("ST_INTERSECTION", CompletionItemKind.Function),
                this.getCompletionItem("ST_INTERSECTS", CompletionItemKind.Function),
                this.getCompletionItem("ST_INTERSECTSBOX", CompletionItemKind.Function),
                this.getCompletionItem("ST_ISCLOSED", CompletionItemKind.Function),
                this.getCompletionItem("ST_ISCOLLECTION", CompletionItemKind.Function),
                this.getCompletionItem("ST_ISEMPTY", CompletionItemKind.Function),
                this.getCompletionItem("ST_ISRING", CompletionItemKind.Function),
                this.getCompletionItem("ST_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("ST_LINEINTERPOLATEPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_LINELOCATEPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_LINESUBSTRING", CompletionItemKind.Function),
                this.getCompletionItem("ST_MAKELINE", CompletionItemKind.Function),
                this.getCompletionItem("ST_MAKEPOLYGON", CompletionItemKind.Function),
                this.getCompletionItem("ST_MAKEPOLYGONORIENTED", CompletionItemKind.Function),
                this.getCompletionItem("ST_MAXDISTANCE", CompletionItemKind.Function),
                this.getCompletionItem("ST_NPOINTS", CompletionItemKind.Function),
                this.getCompletionItem("ST_NUMGEOMETRIES", CompletionItemKind.Function),
                this.getCompletionItem("ST_NUMPOINTS", CompletionItemKind.Function),
                this.getCompletionItem("ST_PERIMETER", CompletionItemKind.Function),
                this.getCompletionItem("ST_POINTN", CompletionItemKind.Function),
                this.getCompletionItem("ST_REGIONSTATS", CompletionItemKind.Function),
                this.getCompletionItem("ST_SIMPLIFY", CompletionItemKind.Function),
                this.getCompletionItem("ST_SNAPTOGRID", CompletionItemKind.Function),
                this.getCompletionItem("ST_STARTPOINT", CompletionItemKind.Function),
                this.getCompletionItem("ST_TOUCHES", CompletionItemKind.Function),
                this.getCompletionItem("ST_UNION", CompletionItemKind.Function),
                this.getCompletionItem("ST_UNION_AGG", CompletionItemKind.Function),
                this.getCompletionItem("ST_WITHIN", CompletionItemKind.Function),
                this.getCompletionItem("ST_X", CompletionItemKind.Function),
                this.getCompletionItem("ST_Y", CompletionItemKind.Function),
                this.getCompletionItem("SESSION_USER", CompletionItemKind.Function),
                this.getCompletionItem("GENERATE_UUID", CompletionItemKind.Function),
                this.getCompletionItem("SEARCH", CompletionItemKind.Function),
                this.getCompletionItem("GAP_FILL", CompletionItemKind.Function),
                this.getCompletionItem("NET.IP_FROM_STRING", CompletionItemKind.Function),
                this.getCompletionItem("NET.SAFE_IP_FROM_STRING", CompletionItemKind.Function),
                this.getCompletionItem("NET.IP_TO_STRING", CompletionItemKind.Function),
                this.getCompletionItem("NET.IP_NET_MASK", CompletionItemKind.Function),
                this.getCompletionItem("NET.IP_TRUNC", CompletionItemKind.Function),
                this.getCompletionItem("NET.IPV4_FROM_INT64", CompletionItemKind.Function),
                this.getCompletionItem("NET.IPV4_TO_INT64", CompletionItemKind.Function),
                this.getCompletionItem("NET.HOST", CompletionItemKind.Function),
                this.getCompletionItem("NET.PUBLIC_SUFFIX", CompletionItemKind.Function),
                this.getCompletionItem("NET.REG_DOMAIN", CompletionItemKind.Function),
                this.getCompletionItem("ERROR", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.NEW_KEYSET", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.NEW_WRAPPED_KEYSET", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.REWRAP_KEYSET", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.ADD_KEY_FROM_RAW_BYTES", CompletionItemKind.Function),
                this.getCompletionItem("AEAD.DECRYPT_BYTES", CompletionItemKind.Function),
                this.getCompletionItem("AEAD.DECRYPT_STRING", CompletionItemKind.Function),
                this.getCompletionItem("AEAD.ENCRYPT", CompletionItemKind.Function),
                this.getCompletionItem("DETERMINISTIC_DECRYPT_BYTES", CompletionItemKind.Function),
                this.getCompletionItem("DETERMINISTIC_DECRYPT_STRING", CompletionItemKind.Function),
                this.getCompletionItem("DETERMINISTIC_ENCRYPT", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.KEYSET_CHAIN", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.KEYSET_FROM_JSON", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.KEYSET_TO_JSON", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.ROTATE_KEYSET", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.ROTATE_WRAPPED_KEYSET", CompletionItemKind.Function),
                this.getCompletionItem("KEYS.KEYSET_LENGTH", CompletionItemKind.Function),
                this.getCompletionItem("EXTERNAL_OBJECT_TRANSFORM", CompletionItemKind.Function),
                this.getCompletionItem("EXTERNAL_QUERY", CompletionItemKind.Function),
                this.getCompletionItem("APPENDS", CompletionItemKind.Function),
                this.getCompletionItem("CHANGES", CompletionItemKind.Function),
                this.getCompletionItem("OBJ.FETCH_METADATA", CompletionItemKind.Function),
                this.getCompletionItem("OBJ.GET_ACCESS_URL", CompletionItemKind.Function),
                this.getCompletionItem("OBJ.MAKE_REF", CompletionItemKind.Function),
                this.getCompletionItem("DLP_DETERMINISTIC_ENCRYPT", CompletionItemKind.Function),
                this.getCompletionItem("DLP_DETERMINISTIC_DECRYPT", CompletionItemKind.Function),
                this.getCompletionItem("DLP_KEY_CHAIN", CompletionItemKind.Function),
                this.getCompletionItem("TEXT_ANALYZE", CompletionItemKind.Function),
                this.getCompletionItem("BAG_OF_WORDS", CompletionItemKind.Function),
                this.getCompletionItem("TF_IDF", CompletionItemKind.Function),
                this.getCompletionItem("VECTOR_SEARCH", CompletionItemKind.Function),
                this.getCompletionItem("VECTOR_INDEX.STATISTICS", CompletionItemKind.Function)

            ]
        );

    }

    getCompletionItem(label: string, kind?: CompletionItemKind): CompletionItem {

        let completionItem = new CompletionItem(label, kind);

        const meta = FUNCTION_META[label];
        const url = meta?.url ?? 'https://docs.cloud.google.com/bigquery/docs/reference/standard-sql/functions-all';
        const desc = meta?.description ?? '';
        const md = new vscode.MarkdownString(
            (desc ? `${desc}\n\n` : '') +
            `[BigQuery documentation](${url})`
        );
        completionItem.documentation = md;
        completionItem.detail = desc || undefined;

        const snippetArgs = FUNCTION_SNIPPETS[label] ?? '($1)';
        completionItem.insertText = new vscode.SnippetString(`${label}${snippetArgs}`);

        return completionItem;
    }

}

// ---------------------------------------------------------------------------
// Per-function metadata: description + direct documentation URL
// ---------------------------------------------------------------------------

const BASE = 'https://docs.cloud.google.com/bigquery/docs/reference/standard-sql';

interface FunctionMeta { description: string; url: string; }

const FUNCTION_META: Record<string, FunctionMeta> = {
    // Conditional
    'COALESCE':                      { description: 'Returns the first non-NULL expression from a list.', url: `${BASE}/conditional_expressions#coalesce` },
    'IF':                            { description: 'Returns true_result if condition is TRUE, else false_result.', url: `${BASE}/conditional_expressions#if` },
    'IFNULL':                        { description: 'Returns expr if not NULL; otherwise returns null_result.', url: `${BASE}/conditional_expressions#ifnull` },
    'NULLIF':                        { description: 'Returns NULL if expr = expr_to_match, otherwise returns expr.', url: `${BASE}/conditional_expressions#nullif` },
    // Aggregate
    'ANY_VALUE':                     { description: 'Gets an expression for some row in the group.', url: `${BASE}/aggregate_functions#any_value` },
    'ARRAY_AGG':                     { description: 'Returns an ARRAY of expression values.', url: `${BASE}/aggregate_functions#array_agg` },
    'ARRAY_CONCAT_AGG':              { description: 'Concatenates arrays and returns a single array as a result.', url: `${BASE}/aggregate_functions#array_concat_agg` },
    'AVG':                           { description: 'Returns the average of non-NULL values.', url: `${BASE}/aggregate_functions#avg` },
    'BIT_AND':                       { description: 'Performs a bitwise AND operation on an expression.', url: `${BASE}/aggregate_functions#bit_and` },
    'BIT_OR':                        { description: 'Performs a bitwise OR operation on an expression.', url: `${BASE}/aggregate_functions#bit_or` },
    'BIT_XOR':                       { description: 'Performs a bitwise XOR operation on an expression.', url: `${BASE}/aggregate_functions#bit_xor` },
    'COUNT':                         { description: 'Gets the number of rows, or rows with a non-NULL expression.', url: `${BASE}/aggregate_functions#count` },
    'COUNTIF':                       { description: 'Gets the number of TRUE values for an expression.', url: `${BASE}/aggregate_functions#countif` },
    'GROUPING':                      { description: 'Checks if a groupable value in the GROUP BY clause is aggregated.', url: `${BASE}/aggregate_functions#grouping` },
    'LOGICAL_AND':                   { description: 'Gets the logical AND of all non-NULL expressions.', url: `${BASE}/aggregate_functions#logical_and` },
    'LOGICAL_OR':                    { description: 'Gets the logical OR of all non-NULL expressions.', url: `${BASE}/aggregate_functions#logical_or` },
    'MAX':                           { description: 'Gets the maximum non-NULL value.', url: `${BASE}/aggregate_functions#max` },
    'MAX_BY':                        { description: 'Synonym for ANY_VALUE(x HAVING MAX y). Returns the value of x associated with the maximum y.', url: `${BASE}/aggregate_functions#max_by` },
    'MIN':                           { description: 'Gets the minimum non-NULL value.', url: `${BASE}/aggregate_functions#min` },
    'MIN_BY':                        { description: 'Synonym for ANY_VALUE(x HAVING MIN y). Returns the value of x associated with the minimum y.', url: `${BASE}/aggregate_functions#min_by` },
    'STRING_AGG':                    { description: 'Gets a STRING or BYTES value obtained by concatenating non-NULL values.', url: `${BASE}/aggregate_functions#string_agg` },
    'SUM':                           { description: 'Gets the sum of non-NULL values.', url: `${BASE}/aggregate_functions#sum` },
    'CORR':                          { description: 'Computes the Pearson coefficient of correlation of a set of number pairs.', url: `${BASE}/aggregate_functions#corr` },
    'COVAR_POP':                     { description: 'Computes the population covariance of a set of number pairs.', url: `${BASE}/aggregate_functions#covar_pop` },
    'COVAR_SAMP':                    { description: 'Computes the sample covariance of a set of number pairs.', url: `${BASE}/aggregate_functions#covar_samp` },
    'STDDEV_POP':                    { description: 'Computes the population (biased) standard deviation of the values.', url: `${BASE}/aggregate_functions#stddev_pop` },
    'STDDEV_SAMP':                   { description: 'Computes the sample (unbiased) standard deviation of the values.', url: `${BASE}/aggregate_functions#stddev_samp` },
    'STDDEV':                        { description: 'An alias of STDDEV_SAMP.', url: `${BASE}/aggregate_functions#stddev` },
    'VAR_POP':                       { description: 'Computes the population (biased) variance of the values.', url: `${BASE}/aggregate_functions#var_pop` },
    'VAR_SAMP':                      { description: 'Computes the sample (unbiased) variance of the values.', url: `${BASE}/aggregate_functions#var_samp` },
    'VARIANCE':                      { description: 'An alias of VAR_SAMP.', url: `${BASE}/aggregate_functions#variance` },
    // Approximate aggregate
    'APPROX_COUNT_DISTINCT':         { description: 'Gets the approximate result for COUNT(DISTINCT expression).', url: `${BASE}/approximate_aggregate_functions#approx_count_distinct` },
    'APPROX_QUANTILES':              { description: 'Gets the approximate quantile boundaries.', url: `${BASE}/approximate_aggregate_functions#approx_quantiles` },
    'APPROX_TOP_COUNT':              { description: 'Gets the approximate top elements and their approximate count.', url: `${BASE}/approximate_aggregate_functions#approx_top_count` },
    'APPROX_TOP_SUM':                { description: 'Gets the approximate top elements and sum, based on the approximate sum of an assigned weight.', url: `${BASE}/approximate_aggregate_functions#approx_top_sum` },
    // HLL sketch
    'HLL_COUNT.INIT':                { description: 'Aggregates values of the same underlying type into a new HLL++ sketch.', url: `${BASE}/hll_functions#hll_countinit` },
    'HLL_COUNT.MERGE':               { description: 'Merges HLL++ sketches and returns the cardinality of the new sketch.', url: `${BASE}/hll_functions#hll_countmerge` },
    'HLL_COUNT.MERGE_PARTIAL':       { description: 'Merges HLL++ sketches of the same underlying type into a new sketch.', url: `${BASE}/hll_functions#hll_countmerge_partial` },
    'HLL_COUNT.EXTRACT':             { description: 'Extracts a cardinality estimate of an HLL++ sketch.', url: `${BASE}/hll_functions#hll_countextract` },
    // KLL quantiles
    'KLL_QUANTILES.INIT_INT64':           { description: 'Aggregates values into an INT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesinit_int64` },
    'KLL_QUANTILES.INIT_FLOAT64':         { description: 'Aggregates values into a FLOAT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesinit_float64` },
    'KLL_QUANTILES.MERGE_INT64':          { description: 'Merges INT64-initialized KLL sketches and gets the quantiles.', url: `${BASE}/kll_quantiles#kll_quantilesmerge_int64` },
    'KLL_QUANTILES.MERGE_FLOAT64':        { description: 'Merges FLOAT64-initialized KLL sketches and gets the quantiles.', url: `${BASE}/kll_quantiles#kll_quantilesmerge_float64` },
    'KLL_QUANTILES.MERGE_PARTIAL':        { description: 'Merges KLL sketches of the same underlying type into a new sketch.', url: `${BASE}/kll_quantiles#kll_quantilesmerge_partial` },
    'KLL_QUANTILES.MERGE_POINT_INT64':    { description: 'Merges INT64-initialized KLL sketches and gets a specific quantile.', url: `${BASE}/kll_quantiles#kll_quantilesmerge_point_int64` },
    'KLL_QUANTILES.MERGE_POINT_FLOAT64':  { description: 'Merges FLOAT64-initialized KLL sketches and gets a specific quantile.', url: `${BASE}/kll_quantiles#kll_quantilesmerge_point_float64` },
    'KLL_QUANTILES.EXTRACT_INT64':        { description: 'Gets a selected number of quantiles from an INT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesextract_int64` },
    'KLL_QUANTILES.EXTRACT_FLOAT64':      { description: 'Gets a selected number of quantiles from a FLOAT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesextract_float64` },
    'KLL_QUANTILES.EXTRACT_POINT_INT64':  { description: 'Gets a specific quantile from an INT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesextract_point_int64` },
    'KLL_QUANTILES.EXTRACT_POINT_FLOAT64':{ description: 'Gets a specific quantile from a FLOAT64-initialized KLL sketch.', url: `${BASE}/kll_quantiles#kll_quantilesextract_point_float64` },
    // Window / navigation
    'RANK':                          { description: 'Gets the rank (1-based) of each row within a window partition.', url: `${BASE}/navigation_functions#rank` },
    'DENSE_RANK':                    { description: 'Gets the dense rank (1-based, no gaps) of each row within a window.', url: `${BASE}/navigation_functions#dense_rank` },
    'PERCENT_RANK':                  { description: 'Gets the percentile rank (0 to 1) of each row within a window.', url: `${BASE}/navigation_functions#percent_rank` },
    'CUME_DIST':                     { description: 'Gets the cumulative distribution (relative position (0,1]) of each row within a window.', url: `${BASE}/navigation_functions#cume_dist` },
    'NTILE':                         { description: 'Gets the quantile bucket number (1-based) of each row within a window.', url: `${BASE}/navigation_functions#ntile` },
    'ROW_NUMBER':                    { description: 'Gets the sequential row number (1-based) of each row within a window partition.', url: `${BASE}/navigation_functions#row_number` },
    'FIRST_VALUE':                   { description: 'Gets a value from the first row in the current window frame.', url: `${BASE}/navigation_functions#first_value` },
    'LAST_VALUE':                    { description: 'Gets a value from the last row in the current window frame.', url: `${BASE}/navigation_functions#last_value` },
    'NTH_VALUE':                     { description: 'Gets a value for the Nth row of the current window frame.', url: `${BASE}/navigation_functions#nth_value` },
    'LEAD':                          { description: 'Gets a value for a subsequent row within the window.', url: `${BASE}/navigation_functions#lead` },
    'LAG':                           { description: 'Gets a value for a preceding row within the window.', url: `${BASE}/navigation_functions#lag` },
    'PERCENTILE_CONT':               { description: 'Computes the specified percentile for a value, using linear interpolation.', url: `${BASE}/navigation_functions#percentile_cont` },
    'PERCENTILE_DISC':               { description: 'Computes the specified percentile for a discrete value.', url: `${BASE}/navigation_functions#percentile_disc` },
    // Casting / type
    'BIT_COUNT':                     { description: 'Gets the number of bits that are set in the input expression.', url: `${BASE}/bit_functions#bit_count` },
    'CAST':                          { description: 'Converts the result of an expression to the given type.', url: `${BASE}/conversion_functions#cast` },
    'SAFE_CAST':                     { description: 'Similar to CAST but returns NULL when a runtime error would occur.', url: `${BASE}/conversion_functions#safe_cast` },
    'PARSE_BIGNUMERIC':              { description: 'Converts a STRING value to a BIGNUMERIC value.', url: `${BASE}/conversion_functions#parse_bignumeric` },
    'PARSE_NUMERIC':                 { description: 'Converts a STRING value to a NUMERIC value.', url: `${BASE}/conversion_functions#parse_numeric` },
    'TYPEOF':                        { description: 'Gets the name of the data type for an expression.', url: `${BASE}/utility_functions#typeof` },
    // Math
    'ABS':                           { description: 'Computes the absolute value of X.', url: `${BASE}/mathematical_functions#abs` },
    'SIGN':                          { description: 'Returns -1, 0, or +1 for negative, zero, and positive arguments respectively.', url: `${BASE}/mathematical_functions#sign` },
    'IS_INF':                        { description: 'Checks if X is positive or negative infinity.', url: `${BASE}/mathematical_functions#is_inf` },
    'IS_NAN':                        { description: 'Checks if X is a NaN value.', url: `${BASE}/mathematical_functions#is_nan` },
    'IEEE_DIVIDE':                   { description: 'Divides X by Y without generating errors for division by zero or overflow.', url: `${BASE}/mathematical_functions#ieee_divide` },
    'RAND':                          { description: 'Generates a pseudo-random FLOAT64 in [0, 1).', url: `${BASE}/mathematical_functions#rand` },
    'SQRT':                          { description: 'Computes the square root of X.', url: `${BASE}/mathematical_functions#sqrt` },
    'POW':                           { description: 'Returns X raised to the power of Y.', url: `${BASE}/mathematical_functions#pow` },
    'POWER':                         { description: 'Synonym of POW. Returns X raised to the power of Y.', url: `${BASE}/mathematical_functions#power` },
    'EXP':                           { description: 'Computes e to the power of X.', url: `${BASE}/mathematical_functions#exp` },
    'LN':                            { description: 'Computes the natural logarithm of X.', url: `${BASE}/mathematical_functions#ln` },
    'LOG':                           { description: 'Computes the natural logarithm of X, or log of X to base Y.', url: `${BASE}/mathematical_functions#log` },
    'LOG10':                         { description: 'Computes the logarithm of X to base 10.', url: `${BASE}/mathematical_functions#log10` },
    'GREATEST':                      { description: 'Gets the greatest value among X1,...,XN.', url: `${BASE}/mathematical_functions#greatest` },
    'LEAST':                         { description: 'Gets the least value among X1,...,XN.', url: `${BASE}/mathematical_functions#least` },
    'DIV':                           { description: 'Divides integer X by integer Y. Returns the integer quotient.', url: `${BASE}/mathematical_functions#div` },
    'SAFE_DIVIDE':                   { description: 'Equivalent to X / Y, but returns NULL if an error occurs.', url: `${BASE}/mathematical_functions#safe_divide` },
    'SAFE_MULTIPLY':                 { description: 'Equivalent to X * Y, but returns NULL if overflow occurs.', url: `${BASE}/mathematical_functions#safe_multiply` },
    'SAFE_NEGATE':                   { description: 'Equivalent to -X, but returns NULL if overflow occurs.', url: `${BASE}/mathematical_functions#safe_negate` },
    'SAFE_ADD':                      { description: 'Equivalent to X + Y, but returns NULL if overflow occurs.', url: `${BASE}/mathematical_functions#safe_add` },
    'SAFE_SUBTRACT':                 { description: 'Equivalent to X - Y, but returns NULL if overflow occurs.', url: `${BASE}/mathematical_functions#safe_subtract` },
    'MOD':                           { description: 'Gets the remainder of the division of X by Y.', url: `${BASE}/mathematical_functions#mod` },
    'ROUND':                         { description: 'Rounds X to the nearest integer, or to N decimal places.', url: `${BASE}/mathematical_functions#round` },
    'TRUNC':                         { description: 'Rounds X toward zero (truncates), never overflows.', url: `${BASE}/mathematical_functions#trunc` },
    'CEIL':                          { description: 'Gets the smallest integral value that is not less than X.', url: `${BASE}/mathematical_functions#ceil` },
    'CEILING':                       { description: 'Synonym of CEIL.', url: `${BASE}/mathematical_functions#ceiling` },
    'FLOOR':                         { description: 'Gets the largest integral value that is not greater than X.', url: `${BASE}/mathematical_functions#floor` },
    'CBRT':                          { description: 'Computes the cube root of X.', url: `${BASE}/mathematical_functions#cbrt` },
    'COS':                           { description: 'Computes the cosine of X.', url: `${BASE}/mathematical_functions#cos` },
    'COSH':                          { description: 'Computes the hyperbolic cosine of X.', url: `${BASE}/mathematical_functions#cosh` },
    'ACOS':                          { description: 'Computes the inverse cosine of X.', url: `${BASE}/mathematical_functions#acos` },
    'ACOSH':                         { description: 'Computes the inverse hyperbolic cosine of X.', url: `${BASE}/mathematical_functions#acosh` },
    'COT':                           { description: 'Computes the cotangent of X.', url: `${BASE}/mathematical_functions#cot` },
    'COTH':                          { description: 'Computes the hyperbolic cotangent of X.', url: `${BASE}/mathematical_functions#coth` },
    'CSC':                           { description: 'Computes the cosecant of X.', url: `${BASE}/mathematical_functions#csc` },
    'CSCH':                          { description: 'Computes the hyperbolic cosecant of X.', url: `${BASE}/mathematical_functions#csch` },
    'SEC':                           { description: 'Computes the secant of X.', url: `${BASE}/mathematical_functions#sec` },
    'SECH':                          { description: 'Computes the hyperbolic secant of X.', url: `${BASE}/mathematical_functions#sech` },
    'SIN':                           { description: 'Computes the sine of X.', url: `${BASE}/mathematical_functions#sin` },
    'SINH':                          { description: 'Computes the hyperbolic sine of X.', url: `${BASE}/mathematical_functions#sinh` },
    'ASIN':                          { description: 'Computes the inverse sine of X.', url: `${BASE}/mathematical_functions#asin` },
    'ASINH':                         { description: 'Computes the inverse hyperbolic sine of X.', url: `${BASE}/mathematical_functions#asinh` },
    'TAN':                           { description: 'Computes the tangent of X.', url: `${BASE}/mathematical_functions#tan` },
    'TANH':                          { description: 'Computes the hyperbolic tangent of X.', url: `${BASE}/mathematical_functions#tanh` },
    'ATAN':                          { description: 'Computes the inverse tangent of X.', url: `${BASE}/mathematical_functions#atan` },
    'ATANH':                         { description: 'Computes the inverse hyperbolic tangent of X.', url: `${BASE}/mathematical_functions#atanh` },
    'ATAN2':                         { description: 'Computes the inverse tangent of X/Y using signs to determine the quadrant.', url: `${BASE}/mathematical_functions#atan2` },
    // Range
    'RANGE_BUCKET':                  { description: 'Scans through a sorted array and returns the 0-based position of a point\'s upper bound.', url: `${BASE}/mathematical_functions#range_bucket` },
    'RANGE':                         { description: 'Constructs a range of DATE, DATETIME, or TIMESTAMP values.', url: `${BASE}/range_functions#range` },
    'RANGE_CONTAINS':                { description: 'Checks if one range contains another range, or if a value is in a range.', url: `${BASE}/range_functions#range_contains` },
    'RANGE_END':                     { description: 'Gets the upper bound of a range.', url: `${BASE}/range_functions#range_end` },
    'RANGE_INTERSECT':               { description: 'Gets the segment of two ranges that intersect.', url: `${BASE}/range_functions#range_intersect` },
    'RANGE_OVERLAPS':                { description: 'Checks if two ranges overlap.', url: `${BASE}/range_functions#range_overlaps` },
    'RANGE_SESSIONIZE':              { description: 'Produces a table of sessionized ranges.', url: `${BASE}/range_functions#range_sessionize` },
    'RANGE_START':                   { description: 'Gets the lower bound of a range.', url: `${BASE}/range_functions#range_start` },
    // Distance / ML
    'COSINE_DISTANCE':               { description: 'Computes the cosine distance between two vectors.', url: `${BASE}/distance_functions#cosine_distance` },
    'EUCLIDEAN_DISTANCE':            { description: 'Computes the Euclidean distance between two vectors.', url: `${BASE}/distance_functions#euclidean_distance` },
    'EDIT_DISTANCE':                 { description: 'Computes the Levenshtein distance between two STRING or BYTES values.', url: `${BASE}/string_functions#edit_distance` },
    // Hashing
    'FARM_FINGERPRINT':              { description: 'Computes the fingerprint of a STRING or BYTES value using the FarmHash Fingerprint64 algorithm.', url: `${BASE}/hash_functions#farm_fingerprint` },
    'MD5':                           { description: 'Computes the MD5 hash of a STRING or BYTES value. Returns BYTES.', url: `${BASE}/hash_functions#md5` },
    'SHA1':                          { description: 'Computes the SHA-1 hash of a STRING or BYTES value. Returns BYTES.', url: `${BASE}/hash_functions#sha1` },
    'SHA256':                        { description: 'Computes the SHA-256 hash of a STRING or BYTES value. Returns BYTES.', url: `${BASE}/hash_functions#sha256` },
    'SHA512':                        { description: 'Computes the SHA-512 hash of a STRING or BYTES value. Returns BYTES.', url: `${BASE}/hash_functions#sha512` },
    // String functions
    'ASCII':                         { description: 'Gets the ASCII code for the first character or byte in a STRING or BYTES value.', url: `${BASE}/string_functions#ascii` },
    'BYTE_LENGTH':                   { description: 'Gets the number of BYTES in a STRING or BYTES value.', url: `${BASE}/string_functions#byte_length` },
    'CHAR_LENGTH':                   { description: 'Gets the number of characters in a STRING value.', url: `${BASE}/string_functions#char_length` },
    'CHARACTER_LENGTH':              { description: 'Synonym for CHAR_LENGTH.', url: `${BASE}/string_functions#character_length` },
    'CHR':                           { description: 'Converts a Unicode code point to a character.', url: `${BASE}/string_functions#chr` },
    'CODE_POINTS_TO_BYTES':          { description: 'Converts an array of extended ASCII code points to a BYTES value.', url: `${BASE}/string_functions#code_points_to_bytes` },
    'CODE_POINTS_TO_STRING':         { description: 'Converts an array of extended ASCII code points to a STRING value.', url: `${BASE}/string_functions#code_points_to_string` },
    'COLLATE':                       { description: 'Combines a STRING and a collation spec into a collation-supported STRING.', url: `${BASE}/string_functions#collate` },
    'CONCAT':                        { description: 'Concatenates one or more STRING or BYTES values into a single result.', url: `${BASE}/string_functions#concat` },
    'CONTAINS_SUBSTR':               { description: 'Performs a normalized, case-insensitive search to see if a value exists as a substring.', url: `${BASE}/string_functions#contains_substr` },
    'ENDS_WITH':                     { description: 'Checks if a STRING or BYTES value is the suffix of another value.', url: `${BASE}/string_functions#ends_with` },
    'FORMAT':                        { description: 'Formats data and produces the results as a STRING value.', url: `${BASE}/string_functions#format` },
    'FROM_BASE32':                   { description: 'Converts a base32-encoded STRING into a BYTES value.', url: `${BASE}/string_functions#from_base32` },
    'FROM_BASE64':                   { description: 'Converts a base64-encoded STRING into a BYTES value.', url: `${BASE}/string_functions#from_base64` },
    'FROM_HEX':                      { description: 'Converts a hexadecimal-encoded STRING into a BYTES value.', url: `${BASE}/string_functions#from_hex` },
    'INITCAP':                       { description: 'Formats a STRING as proper case (first char of each word uppercase, rest lowercase).', url: `${BASE}/string_functions#initcap` },
    'INSTR':                         { description: 'Finds the position of a subvalue inside another value, with optional offset/occurrence.', url: `${BASE}/string_functions#instr` },
    'LEFT':                          { description: 'Gets the specified leftmost portion from a STRING or BYTES value.', url: `${BASE}/string_functions#left` },
    'LENGTH':                        { description: 'Gets the length of a STRING or BYTES value.', url: `${BASE}/string_functions#length` },
    'LPAD':                          { description: 'Prepends a STRING or BYTES value with a pattern.', url: `${BASE}/string_functions#lpad` },
    'LOWER':                         { description: 'Formats alphabetic characters in a STRING as lowercase.', url: `${BASE}/string_functions#lower` },
    'LTRIM':                         { description: 'Identical to TRIM but only removes leading characters.', url: `${BASE}/string_functions#ltrim` },
    'NORMALIZE':                     { description: 'Case-sensitively normalizes the characters in a STRING value.', url: `${BASE}/string_functions#normalize` },
    'NORMALIZE_AND_CASEFOLD':        { description: 'Case-insensitively normalizes the characters in a STRING value.', url: `${BASE}/string_functions#normalize_and_casefold` },
    'OCTET_LENGTH':                  { description: 'Alias for BYTE_LENGTH. Gets the number of bytes in a STRING or BYTES value.', url: `${BASE}/string_functions#octet_length` },
    'REGEXP_CONTAINS':               { description: 'Checks if a value is a partial match for a regular expression.', url: `${BASE}/string_functions#regexp_contains` },
    'REGEXP_EXTRACT':                { description: 'Produces a substring that matches a regular expression.', url: `${BASE}/string_functions#regexp_extract` },
    'REGEXP_EXTRACT_ALL':            { description: 'Produces an array of all substrings that match a regular expression.', url: `${BASE}/string_functions#regexp_extract_all` },
    'REGEXP_INSTR':                  { description: 'Finds the position of a regex match in a value, with optional offset/occurrence.', url: `${BASE}/string_functions#regexp_instr` },
    'REGEXP_REPLACE':                { description: 'Replaces all substrings matching a regex with a specified value.', url: `${BASE}/string_functions#regexp_replace` },
    'REGEXP_SUBSTR':                 { description: 'Synonym for REGEXP_EXTRACT. Produces a substring matching a regular expression.', url: `${BASE}/string_functions#regexp_substr` },
    'REPLACE':                       { description: 'Replaces all occurrences of a pattern with another pattern in a STRING or BYTES value.', url: `${BASE}/string_functions#replace` },
    'REPEAT':                        { description: 'Produces a STRING or BYTES value that consists of an original value, repeated.', url: `${BASE}/string_functions#repeat` },
    'REVERSE':                       { description: 'Reverses a STRING or BYTES value.', url: `${BASE}/string_functions#reverse` },
    'RIGHT':                         { description: 'Gets the specified rightmost portion from a STRING or BYTES value.', url: `${BASE}/string_functions#right` },
    'RPAD':                          { description: 'Appends a STRING or BYTES value with a pattern.', url: `${BASE}/string_functions#rpad` },
    'RTRIM':                         { description: 'Identical to TRIM but only removes trailing characters.', url: `${BASE}/string_functions#rtrim` },
    'SAFE_CONVERT_BYTES_TO_STRING':  { description: 'Converts BYTES to STRING, replacing invalid UTF-8 chars with U+FFFD.', url: `${BASE}/string_functions#safe_convert_bytes_to_string` },
    'SOUNDEX':                       { description: 'Gets the Soundex codes for words in a STRING value.', url: `${BASE}/string_functions#soundex` },
    'SPLIT':                         { description: 'Splits a STRING or BYTES value using a delimiter.', url: `${BASE}/string_functions#split` },
    'STARTS_WITH':                   { description: 'Checks if a STRING or BYTES value is a prefix of another value.', url: `${BASE}/string_functions#starts_with` },
    'STRPOS':                        { description: 'Finds the position of the first occurrence of a subvalue inside another value.', url: `${BASE}/string_functions#strpos` },
    'SUBSTR':                        { description: 'Gets a portion of a STRING or BYTES value.', url: `${BASE}/string_functions#substr` },
    'SUBSTRING':                     { description: 'Alias for SUBSTR. Gets a portion of a STRING or BYTES value.', url: `${BASE}/string_functions#substring` },
    'TO_BASE32':                     { description: 'Converts a BYTES value to a base32-encoded STRING value.', url: `${BASE}/string_functions#to_base32` },
    'TO_BASE64':                     { description: 'Converts a BYTES value to a base64-encoded STRING value.', url: `${BASE}/string_functions#to_base64` },
    'TO_CODE_POINTS':                { description: 'Converts a STRING or BYTES value into an ARRAY<INT64> of extended ASCII code points.', url: `${BASE}/string_functions#to_code_points` },
    'TO_HEX':                        { description: 'Converts a BYTES value to a hexadecimal STRING value.', url: `${BASE}/string_functions#to_hex` },
    'TRANSLATE':                     { description: 'Within a value, replaces each source character with the corresponding target character.', url: `${BASE}/string_functions#translate` },
    'TRIM':                          { description: 'Removes specified leading and trailing Unicode code points or bytes from a STRING or BYTES value.', url: `${BASE}/string_functions#trim` },
    'UNICODE':                       { description: 'Gets the Unicode code point for the first character in a value.', url: `${BASE}/string_functions#unicode` },
    'UPPER':                         { description: 'Formats alphabetic characters in a STRING value as uppercase.', url: `${BASE}/string_functions#upper` },
    // JSON
    'JSON_EXTRACT':                  { description: '(Deprecated) Extracts a JSON value and converts it to a SQL JSON-formatted STRING or JSON value. Use JSON_QUERY instead.', url: `${BASE}/json_functions#json_extract` },
    'JSON_QUERY':                    { description: 'Extracts a JSON value and converts it to a SQL JSON-formatted STRING or JSON value.', url: `${BASE}/json_functions#json_query` },
    'JSON_EXTRACT_SCALAR':           { description: '(Deprecated) Extracts a JSON scalar value and converts it to a SQL STRING value. Use JSON_VALUE instead.', url: `${BASE}/json_functions#json_extract_scalar` },
    'JSON_VALUE':                    { description: 'Extracts a JSON scalar value and converts it to a SQL STRING value.', url: `${BASE}/json_functions#json_value` },
    'JSON_EXTRACT_ARRAY':            { description: '(Deprecated) Extracts a JSON array and converts it to ARRAY<JSON-formatted STRING> or ARRAY<JSON>. Use JSON_QUERY_ARRAY instead.', url: `${BASE}/json_functions#json_extract_array` },
    'JSON_QUERY_ARRAY':              { description: 'Extracts a JSON array and converts it to a SQL ARRAY<JSON-formatted STRING> or ARRAY<JSON> value.', url: `${BASE}/json_functions#json_query_array` },
    'JSON_EXTRACT_STRING_ARRAY':     { description: '(Deprecated) Extracts a JSON array of scalar values and converts it to ARRAY<STRING>. Use JSON_VALUE_ARRAY instead.', url: `${BASE}/json_functions#json_extract_string_array` },
    'JSON_VALUE_ARRAY':              { description: 'Extracts a JSON array of scalar values and converts it to a SQL ARRAY<STRING> value.', url: `${BASE}/json_functions#json_value_array` },
    'JSON_ARRAY':                    { description: 'Creates a JSON array.', url: `${BASE}/json_functions#json_array` },
    'JSON_ARRAY_APPEND':             { description: 'Appends JSON data to the end of a JSON array.', url: `${BASE}/json_functions#json_array_append` },
    'JSON_ARRAY_INSERT':             { description: 'Inserts JSON data into a JSON array.', url: `${BASE}/json_functions#json_array_insert` },
    'JSON_FLATTEN':                  { description: 'Produces a new ARRAY<JSON> containing all non-array values from the input JSON, flattening consecutive nested arrays.', url: `${BASE}/json_functions#json_flatten` },
    'JSON_KEYS':                     { description: 'Extracts unique JSON keys from a JSON expression.', url: `${BASE}/json_functions#json_keys` },
    'JSON_OBJECT':                   { description: 'Creates a JSON object.', url: `${BASE}/json_functions#json_object` },
    'JSON_REMOVE':                   { description: 'Produces JSON with the specified JSON data removed.', url: `${BASE}/json_functions#json_remove` },
    'JSON_SET':                      { description: 'Inserts or replaces JSON data.', url: `${BASE}/json_functions#json_set` },
    'JSON_STRIP_NULLS':              { description: 'Removes JSON nulls from JSON objects and JSON arrays.', url: `${BASE}/json_functions#json_strip_nulls` },
    'JSON_TYPE':                     { description: 'Gets the JSON type of the outermost JSON value as a SQL STRING.', url: `${BASE}/json_functions#json_type` },
    'PARSE_JSON':                    { description: 'Converts a JSON-formatted STRING value to a JSON value.', url: `${BASE}/json_functions#parse_json` },
    'TO_JSON':                       { description: 'Converts a SQL value to a JSON value.', url: `${BASE}/json_functions#to_json` },
    'TO_JSON_STRING':                { description: 'Converts a SQL value to a JSON-formatted STRING value.', url: `${BASE}/json_functions#to_json_string` },
    // JSON type conversion
    'STRING':                        { description: 'Converts a JSON string to a SQL STRING value, or a TIMESTAMP to STRING.', url: `${BASE}/json_functions#string` },
    'BOOL':                          { description: 'Converts a JSON boolean to a SQL BOOL value.', url: `${BASE}/json_functions#bool` },
    'INT64':                         { description: 'Converts a JSON number to a SQL INT64 value.', url: `${BASE}/json_functions#int64` },
    'FLOAT64':                       { description: 'Converts a JSON number to a SQL FLOAT64 value.', url: `${BASE}/json_functions#float64` },
    'LAX_BOOL':                      { description: 'Attempts to convert a JSON value to a SQL BOOL value, returning NULL on failure.', url: `${BASE}/json_functions#lax_bool` },
    'LAX_FLOAT64':                   { description: 'Attempts to convert a JSON value to a SQL FLOAT64 value, returning NULL on failure.', url: `${BASE}/json_functions#lax_float64` },
    'LAX_INT64':                     { description: 'Attempts to convert a JSON value to a SQL INT64 value, returning NULL on failure.', url: `${BASE}/json_functions#lax_int64` },
    'LAX_STRING':                    { description: 'Attempts to convert a JSON value to a SQL STRING value, returning NULL on failure.', url: `${BASE}/json_functions#lax_string` },
    // Array
    'ARRAY':                         { description: 'Produces an array with one element for each row in a subquery.', url: `${BASE}/array_functions#array` },
    'ARRAY_CONCAT':                  { description: 'Concatenates one or more arrays with the same element type into a single array.', url: `${BASE}/array_functions#array_concat` },
    'ARRAY_LENGTH':                  { description: 'Gets the number of elements in an array.', url: `${BASE}/array_functions#array_length` },
    'ARRAY_TO_STRING':               { description: 'Produces a concatenation of the array elements as a STRING value.', url: `${BASE}/array_functions#array_to_string` },
    'GENERATE_ARRAY':                { description: 'Generates an array of values in a range.', url: `${BASE}/array_functions#generate_array` },
    'GENERATE_DATE_ARRAY':           { description: 'Generates an array of dates in a range.', url: `${BASE}/array_functions#generate_date_array` },
    'GENERATE_TIMESTAMP_ARRAY':      { description: 'Generates an array of timestamps in a range.', url: `${BASE}/array_functions#generate_timestamp_array` },
    'GENERATE_RANGE_ARRAY':          { description: 'Splits a range into an array of subranges.', url: `${BASE}/range_functions#generate_range_array` },
    'ARRAY_REVERSE':                 { description: 'Reverses the order of elements in an array.', url: `${BASE}/array_functions#array_reverse` },
    'ARRAY_FIRST':                   { description: 'Gets the first element in an array.', url: `${BASE}/array_functions#array_first` },
    'ARRAY_LAST':                    { description: 'Gets the last element in an array.', url: `${BASE}/array_functions#array_last` },
    'ARRAY_SLICE':                   { description: 'Produces an array containing zero or more consecutive elements from an input array.', url: `${BASE}/array_functions#array_slice` },
    // Date
    'CURRENT_DATE':                  { description: 'Returns the current date as a DATE value.', url: `${BASE}/date_functions#current_date` },
    'EXTRACT':                       { description: 'Extracts a date/time part from a DATE, DATETIME, TIME, TIMESTAMP, or INTERVAL value.', url: `${BASE}/date_functions#extract` },
    'DATE':                          { description: 'Constructs a DATE value.', url: `${BASE}/date_functions#date` },
    'DATE_ADD':                      { description: 'Adds a specified time interval to a DATE value.', url: `${BASE}/date_functions#date_add` },
    'DATE_BUCKET':                   { description: 'Gets the lower bound of the date bucket that contains a date.', url: `${BASE}/date_functions#date_bucket` },
    'DATE_SUB':                      { description: 'Subtracts a specified time interval from a DATE value.', url: `${BASE}/date_functions#date_sub` },
    'DATE_DIFF':                     { description: 'Gets the number of unit boundaries between two DATE values at a particular granularity.', url: `${BASE}/date_functions#date_diff` },
    'DATE_TRUNC':                    { description: 'Truncates a DATE, DATETIME, or TIMESTAMP value at a particular granularity.', url: `${BASE}/date_functions#date_trunc` },
    'DATE_FROM_UNIX_DATE':           { description: 'Interprets an INT64 as the number of days since 1970-01-01.', url: `${BASE}/date_functions#date_from_unix_date` },
    'FORMAT_DATE':                   { description: 'Formats a DATE value according to a specified format string.', url: `${BASE}/date_functions#format_date` },
    'LAST_DAY':                      { description: 'Gets the last day in a specified time period that contains a DATE or DATETIME value.', url: `${BASE}/date_functions#last_day` },
    'PARSE_DATE':                    { description: 'Converts a STRING value to a DATE value.', url: `${BASE}/date_functions#parse_date` },
    'UNIX_DATE':                     { description: 'Converts a DATE value to the number of days since 1970-01-01.', url: `${BASE}/date_functions#unix_date` },
    // Datetime
    'CURRENT_DATETIME':              { description: 'Returns the current date and time as a DATETIME value.', url: `${BASE}/datetime_functions#current_datetime` },
    'DATETIME':                      { description: 'Constructs a DATETIME value.', url: `${BASE}/datetime_functions#datetime` },
    'DATETIME_ADD':                  { description: 'Adds a specified time interval to a DATETIME value.', url: `${BASE}/datetime_functions#datetime_add` },
    'DATETIME_BUCKET':               { description: 'Gets the lower bound of the datetime bucket that contains a datetime.', url: `${BASE}/datetime_functions#datetime_bucket` },
    'DATETIME_SUB':                  { description: 'Subtracts a specified time interval from a DATETIME value.', url: `${BASE}/datetime_functions#datetime_sub` },
    'DATETIME_DIFF':                 { description: 'Gets the number of unit boundaries between two DATETIME values at a particular granularity.', url: `${BASE}/datetime_functions#datetime_diff` },
    'DATETIME_TRUNC':                { description: 'Truncates a DATETIME or TIMESTAMP value at a particular granularity.', url: `${BASE}/datetime_functions#datetime_trunc` },
    'FORMAT_DATETIME':               { description: 'Formats a DATETIME value according to a specified format string.', url: `${BASE}/datetime_functions#format_datetime` },
    'PARSE_DATETIME':                { description: 'Converts a STRING value to a DATETIME value.', url: `${BASE}/datetime_functions#parse_datetime` },
    // Time
    'CURRENT_TIME':                  { description: 'Returns the current time as a TIME value.', url: `${BASE}/time_functions#current_time` },
    'TIME':                          { description: 'Constructs a TIME value.', url: `${BASE}/time_functions#time` },
    'TIME_ADD':                      { description: 'Adds a specified time interval to a TIME value.', url: `${BASE}/time_functions#time_add` },
    'TIME_SUB':                      { description: 'Subtracts a specified time interval from a TIME value.', url: `${BASE}/time_functions#time_sub` },
    'TIME_DIFF':                     { description: 'Gets the number of unit boundaries between two TIME values at a particular granularity.', url: `${BASE}/time_functions#time_diff` },
    'TIME_TRUNC':                    { description: 'Truncates a TIME value at a particular granularity.', url: `${BASE}/time_functions#time_trunc` },
    'FORMAT_TIME':                   { description: 'Formats a TIME value according to the specified format string.', url: `${BASE}/time_functions#format_time` },
    'PARSE_TIME':                    { description: 'Converts a STRING value to a TIME value.', url: `${BASE}/time_functions#parse_time` },
    // Timestamp
    'CURRENT_TIMESTAMP':             { description: 'Returns the current date and time as a TIMESTAMP value.', url: `${BASE}/timestamp_functions#current_timestamp` },
    'TIMESTAMP':                     { description: 'Constructs a TIMESTAMP value.', url: `${BASE}/timestamp_functions#timestamp` },
    'TIMESTAMP_ADD':                 { description: 'Adds a specified time interval to a TIMESTAMP value.', url: `${BASE}/timestamp_functions#timestamp_add` },
    'TIMESTAMP_BUCKET':              { description: 'Gets the lower bound of the timestamp bucket that contains a timestamp.', url: `${BASE}/timestamp_functions#timestamp_bucket` },
    'TIMESTAMP_SUB':                 { description: 'Subtracts a specified time interval from a TIMESTAMP value.', url: `${BASE}/timestamp_functions#timestamp_sub` },
    'TIMESTAMP_DIFF':                { description: 'Gets the number of unit boundaries between two TIMESTAMP values at a particular granularity.', url: `${BASE}/timestamp_functions#timestamp_diff` },
    'TIMESTAMP_TRUNC':               { description: 'Truncates a TIMESTAMP or DATETIME value at a particular granularity.', url: `${BASE}/timestamp_functions#timestamp_trunc` },
    'FORMAT_TIMESTAMP':              { description: 'Formats a TIMESTAMP value according to the specified format string.', url: `${BASE}/timestamp_functions#format_timestamp` },
    'PARSE_TIMESTAMP':               { description: 'Converts a STRING value to a TIMESTAMP value.', url: `${BASE}/timestamp_functions#parse_timestamp` },
    'TIMESTAMP_SECONDS':             { description: 'Converts the number of seconds since 1970-01-01 00:00:00 UTC to a TIMESTAMP.', url: `${BASE}/timestamp_functions#timestamp_seconds` },
    'TIMESTAMP_MILLIS':              { description: 'Converts the number of milliseconds since 1970-01-01 00:00:00 UTC to a TIMESTAMP.', url: `${BASE}/timestamp_functions#timestamp_millis` },
    'TIMESTAMP_MICROS':              { description: 'Converts the number of microseconds since 1970-01-01 00:00:00 UTC to a TIMESTAMP.', url: `${BASE}/timestamp_functions#timestamp_micros` },
    'UNIX_SECONDS':                  { description: 'Converts a TIMESTAMP to the number of seconds since 1970-01-01 00:00:00 UTC.', url: `${BASE}/timestamp_functions#unix_seconds` },
    'UNIX_MILLIS':                   { description: 'Converts a TIMESTAMP to the number of milliseconds since 1970-01-01 00:00:00 UTC.', url: `${BASE}/timestamp_functions#unix_millis` },
    'UNIX_MICROS':                   { description: 'Converts a TIMESTAMP to the number of microseconds since 1970-01-01 00:00:00 UTC.', url: `${BASE}/timestamp_functions#unix_micros` },
    // Interval
    'MAKE_INTERVAL':                 { description: 'Constructs an INTERVAL value.', url: `${BASE}/interval_functions#make_interval` },
    'JUSTIFY_DAYS':                  { description: 'Normalizes the day part of an INTERVAL value.', url: `${BASE}/interval_functions#justify_days` },
    'JUSTIFY_HOURS':                 { description: 'Normalizes the time part of an INTERVAL value.', url: `${BASE}/interval_functions#justify_hours` },
    'JUSTIFY_INTERVAL':              { description: 'Normalizes the day and time parts of an INTERVAL value.', url: `${BASE}/interval_functions#justify_interval` },
    // Geography
    'S2_CELLIDFROMPOINT':            { description: 'Gets the S2 cell ID covering a point GEOGRAPHY value.', url: `${BASE}/geography_functions#s2_cellidfrompoint` },
    'S2_COVERINGCELLIDS':            { description: 'Gets an array of S2 cell IDs that cover a GEOGRAPHY value.', url: `${BASE}/geography_functions#s2_coveringcellids` },
    'ST_ANGLE':                      { description: 'Takes three point GEOGRAPHYs representing two lines and returns the angle between them.', url: `${BASE}/geography_functions#st_angle` },
    'ST_AREA':                       { description: 'Gets the area covered by the polygons in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_area` },
    'ST_ASBINARY':                   { description: 'Converts a GEOGRAPHY value to a BYTES WKB geography value.', url: `${BASE}/geography_functions#st_asbinary` },
    'ST_ASGEOJSON':                  { description: 'Converts a GEOGRAPHY value to a STRING GeoJSON geography value.', url: `${BASE}/geography_functions#st_asgeojson` },
    'ST_ASTEXT':                     { description: 'Converts a GEOGRAPHY value to a STRING WKT geography value.', url: `${BASE}/geography_functions#st_astext` },
    'ST_AZIMUTH':                    { description: 'Gets the azimuth of a line segment formed by two point GEOGRAPHY values.', url: `${BASE}/geography_functions#st_azimuth` },
    'ST_BOUNDARY':                   { description: 'Gets the union of component boundaries in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_boundary` },
    'ST_BOUNDINGBOX':                { description: 'Gets the bounding box for a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_boundingbox` },
    'ST_BUFFER':                     { description: 'Gets the buffer around a GEOGRAPHY value using a specific number of segments.', url: `${BASE}/geography_functions#st_buffer` },
    'ST_BUFFERWITHTOLERANCE':        { description: 'Gets the buffer around a GEOGRAPHY value using tolerance.', url: `${BASE}/geography_functions#st_bufferwithtolerance` },
    'ST_CENTROID':                   { description: 'Gets the centroid of a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_centroid` },
    'ST_CENTROID_AGG':               { description: 'Gets the centroid of a set of GEOGRAPHY values.', url: `${BASE}/geography_functions#st_centroid_agg` },
    'ST_CLOSESTPOINT':               { description: 'Gets the point on the first GEOGRAPHY closest to any point in the second.', url: `${BASE}/geography_functions#st_closestpoint` },
    'ST_CLUSTERDBSCAN':              { description: 'Performs DBSCAN clustering on GEOGRAPHY values and returns a 0-based cluster number.', url: `${BASE}/geography_functions#st_clusterdbscan` },
    'ST_CONTAINS':                   { description: 'Checks if one GEOGRAPHY value contains another.', url: `${BASE}/geography_functions#st_contains` },
    'ST_CONVEXHULL':                 { description: 'Returns the convex hull for a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_convexhull` },
    'ST_COVEREDBY':                  { description: 'Checks if all points of the first GEOGRAPHY are on the boundary or interior of the second.', url: `${BASE}/geography_functions#st_coveredby` },
    'ST_COVERS':                     { description: 'Checks if all points of the second GEOGRAPHY are on the boundary or interior of the first.', url: `${BASE}/geography_functions#st_covers` },
    'ST_DIFFERENCE':                 { description: 'Gets the point set difference between two GEOGRAPHY values.', url: `${BASE}/geography_functions#st_difference` },
    'ST_DIMENSION':                  { description: 'Gets the dimension of the highest-dimensional element in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_dimension` },
    'ST_DISJOINT':                   { description: 'Checks if two GEOGRAPHY values are disjoint (don\'t intersect).', url: `${BASE}/geography_functions#st_disjoint` },
    'ST_DISTANCE':                   { description: 'Gets the shortest distance in meters between two GEOGRAPHY values.', url: `${BASE}/geography_functions#st_distance` },
    'ST_DUMP':                       { description: 'Returns an array of simple GEOGRAPHY components in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_dump` },
    'ST_DWITHIN':                    { description: 'Checks if any points in two GEOGRAPHY values are within a given distance.', url: `${BASE}/geography_functions#st_dwithin` },
    'ST_ENDPOINT':                   { description: 'Gets the last point of a linestring GEOGRAPHY value.', url: `${BASE}/geography_functions#st_endpoint` },
    'ST_EQUALS':                     { description: 'Checks if two GEOGRAPHY values represent the same geography.', url: `${BASE}/geography_functions#st_equals` },
    'ST_EXTENT':                     { description: 'Gets the bounding box for a group of GEOGRAPHY values.', url: `${BASE}/geography_functions#st_extent` },
    'ST_EXTERIORRING':               { description: 'Returns a linestring GEOGRAPHY corresponding to the outermost ring of a polygon.', url: `${BASE}/geography_functions#st_exteriorring` },
    'ST_GEOGFROM':                   { description: 'Converts a STRING or BYTES value into a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_geogfrom` },
    'ST_GEOGFROMGEOJSON':            { description: 'Converts a STRING GeoJSON geometry value into a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_geogfromgeojson` },
    'ST_GEOGFROMTEXT':               { description: 'Converts a STRING WKT geometry value into a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_geogfromtext` },
    'ST_GEOGFROMWKB':                { description: 'Converts a BYTES or hex-text STRING WKB geometry value into a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_geogfromwkb` },
    'ST_GEOGPOINT':                  { description: 'Creates a point GEOGRAPHY value for a given longitude and latitude.', url: `${BASE}/geography_functions#st_geogpoint` },
    'ST_GEOGPOINTFROMGEOHASH':       { description: 'Gets a point GEOGRAPHY in the middle of a bounding box defined by a GeoHash STRING.', url: `${BASE}/geography_functions#st_geogpointfromgeohash` },
    'ST_GEOHASH':                    { description: 'Converts a point GEOGRAPHY value to a STRING GeoHash value.', url: `${BASE}/geography_functions#st_geohash` },
    'ST_GEOMETRYTYPE':               { description: 'Gets the OGC geometry type for a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_geometrytype` },
    'ST_HAUSDORFFDISTANCE':          { description: 'Gets the discrete Hausdorff distance between two geometries.', url: `${BASE}/geography_functions#st_hausdorffdistance` },
    'ST_HAUSDORFFDWITHIN':           { description: 'Checks if the Hausdorff distance between two GEOGRAPHY values is within a given distance.', url: `${BASE}/geography_functions#st_hausdorffdwithin` },
    'ST_INTERIORRINGS':              { description: 'Gets the interior rings of a polygon GEOGRAPHY value.', url: `${BASE}/geography_functions#st_interiorrings` },
    'ST_INTERSECTION':               { description: 'Gets the point set intersection of two GEOGRAPHY values.', url: `${BASE}/geography_functions#st_intersection` },
    'ST_INTERSECTS':                 { description: 'Checks if at least one point appears in two GEOGRAPHY values.', url: `${BASE}/geography_functions#st_intersects` },
    'ST_INTERSECTSBOX':              { description: 'Checks if a GEOGRAPHY value intersects a rectangle.', url: `${BASE}/geography_functions#st_intersectsbox` },
    'ST_ISCLOSED':                   { description: 'Checks if all components in a GEOGRAPHY value are closed.', url: `${BASE}/geography_functions#st_isclosed` },
    'ST_ISCOLLECTION':               { description: 'Checks if the total number of points, linestrings, and polygons is greater than one.', url: `${BASE}/geography_functions#st_iscollection` },
    'ST_ISEMPTY':                    { description: 'Checks if a GEOGRAPHY value is empty.', url: `${BASE}/geography_functions#st_isempty` },
    'ST_ISRING':                     { description: 'Checks if a GEOGRAPHY value is a closed, simple linestring.', url: `${BASE}/geography_functions#st_isring` },
    'ST_LENGTH':                     { description: 'Gets the total length of lines in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_length` },
    'ST_LINEINTERPOLATEPOINT':       { description: 'Gets a point at a specific fraction in a linestring GEOGRAPHY value.', url: `${BASE}/geography_functions#st_lineinterpolatepoint` },
    'ST_LINELOCATEPOINT':            { description: 'Gets the fraction along a linestring closest to a given point GEOGRAPHY value.', url: `${BASE}/geography_functions#st_linelocatepoint` },
    'ST_LINESUBSTRING':              { description: 'Gets a segment of a single linestring at a specific starting and ending fraction.', url: `${BASE}/geography_functions#st_linesubstring` },
    'ST_MAKELINE':                   { description: 'Creates a linestring GEOGRAPHY by concatenating point and linestring vertices.', url: `${BASE}/geography_functions#st_makeline` },
    'ST_MAKEPOLYGON':                { description: 'Constructs a polygon GEOGRAPHY by combining a shell with polygon holes.', url: `${BASE}/geography_functions#st_makepolygon` },
    'ST_MAKEPOLYGONORIENTED':        { description: 'Constructs a polygon GEOGRAPHY using an array of linestring GEOGRAPHYs; vertex ordering determines orientation.', url: `${BASE}/geography_functions#st_makepolygonoriented` },
    'ST_MAXDISTANCE':                { description: 'Gets the longest distance between two non-empty GEOGRAPHY values.', url: `${BASE}/geography_functions#st_maxdistance` },
    'ST_NPOINTS':                    { description: 'An alias of ST_NUMPOINTS. Gets the number of vertices in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_npoints` },
    'ST_NUMGEOMETRIES':              { description: 'Gets the number of geometries in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_numgeometries` },
    'ST_NUMPOINTS':                  { description: 'Gets the number of vertices in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_numpoints` },
    'ST_PERIMETER':                  { description: 'Gets the length of the boundary of the polygons in a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_perimeter` },
    'ST_POINTN':                     { description: 'Gets the point at a specific index of a linestring GEOGRAPHY value.', url: `${BASE}/geography_functions#st_pointn` },
    'ST_REGIONSTATS':                { description: 'Computes statistics describing the pixels in a geospatial raster image that intersect a GEOGRAPHY value.', url: `${BASE}/geography_functions#st_regionstats` },
    'ST_SIMPLIFY':                   { description: 'Converts a GEOGRAPHY value into a simplified GEOGRAPHY value using tolerance.', url: `${BASE}/geography_functions#st_simplify` },
    'ST_SNAPTOGRID':                 { description: 'Produces a GEOGRAPHY where each vertex has been snapped to a longitude/latitude grid.', url: `${BASE}/geography_functions#st_snaptogrid` },
    'ST_STARTPOINT':                 { description: 'Gets the first point of a linestring GEOGRAPHY value.', url: `${BASE}/geography_functions#st_startpoint` },
    'ST_TOUCHES':                    { description: 'Checks if two GEOGRAPHY values intersect and their interiors have no elements in common.', url: `${BASE}/geography_functions#st_touches` },
    'ST_UNION':                      { description: 'Gets the point set union of multiple GEOGRAPHY values.', url: `${BASE}/geography_functions#st_union` },
    'ST_UNION_AGG':                  { description: 'Aggregates over GEOGRAPHY values and gets their point set union.', url: `${BASE}/geography_functions#st_union_agg` },
    'ST_WITHIN':                     { description: 'Checks if one GEOGRAPHY value is contained within another.', url: `${BASE}/geography_functions#st_within` },
    'ST_X':                          { description: 'Gets the longitude from a point GEOGRAPHY value.', url: `${BASE}/geography_functions#st_x` },
    'ST_Y':                          { description: 'Gets the latitude from a point GEOGRAPHY value.', url: `${BASE}/geography_functions#st_y` },
    // Utility / security
    'SESSION_USER':                  { description: 'Returns the email address or principal identifier of the user running the query.', url: `${BASE}/utility_functions#session_user` },
    'GENERATE_UUID':                 { description: 'Produces a random universally unique identifier (UUID) as a STRING value.', url: `${BASE}/utility_functions#generate_uuid` },
    'ERROR':                         { description: 'Produces an error with a custom error message.', url: `${BASE}/utility_functions#error` },
    'SEARCH':                        { description: 'Checks whether a table or other search data contains a set of search terms.', url: `${BASE}/search_functions#search` },
    'GAP_FILL':                      { description: 'Finds and fills gaps in a time series.', url: `${BASE}/time_series_functions#gap_fill` },
    // NET
    'NET.IP_FROM_STRING':            { description: 'Converts an IPv4 or IPv6 address from a STRING to a BYTES value in network byte order.', url: `${BASE}/net_functions#netip_from_string` },
    'NET.SAFE_IP_FROM_STRING':       { description: 'Like NET.IP_FROM_STRING but returns NULL instead of an error for invalid input.', url: `${BASE}/net_functions#netsafe_ip_from_string` },
    'NET.IP_TO_STRING':              { description: 'Converts an IPv4 or IPv6 address from network byte order BYTES to a STRING value.', url: `${BASE}/net_functions#netip_to_string` },
    'NET.IP_NET_MASK':               { description: 'Gets a network mask.', url: `${BASE}/net_functions#netip_net_mask` },
    'NET.IP_TRUNC':                  { description: 'Converts a BYTES IPv4/IPv6 address in network byte order to a BYTES subnet address.', url: `${BASE}/net_functions#netip_trunc` },
    'NET.IPV4_FROM_INT64':           { description: 'Converts an IPv4 address from an INT64 value to BYTES in network byte order.', url: `${BASE}/net_functions#netipv4_from_int64` },
    'NET.IPV4_TO_INT64':             { description: 'Converts an IPv4 address from BYTES in network byte order to an INT64 value.', url: `${BASE}/net_functions#netipv4_to_int64` },
    'NET.HOST':                      { description: 'Gets the hostname from a URL.', url: `${BASE}/net_functions#nethost` },
    'NET.PUBLIC_SUFFIX':             { description: 'Gets the public suffix from a URL.', url: `${BASE}/net_functions#netpublic_suffix` },
    'NET.REG_DOMAIN':                { description: 'Gets the registered or registrable domain from a URL.', url: `${BASE}/net_functions#netreg_domain` },
    // Encryption / keys
    'KEYS.NEW_KEYSET':               { description: 'Gets a serialized keyset containing a new key based on the key type.', url: `${BASE}/aead_encryption_functions#keysnew_keyset` },
    'KEYS.NEW_WRAPPED_KEYSET':       { description: 'Creates a new keyset and encrypts it with a Cloud KMS key.', url: `${BASE}/aead_encryption_functions#keysnew_wrapped_keyset` },
    'KEYS.REWRAP_KEYSET':            { description: 'Re-encrypts a wrapped keyset with a new Cloud KMS key.', url: `${BASE}/aead_encryption_functions#keysrewrap_keyset` },
    'KEYS.ADD_KEY_FROM_RAW_BYTES':   { description: 'Adds a key to a keyset and returns the new keyset as serialized BYTES.', url: `${BASE}/aead_encryption_functions#keysadd_key_from_raw_bytes` },
    'KEYS.KEYSET_CHAIN':             { description: 'Produces a Tink keyset that\'s encrypted with a Cloud KMS key.', url: `${BASE}/aead_encryption_functions#keyskeyset_chain` },
    'KEYS.KEYSET_FROM_JSON':         { description: 'Converts a STRING JSON keyset to a serialized BYTES value.', url: `${BASE}/aead_encryption_functions#keyskeyset_from_json` },
    'KEYS.KEYSET_TO_JSON':           { description: 'Gets a JSON STRING representation of a keyset.', url: `${BASE}/aead_encryption_functions#keyskeyset_to_json` },
    'KEYS.ROTATE_KEYSET':            { description: 'Adds a new primary cryptographic key to a keyset, based on the key type.', url: `${BASE}/aead_encryption_functions#keysrotate_keyset` },
    'KEYS.ROTATE_WRAPPED_KEYSET':    { description: 'Rewraps a keyset and rotates it.', url: `${BASE}/aead_encryption_functions#keysrotate_wrapped_keyset` },
    'KEYS.KEYSET_LENGTH':            { description: 'Gets the number of keys in the provided keyset.', url: `${BASE}/aead_encryption_functions#keyskeyset_length` },
    'AEAD.DECRYPT_BYTES':            { description: 'Uses the matching key from a keyset to decrypt a BYTES ciphertext.', url: `${BASE}/aead_encryption_functions#aeaddecrypt_bytes` },
    'AEAD.DECRYPT_STRING':           { description: 'Uses the matching key from a keyset to decrypt a BYTES ciphertext into a STRING plaintext.', url: `${BASE}/aead_encryption_functions#aeaddecrypt_string` },
    'AEAD.ENCRYPT':                  { description: 'Encrypts STRING plaintext using the primary cryptographic key in a keyset.', url: `${BASE}/aead_encryption_functions#aead_encrypt` },
    'DETERMINISTIC_DECRYPT_BYTES':   { description: 'Uses the matching key from a keyset to decrypt a BYTES ciphertext, using deterministic AEAD.', url: `${BASE}/aead_encryption_functions#deterministic_decrypt_bytes` },
    'DETERMINISTIC_DECRYPT_STRING':  { description: 'Uses the matching key from a keyset to decrypt a BYTES ciphertext into a STRING plaintext, using deterministic AEAD.', url: `${BASE}/aead_encryption_functions#deterministic_decrypt_string` },
    'DETERMINISTIC_ENCRYPT':         { description: 'Encrypts STRING plaintext using the primary key in a keyset, with deterministic AEAD encryption.', url: `${BASE}/aead_encryption_functions#deterministic_encrypt` },
    // External / DLP / Object
    'EXTERNAL_OBJECT_TRANSFORM':     { description: 'Produces an object table with the original columns plus one or more additional transform columns.', url: `${BASE}/utility_functions#external_object_transform` },
    'EXTERNAL_QUERY':                { description: 'Executes a query on an external database and returns the results as a temporary table.', url: `${BASE}/federated_query_functions#external_query` },
    'APPENDS':                       { description: 'Returns all rows appended to a table for a given time range.', url: `${BASE}/utility_functions#appends` },
    'CHANGES':                       { description: 'Returns all rows that have changed in a table for a given time range.', url: `${BASE}/utility_functions#changes` },
    'OBJ.FETCH_METADATA':            { description: 'Fetches Cloud Storage metadata for a partially populated ObjectRef value.', url: `${BASE}/utility_functions#objfetch_metadata` },
    'OBJ.GET_ACCESS_URL':            { description: 'Returns access URLs for a Cloud Storage object.', url: `${BASE}/utility_functions#objget_access_url` },
    'OBJ.MAKE_REF':                  { description: 'Creates an ObjectRef value that contains reference information for a Cloud Storage object.', url: `${BASE}/utility_functions#objmake_ref` },
    // DLP
    'DLP_DETERMINISTIC_ENCRYPT':     { description: 'Encrypts data with a DLP-compatible algorithm.', url: `${BASE}/dlp_functions#dlp_deterministic_encrypt` },
    'DLP_DETERMINISTIC_DECRYPT':     { description: 'Decrypts DLP-encrypted data.', url: `${BASE}/dlp_functions#dlp_deterministic_decrypt` },
    'DLP_KEY_CHAIN':                 { description: 'Gets a data encryption key that\'s wrapped by Cloud Key Management Service.', url: `${BASE}/dlp_functions#dlp_key_chain` },
    // Search / text / ML
    'TEXT_ANALYZE':                  { description: 'Extracts terms (tokens) from text and converts them into a tokenized document.', url: `${BASE}/search_functions#text_analyze` },
    'BAG_OF_WORDS':                  { description: 'Gets the frequency of each term (token) in a tokenized document.', url: `${BASE}/search_functions#bag_of_words` },
    'TF_IDF':                        { description: 'Evaluates how relevant a term (token) is to a tokenized document in a set of tokenized documents.', url: `${BASE}/search_functions#tf_idf` },
    'VECTOR_SEARCH':                 { description: 'Performs a vector search on embeddings to find semantically similar entities.', url: `${BASE}/vector_search_functions#vector_search` },
    'VECTOR_INDEX.STATISTICS':       { description: 'Calculates how much an indexed table\'s data has drifted since the vector index was trained.', url: `${BASE}/vector_search_functions#vector_indexstatistics` },
};

// ---------------------------------------------------------------------------
// Function snippet signatures
// Each entry maps a function name to the args portion (from opening paren).
// Tab stops use ${N:name} for positional args and ${N|a,b,c|} for choices.
// ---------------------------------------------------------------------------

const FUNCTION_SNIPPETS: Record<string, string> = {
    // Conditional
    'COALESCE':                 '(${1:expression1}, ${2:expression2})',
    'IF':                       '(${1:condition}, ${2:true_result}, ${3:false_result})',
    'IFNULL':                   '(${1:expression}, ${2:null_result})',
    'NULLIF':                   '(${1:expression}, ${2:expression_to_match})',
    'IIF':                      '(${1:condition}, ${2:true_result}, ${3:false_result})',
    // Aggregate
    'COUNT':                    '(${1:expression})',
    'COUNTIF':                  '(${1:condition})',
    'SUM':                      '(${1:expression})',
    'AVG':                      '(${1:expression})',
    'MIN':                      '(${1:expression})',
    'MAX':                      '(${1:expression})',
    'MIN_BY':                   '(${1:value_expression}, ${2:order_expression})',
    'MAX_BY':                   '(${1:value_expression}, ${2:order_expression})',
    'STRING_AGG':               '(${1:expression}, ${2:\'delimiter\'})',
    'ARRAY_AGG':                '(${1:expression})',
    'ANY_VALUE':                '(${1:expression})',
    'APPROX_COUNT_DISTINCT':    '(${1:expression})',
    'APPROX_QUANTILES':         '(${1:expression}, ${2:number})',
    'APPROX_TOP_COUNT':         '(${1:expression}, ${2:number})',
    'APPROX_TOP_SUM':           '(${1:value}, ${2:weight}, ${3:number})',
    // Casting
    'CAST':                     '(${1:expression} AS ${2:type})',
    'SAFE_CAST':                '(${1:expression} AS ${2:type})',
    // Math
    'ABS':                      '(${1:numeric_expression})',
    'SIGN':                     '(${1:numeric_expression})',
    'ROUND':                    '(${1:numeric_expression}, ${2:decimal_places})',
    'FLOOR':                    '(${1:numeric_expression})',
    'CEIL':                     '(${1:numeric_expression})',
    'CEILING':                  '(${1:numeric_expression})',
    'TRUNC':                    '(${1:numeric_expression})',
    'MOD':                      '(${1:dividend}, ${2:divisor})',
    'DIV':                      '(${1:dividend}, ${2:divisor})',
    'POW':                      '(${1:base}, ${2:exponent})',
    'POWER':                    '(${1:base}, ${2:exponent})',
    'SQRT':                     '(${1:numeric_expression})',
    'CBRT':                     '(${1:numeric_expression})',
    'LOG':                      '(${1:numeric_expression})',
    'LOG10':                    '(${1:numeric_expression})',
    'LN':                       '(${1:numeric_expression})',
    'EXP':                      '(${1:numeric_expression})',
    'GREATEST':                 '(${1:value1}, ${2:value2})',
    'LEAST':                    '(${1:value1}, ${2:value2})',
    'SAFE_DIVIDE':              '(${1:numerator}, ${2:denominator})',
    'SAFE_ADD':                 '(${1:x}, ${2:y})',
    'SAFE_SUBTRACT':            '(${1:x}, ${2:y})',
    'SAFE_MULTIPLY':            '(${1:x}, ${2:y})',
    'SAFE_NEGATE':              '(${1:numeric_expression})',
    'IEEE_DIVIDE':              '(${1:x}, ${2:y})',
    'RANGE_BUCKET':             '(${1:point}, ${2:boundaries_array})',
    // String
    'CONCAT':                   '(${1:value1}, ${2:value2})',
    'SUBSTR':                   '(${1:value}, ${2:position}, ${3:length})',
    'SUBSTRING':                '(${1:value}, ${2:position}, ${3:length})',
    'REPLACE':                  '(${1:original_value}, ${2:from_value}, ${3:to_value})',
    'REGEXP_REPLACE':           '(${1:value}, ${2:r\'regexp\'}, ${3:replacement})',
    'REGEXP_EXTRACT':           '(${1:value}, ${2:r\'regexp\'})',
    'REGEXP_EXTRACT_ALL':       '(${1:value}, ${2:r\'regexp\'})',
    'REGEXP_CONTAINS':          '(${1:value}, ${2:r\'regexp\'})',
    'REGEXP_INSTR':             '(${1:value}, ${2:r\'regexp\'})',
    'REGEXP_SUBSTR':            '(${1:value}, ${2:r\'regexp\'})',
    'SPLIT':                    '(${1:value}, ${2:delimiter})',
    'TRIM':                     '(${1:value})',
    'LTRIM':                    '(${1:value})',
    'RTRIM':                    '(${1:value})',
    'UPPER':                    '(${1:value})',
    'LOWER':                    '(${1:value})',
    'LENGTH':                   '(${1:value})',
    'CHAR_LENGTH':              '(${1:value})',
    'CHARACTER_LENGTH':         '(${1:value})',
    'BYTE_LENGTH':              '(${1:value})',
    'OCTET_LENGTH':             '(${1:value})',
    'LEFT':                     '(${1:value}, ${2:length})',
    'RIGHT':                    '(${1:value}, ${2:length})',
    'LPAD':                     '(${1:value}, ${2:return_length}, ${3:pattern})',
    'RPAD':                     '(${1:value}, ${2:return_length}, ${3:pattern})',
    'REPEAT':                   '(${1:value}, ${2:repetitions})',
    'REVERSE':                  '(${1:value})',
    'STARTS_WITH':              '(${1:value}, ${2:prefix})',
    'ENDS_WITH':                '(${1:value}, ${2:suffix})',
    'CONTAINS_SUBSTR':          '(${1:value}, ${2:search_value})',
    'STRPOS':                   '(${1:value}, ${2:subvalue})',
    'INSTR':                    '(${1:value}, ${2:subvalue})',
    'FORMAT':                   '(${1:format_string}, ${2:value})',
    'INITCAP':                  '(${1:value})',
    'SOUNDEX':                  '(${1:value})',
    'TRANSLATE':                '(${1:value}, ${2:source_characters}, ${3:target_characters})',
    'COLLATE':                  '(${1:value}, ${2:collation_spec})',
    'NORMALIZE':                '(${1:value})',
    'NORMALIZE_AND_CASEFOLD':   '(${1:value})',
    'EDIT_DISTANCE':            '(${1:value1}, ${2:value2})',
    'TO_BASE64':                '(${1:bytes_value})',
    'FROM_BASE64':              '(${1:string_value})',
    'TO_BASE32':                '(${1:bytes_value})',
    'FROM_BASE32':              '(${1:string_value})',
    'TO_HEX':                   '(${1:bytes_value})',
    'FROM_HEX':                 '(${1:string_value})',
    'TO_CODE_POINTS':           '(${1:value})',
    'CODE_POINTS_TO_STRING':    '(${1:array_of_code_points})',
    'CODE_POINTS_TO_BYTES':     '(${1:array_of_code_points})',
    'ASCII':                    '(${1:value})',
    'CHR':                      '(${1:code_point})',
    'UNICODE':                  '(${1:value})',
    'SAFE_CONVERT_BYTES_TO_STRING': '(${1:bytes_value})',
    // JSON
    'JSON_VALUE':               '(${1:json_expr}, ${2:\'\\$.path\'})',
    'JSON_QUERY':               '(${1:json_expr}, ${2:\'\\$.path\'})',
    'JSON_VALUE_ARRAY':         '(${1:json_expr}, ${2:\'\\$.path\'})',
    'JSON_QUERY_ARRAY':         '(${1:json_expr}, ${2:\'\\$.path\'})',
    'JSON_EXTRACT':             '(${1:json_string}, ${2:\'\\$.path\'})',
    'JSON_EXTRACT_SCALAR':      '(${1:json_string}, ${2:\'\\$.path\'})',
    'JSON_EXTRACT_ARRAY':       '(${1:json_string}, ${2:\'\\$.path\'})',
    'JSON_EXTRACT_STRING_ARRAY':'(${1:json_string}, ${2:\'\\$.path\'})',
    'JSON_TYPE':                '(${1:json_expr})',
    'JSON_KEYS':                '(${1:json_expr})',
    'JSON_STRIP_NULLS':         '(${1:json_expr})',
    'JSON_FLATTEN':             '(${1:json_expr})',
    'PARSE_JSON':               '(${1:string_value})',
    'TO_JSON':                  '(${1:value})',
    'TO_JSON_STRING':           '(${1:value})',
    'STRING':                   '(${1:json_expr})',
    'BOOL':                     '(${1:json_expr})',
    'INT64':                    '(${1:json_expr})',
    'FLOAT64':                  '(${1:json_expr})',
    'LAX_BOOL':                 '(${1:json_expr})',
    'LAX_FLOAT64':              '(${1:json_expr})',
    'LAX_INT64':                '(${1:json_expr})',
    'LAX_STRING':               '(${1:json_expr})',
    // Array
    'ARRAY_LENGTH':             '(${1:array_expression})',
    'ARRAY_CONCAT':             '(${1:array1}, ${2:array2})',
    'ARRAY_TO_STRING':          '(${1:array_expression}, ${2:delimiter})',
    'ARRAY_REVERSE':            '(${1:array_expression})',
    'ARRAY_FIRST':              '(${1:array_expression})',
    'ARRAY_LAST':               '(${1:array_expression})',
    'ARRAY_SLICE':              '(${1:array_expression}, ${2:start_offset}, ${3:end_offset})',
    'GENERATE_ARRAY':           '(${1:start_value}, ${2:end_value}, ${3:step_value})',
    'GENERATE_DATE_ARRAY':      '(${1:start_date}, ${2:end_date}, INTERVAL ${3:step} ${4|DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'GENERATE_TIMESTAMP_ARRAY': '(${1:start_timestamp}, ${2:end_timestamp}, INTERVAL ${3:step} ${4|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY|})',
    // Date
    'DATE':                     '(${1:year}, ${2:month}, ${3:day})',
    'DATE_ADD':                 '(${1:date_expression}, INTERVAL ${2:int64_value} ${3|DAY,WEEK,ISOWEEK,MONTH,QUARTER,YEAR|})',
    'DATE_SUB':                 '(${1:date_expression}, INTERVAL ${2:int64_value} ${3|DAY,WEEK,ISOWEEK,MONTH,QUARTER,YEAR|})',
    'DATE_DIFF':                '(${1:end_date}, ${2:start_date}, ${3|DAY,WEEK,ISOWEEK,MONTH,QUARTER,YEAR|})',
    'DATE_TRUNC':               '(${1:date_expression}, ${2|DAY,WEEK,ISOWEEK,MONTH,QUARTER,YEAR|})',
    'DATE_BUCKET':              '(${1:date_expression}, INTERVAL ${2:bucket_width} ${3|DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'FORMAT_DATE':              '(${1:\'%Y-%m-%d\'}, ${2:date_expression})',
    'PARSE_DATE':               '(${1:\'%Y-%m-%d\'}, ${2:string_value})',
    'LAST_DAY':                 '(${1:date_expression}, ${2|MONTH,QUARTER,YEAR,WEEK,ISOWEEK|})',
    'DATE_FROM_UNIX_DATE':      '(${1:int64_value})',
    'UNIX_DATE':                '(${1:date_value})',
    'EXTRACT':                  '(${1|YEAR,MONTH,DAY,DAYOFWEEK,DAYOFYEAR,WEEK,ISOWEEK,QUARTER,HOUR,MINUTE,SECOND,MILLISECOND,MICROSECOND|} FROM ${2:datetime_expression})',
    // Datetime
    'DATETIME':                 '(${1:date_expression}, ${2:time_expression})',
    'DATETIME_ADD':             '(${1:datetime_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'DATETIME_SUB':             '(${1:datetime_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'DATETIME_DIFF':            '(${1:end_datetime}, ${2:start_datetime}, ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'DATETIME_TRUNC':           '(${1:datetime_expression}, ${2|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'DATETIME_BUCKET':          '(${1:datetime_expression}, INTERVAL ${2:bucket_width} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'FORMAT_DATETIME':          '(${1:\'%Y-%m-%dT%H:%M:%S\'}, ${2:datetime_expression})',
    'PARSE_DATETIME':           '(${1:\'%Y-%m-%dT%H:%M:%S\'}, ${2:string_value})',
    // Time
    'TIME':                     '(${1:hour}, ${2:minute}, ${3:second})',
    'TIME_ADD':                 '(${1:time_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR|})',
    'TIME_SUB':                 '(${1:time_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR|})',
    'TIME_DIFF':                '(${1:end_time}, ${2:start_time}, ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR|})',
    'TIME_TRUNC':               '(${1:time_expression}, ${2|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR|})',
    'FORMAT_TIME':              '(${1:\'%H:%M:%S\'}, ${2:time_expression})',
    'PARSE_TIME':               '(${1:\'%H:%M:%S\'}, ${2:string_value})',
    // Timestamp
    'TIMESTAMP':                '(${1:string_expression})',
    'TIMESTAMP_ADD':            '(${1:timestamp_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY|})',
    'TIMESTAMP_SUB':            '(${1:timestamp_expression}, INTERVAL ${2:int64_value} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY|})',
    'TIMESTAMP_DIFF':           '(${1:end_timestamp}, ${2:start_timestamp}, ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY|})',
    'TIMESTAMP_TRUNC':          '(${1:timestamp_expression}, ${2|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'TIMESTAMP_BUCKET':         '(${1:timestamp_expression}, INTERVAL ${2:bucket_width} ${3|MICROSECOND,MILLISECOND,SECOND,MINUTE,HOUR,DAY,WEEK,MONTH,QUARTER,YEAR|})',
    'FORMAT_TIMESTAMP':         '(${1:\'%Y-%m-%dT%H:%M:%SZ\'}, ${2:timestamp_expression})',
    'PARSE_TIMESTAMP':          '(${1:\'%Y-%m-%dT%H:%M:%SZ\'}, ${2:string_value})',
    'TIMESTAMP_SECONDS':        '(${1:int64_value})',
    'TIMESTAMP_MILLIS':         '(${1:int64_value})',
    'TIMESTAMP_MICROS':         '(${1:int64_value})',
    'UNIX_SECONDS':             '(${1:timestamp_value})',
    'UNIX_MILLIS':              '(${1:timestamp_value})',
    'UNIX_MICROS':              '(${1:timestamp_value})',
    // Interval
    'MAKE_INTERVAL':            '(year => ${1:0}, month => ${2:0}, day => ${3:0}, hour => ${4:0}, minute => ${5:0}, second => ${6:0})',
    // Hashing
    'FARM_FINGERPRINT':         '(${1:value})',
    'MD5':                      '(${1:value})',
    'SHA1':                     '(${1:value})',
    'SHA256':                   '(${1:value})',
    'SHA512':                   '(${1:value})',
    // Window / navigation
    'NTILE':                    '(${1:buckets})',
    'NTH_VALUE':                '(${1:value_expression}, ${2:nth})',
    'FIRST_VALUE':              '(${1:value_expression})',
    'LAST_VALUE':               '(${1:value_expression})',
    'LAG':                      '(${1:value_expression}, ${2:offset}, ${3:default_value})',
    'LEAD':                     '(${1:value_expression}, ${2:offset}, ${3:default_value})',
    'PERCENTILE_CONT':          '(${1:value}, ${2:percentile}) OVER ()',
    'PERCENTILE_DISC':          '(${1:value}, ${2:percentile}) OVER ()',
    // Geography
    'ST_GEOGPOINT':             '(${1:longitude}, ${2:latitude})',
    'ST_DISTANCE':              '(${1:geography1}, ${2:geography2})',
    'ST_AREA':                  '(${1:geography_expression})',
    'ST_INTERSECTS':            '(${1:geography1}, ${2:geography2})',
    'ST_CONTAINS':              '(${1:geography1}, ${2:geography2})',
    'ST_WITHIN':                '(${1:geography1}, ${2:geography2})',
    'ST_DWITHIN':               '(${1:geography1}, ${2:geography2}, ${3:distance_meters})',
    'ST_BUFFER':                '(${1:geography_expression}, ${2:buffer_radius_meters})',
    'ST_GEOGFROM':              '(${1:expression})',
    'ST_GEOGFROMTEXT':          '(${1:wkt_string})',
    'ST_GEOGFROMWKB':           '(${1:wkb_bytes_or_hex})',
    'ST_GEOGFROMGEOJSON':       '(${1:geojson_string})',
    'ST_ASTEXT':                '(${1:geography_expression})',
    'ST_ASGEOJSON':             '(${1:geography_expression})',
    'ST_ASBINARY':              '(${1:geography_expression})',
    'ST_LENGTH':                '(${1:geography_expression})',
    'ST_PERIMETER':             '(${1:geography_expression})',
    'ST_NUMPOINTS':             '(${1:geography_expression})',
    'ST_CENTROID':              '(${1:geography_expression})',
    'ST_MAKELINE':              '(${1:geography_array})',
    'ST_MAKEPOLYGON':           '(${1:linestring_geography})',
    'ST_DIFFERENCE':            '(${1:geography1}, ${2:geography2})',
    'ST_UNION':                 '(${1:geography_array})',
    'ST_INTERSECTION':          '(${1:geography1}, ${2:geography2})',
    'ST_SIMPLIFY':              '(${1:geography_expression}, ${2:tolerance_in_meters})',
    'ST_GEOHASH':               '(${1:point_geography}, ${2:maxchars})',
    'ST_X':                     '(${1:point_geography})',
    'ST_Y':                     '(${1:point_geography})',
    // Utility
    'ERROR':                    '(${1:error_message})',
    'SEARCH':                   '(${1:table_or_column}, ${2:search_query})',
    'GENERATE_UUID':            '()',
    'SESSION_USER':             '()',
    'CURRENT_DATE':             '()',
    'CURRENT_DATETIME':         '()',
    'CURRENT_TIME':             '()',
    'CURRENT_TIMESTAMP':        '()',
    'RAND':                     '()',
};

// ---------------------------------------------------------------------------
// Module-level utilities
// ---------------------------------------------------------------------------

function dedupeByName<T extends { column_name: string }>(rows: T[]): T[] {
    return rows.filter((el, idx, arr) =>
        arr.findIndex(e => e.column_name === el.column_name) === idx
    );
}

function buildColumnDoc(
    name: string,
    dataType: string,
    isPartition: boolean,
    description: string | undefined,
): vscode.MarkdownString {
    return new vscode.MarkdownString(
        `**\`${name}\`** · \`${dataType}\`` +
        (isPartition ? '\n\n🔑 *Partition column*' : '') +
        (description ? `\n\n${description}` : '')
    );
}
