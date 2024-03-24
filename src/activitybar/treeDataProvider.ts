import * as vscode from 'vscode';
import { BigqueryTreeItem, TreeItemType } from './treeItem';
import { BigQuery, Dataset, Model, Routine, Table } from '@google-cloud/bigquery';
import { Authentication } from '../services/authentication';
import { getBigQueryClient, SETTING_PINNED_PROJECTS, SETTING_PROJECTS, SETTING_TABLES } from '../extensionCommands';
import { GetMetadataOptions, MetadataResponse } from '@google-cloud/common/build/src/service-object';
import { TableReference } from '../services/tableMetadata';

// const { google } = require('googleapis');
// const vault = google.vault('v1');

export class BigQueryTreeDataProvider implements vscode.TreeDataProvider<BigqueryTreeItem> {

    private routineTreeItems: BigqueryTreeItem[] = [];
    private modelTreeItems: BigqueryTreeItem[] = [];

    constructor() {
    }

    private _onDidChangeTreeData = new vscode.EventEmitter<void | BigqueryTreeItem | BigqueryTreeItem[] | null | undefined>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    getTreeItem(element: BigqueryTreeItem): vscode.TreeItem | Thenable<vscode.TreeItem> {
        return element;
    }

    getChildren(element?: BigqueryTreeItem): vscode.ProviderResult<BigqueryTreeItem[]> {

        return new Promise(async (resolve, reject) => {

            if (element === null || element === undefined) {
                resolve(this.getProjects());
                return;
            }

            const treeItemType = element.treeItemType;
            const projectId = element.projectId ?? 'xxx';
            const datasetId = element.datasetId ?? 'xxx';

            switch (treeItemType) {
                case TreeItemType.project:

                    // const savedQueries = await this.getSavedQueries(projectId)
                    //     .catch(err => console.error(err));

                    await
                        this.getDatasets(projectId)
                            .then(datasets => resolve(datasets))
                            .catch(e => reject(e));

                case TreeItemType.dataset:
                case TreeItemType.datasetLink:

                    if (element.datasetId === null) { resolve([]); return; }

                    const treeItems: BigqueryTreeItem[] = [];

                    const tablesPromise = this.getTables(projectId, datasetId);

                    if (treeItemType === TreeItemType.dataset) {
                        const routinesPromise = this.getRoutines(projectId, datasetId);
                        const modelsPromise = this.getModels(projectId, datasetId);

                        await
                            Promise.all([routinesPromise, modelsPromise, tablesPromise])
                                .catch(e => reject(e));

                        const routines = await routinesPromise;

                        if (routines.length > 0) {
                            this.routineTreeItems = this.deduplicate(projectId, datasetId, this.routineTreeItems, routines);

                            const routinesTreeItem = new BigqueryTreeItem(TreeItemType.routine, projectId, datasetId, null, `Routines (${routines.length})`, '', false, vscode.TreeItemCollapsibleState.Collapsed);
                            treeItems.push(routinesTreeItem);
                        }

                        const models = await modelsPromise;
                        if (models.length > 0) {
                            this.modelTreeItems = this.deduplicate(projectId, datasetId, this.modelTreeItems, models);

                            const modelTreeItem = new BigqueryTreeItem(TreeItemType.model, projectId, datasetId, null, `Models (${models.length})`, '', false, vscode.TreeItemCollapsibleState.Collapsed);
                            treeItems.push(modelTreeItem);
                        }
                    }

                    await tablesPromise
                        .then(tables => {
                            treeItems.push(...tables);
                            resolve(treeItems);
                        })
                        .catch(e => reject(e));


                case TreeItemType.routine:

                    const qRoutines = this.routineTreeItems
                        .filter(c => c.projectId === projectId && c.datasetId === datasetId)
                        .sort((a, b) => (a.description || '').toString().localeCompare((b.description || '').toString()));

                    resolve(qRoutines);

                    break;
                case TreeItemType.model:

                    const qModel = this.modelTreeItems
                        .filter(c => c.projectId === projectId && c.datasetId === datasetId)
                        .sort((a, b) => (a.description || '').toString().localeCompare((b.description || '').toString()));

                    resolve(qModel);

                    break;
                default:
                    resolve([]);
            }

        });

    }

    private async getProjects(): Promise<BigqueryTreeItem[]> {

        const defaultProjectIdPromise = Authentication.getDefaultProjectId();
        const bqClient = await getBigQueryClient();
        const bqProjects = await bqClient.getProjects();

        let listProjects = this.getProjectsFromSettings();
        for (const project of bqProjects.projects || []) {
            const projectId = (project.id || 'xxx').toLowerCase();
            if (listProjects.indexOf(projectId) < 0) {
                listProjects.push(projectId);
            }
        }

        const defaultProjectId = await defaultProjectIdPromise;

        const pinnedProjects = vscode.workspace
            .getConfiguration()
            .get(SETTING_PINNED_PROJECTS) as string[] || []
                .sort((a: string, b: string) => a.localeCompare(b));

        const listProjectSorted =
            listProjects
                .sort((a, b) =>
                    (
                        pinnedProjects.indexOf(a) >= 0
                        &&
                        pinnedProjects.indexOf(b) >= 0
                    ) ? a.localeCompare(b) : (
                        pinnedProjects.indexOf(a) >= 0
                        &&
                        pinnedProjects.indexOf(b) < 0
                    ) ? -10 : (
                        pinnedProjects.indexOf(a) < 0
                        &&
                        pinnedProjects.indexOf(b) < 0
                    ) ? a.localeCompare(b) : 0
                )
            ;

        return listProjectSorted
            .map(projectId => {
                return new BigqueryTreeItem(
                    TreeItemType.project,
                    projectId,
                    null,
                    null,
                    projectId,
                    defaultProjectId === projectId ? 'DEFAULT' : '',
                    pinnedProjects.indexOf(projectId) >= 0,
                    vscode.TreeItemCollapsibleState.Collapsed
                );
            });

    }

    private async getDatasets(projectId: string): Promise<BigqueryTreeItem[]> {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const datasets = await bigqueryClient.getDatasets({ all: true, filter: '' });

        const datasetList = datasets[0].filter(c => c.id !== null && (!c.id?.startsWith('_')));

        const tablesFromSettings = this.getTablesFromSettings();
        const qDatasetsFromTablesInSettings = tablesFromSettings.filter(c => c.projectId === projectId);

        if (qDatasetsFromTablesInSettings.length > 0) {

            for (let index = 0; index < qDatasetsFromTablesInSettings.length; index++) {
                const element = qDatasetsFromTablesInSettings[index];

                if (!datasetList.find(c => c.id === element.datasetId)) {

                    const getMetadata = function (options?: GetMetadataOptions): Promise<MetadataResponse> {
                        return new Promise((resolve, reject) => {
                            resolve({
                                // eslint-disable-next-line @typescript-eslint/naming-convention
                                "0": {
                                    type: ''
                                }
                            } as MetadataResponse);
                        });
                    };

                    datasetList.push({
                        id: element.datasetId,
                        getMetadata: getMetadata
                    } as Dataset);
                }
            }

        }

        const datasetPromises = datasetList.map(c => {
            return c.getMetadata()
                .then(metadata => {

                    let treeItemType = TreeItemType.dataset;
                    if (metadata[0].type === 'LINKED') {
                        treeItemType = TreeItemType.datasetLink;
                    }

                    const datasetId = c.id ?? 'xxx';
                    return new BigqueryTreeItem(treeItemType, projectId, datasetId, null, datasetId, '', false, vscode.TreeItemCollapsibleState.Collapsed);

                });
        });

        return (await Promise.all(datasetPromises));

    }

    private async getTables(projectId: string, datasetId: string): Promise<BigqueryTreeItem[]> {

        let tables: Table[] = [];
        try {
            const bigqueryClient = new BigQuery({ projectId: projectId });
            const dataset = bigqueryClient.dataset(datasetId);
            const getTablesResponse = await dataset.getTables();
            tables = getTablesResponse[0]
                .filter(c => c.id !== null && (!c.id?.startsWith('_')));
        } catch (error) { }

        const tablesFromSettings = this.getTablesFromSettings();
        const qTablesInSettings = tablesFromSettings.filter(c => c.projectId === projectId && c.datasetId === datasetId);

        if (qTablesInSettings.length > 0) {

            for (let index = 0; index < qTablesInSettings.length; index++) {
                const element = qTablesInSettings[index];

                if (!tables.find(c => c.id === element.tableId)) {
                    tables.push({
                        id: element.tableId,
                        metadata: {
                            type: 'TABLE'
                        }
                    } as Table);
                }
            }
        }

        return tables
            .map(c => {
                const tableId = c.id ?? 'xxx';

                let treeItemType = TreeItemType.table;
                if (c.metadata.timePartitioning) {
                    treeItemType = TreeItemType.partitionedTable;
                } else {
                    if (c.metadata.type === 'VIEW') {
                        treeItemType = TreeItemType.tableView;
                    }
                }
                return new BigqueryTreeItem(treeItemType, projectId, datasetId, tableId, tableId, '', false, vscode.TreeItemCollapsibleState.None);
            });

    }

    private getProjectsFromSettings(): string[] {

        let projects = (vscode.workspace
            .getConfiguration()
            .get(SETTING_PROJECTS) as string[] || [])
            .map(c => (c as string).toLowerCase())
            ;

        const tables = this.getTablesFromSettings();
        for (let index = 0; index < tables.length; index++) {
            const element = tables[index];
            if (projects.indexOf(element.projectId) < 0) {
                projects.push(element.projectId);
            }
        }

        return projects;
    }

    private getTablesFromSettings(): TableReference[] {

        return (vscode.workspace
            .getConfiguration()
            .get(SETTING_TABLES) as string[] || [])
            .map(c => (c as string).toLowerCase())
            .map(c => c.split('.'))
            .filter(c => c.length === 3)
            .map(
                c => { return { projectId: c[0], datasetId: c[1], tableId: c[2] } as TableReference; }
            );

    }

    private async getRoutines(projectId: string, datasetId: string) {

        let routines: Routine[] = [];
        try {
            const bigqueryClient = new BigQuery({ projectId: projectId });
            const dataset = bigqueryClient.dataset(datasetId);
            routines = (await dataset.getRoutines())[0];
        } catch (error) { }

        return routines
            .map(c => {
                const routineId = c.id ?? 'xxx';
                return new BigqueryTreeItem(TreeItemType.routine, projectId, datasetId, routineId, routineId, '', false, vscode.TreeItemCollapsibleState.None);
            });

    }

    private async getModels(projectId: string, datasetId: string) {

        let models: Model[] = [];
        try {
            const bigqueryClient = new BigQuery({ projectId: projectId });
            const dataset = bigqueryClient.dataset(datasetId);
            models = (await dataset.getModels())[0];
        } catch (error) { }

        return models
            .map(c => {
                const modelId = c.id ?? 'xxx';
                return new BigqueryTreeItem(TreeItemType.model, projectId, datasetId, modelId, modelId, '', false, vscode.TreeItemCollapsibleState.None);
            });
    }

    private deduplicate(projectId: string, datasetId: string, treeItems: BigqueryTreeItem[], newItems: BigqueryTreeItem[]): BigqueryTreeItem[] {
        const list = treeItems.filter(c => c.projectId !== projectId && c.datasetId !== datasetId);
        list.push(...newItems);
        return list;
    }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

}