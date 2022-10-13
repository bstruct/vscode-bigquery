import * as vscode from 'vscode';
import { BigqueryTreeItem, TreeItemType } from './treeItem';
import { BigQuery } from '@google-cloud/bigquery';
import { ProjectsClient } from '@google-cloud/resource-manager';
import { Authentication } from '../services/authentication';
import { SETTING_PINNED_PROJECTS } from '../extensionCommands';

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

    private async getProjects() {

        const projectsClient = new ProjectsClient();

        const defaultProjectIdPromise = Authentication.getDefaultProjectId();
        const projects = await projectsClient.searchProjectsAsync();

        let listProjects = [];
        for await (const project of projects) {
            listProjects.push(project);
        }

        const defaultProjectId = await defaultProjectIdPromise;

        const pinnedProjects = vscode.workspace
            .getConfiguration()
            .get(SETTING_PINNED_PROJECTS) as string[] || []
                .sort((a: string, b: string) => a.localeCompare(b));

        const listProjectSorted =
            listProjects
                .filter(c => c.state === 'ACTIVE')
                .map(c => c.projectId || 'xxx')
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

        const datasetPromises = datasets[0]
            .filter(c => c.id !== null && (!c.id?.startsWith('_')))
            .map(c => {

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

    private async getTables(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const tables = await dataset.getTables();

        return tables[0]
            .filter(c => c.id !== null && (!c.id?.startsWith('_')))
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

    private async getRoutines(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const routines = await dataset.getRoutines();

        return routines[0]
            .map(c => {

                const routineId = c.id ?? 'xxx';

                return new BigqueryTreeItem(TreeItemType.routine, projectId, datasetId, routineId, routineId, '', false, vscode.TreeItemCollapsibleState.None);
            });

    }

    private async getModels(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const models = await dataset.getModels();

        return models[0]
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