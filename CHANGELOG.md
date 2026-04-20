# Changelog

All notable changes to the **BigQuery Data View** extension are documented here.

---

## 0.5.1

- **Fix** - Included `location` parameter in `getQueryResults` API request (Issue #90).

## 0.5.0

### BigQuery Notebooks are here! 🎉

A brand-new cell-based workflow for BigQuery SQL — right inside VS Code.

Create **`.bqnb`** notebook files to organize, execute, and visualize BigQuery queries in a familiar notebook interface.

#### What you can do

- **Write SQL in cells** with full syntax highlighting, code completion, and live diagnostics — the same language features available in `.bqsql` files.
- **Execute cells individually** and see results inline with a high-performance data grid.
- **Visualize results** with built-in Chart.js charts (Bar, Line, Scatter, Pie, Doughnut) directly in cell outputs.
- **Inspect job metadata** per cell — bytes processed, timing, statement type, and more.
- **Export from notebooks** — Download CSV, Download JSONL, and Send to Pub/Sub are all available from cell outputs.
- **Multi-statement scripts** are supported — child jobs are split into separate outputs automatically.

#### Getting started

1. Open the **BigQuery** activity bar and click the **Create Notebook** button in the Explorer toolbar, or run the command **`BigueryView: Create New Notebook`**.
2. Write a BigQuery SQL query in the first cell.
3. Click the **Run** button on the cell (or use the notebook toolbar).
4. Results appear inline — switch between the **Results**, **Chart**, and **Job Info** tabs.

### Other changes

- **Copilot Chat integration** — use `@bigquery` in GitHub Copilot Chat for schema-aware SQL assistance. Try `/explain`, `/optimize`, and `/schema` commands.
- **Search** — press `Ctrl+Shift+F10` (`Cmd+Shift+F10` on macOS) to search datasets, tables, and columns across your projects using the Dataplex Catalog API.
- **Job History panel** — browse recent BigQuery jobs, filter to your own, and open any job's SQL in a new editor.
- **Open DDL** now supports routines (procedures/UDFs) and ML models in addition to tables and views.
- **Create Query** on ML models generates `ML.PREDICT`/`ML.FORECAST` templates.
