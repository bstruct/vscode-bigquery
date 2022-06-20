import * as vscode from 'vscode';
import { BigqueryTreeItem, TreeItemType } from './tree-item';
import { BigQuery } from '@google-cloud/bigquery';
import { ProjectsClient } from '@google-cloud/resource-manager';

export class BigQueryTreeDataProvider implements vscode.TreeDataProvider<BigqueryTreeItem> {

    private routineTreeItems: BigqueryTreeItem[] = [];
    private modelTreeItems: BigqueryTreeItem[] = [];

    constructor() {
    }

    onDidChangeTreeData?: vscode.Event<void | BigqueryTreeItem | BigqueryTreeItem[] | null | undefined> | undefined;

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
                case TreeItemType.Project:
                    const datasets = (await this.getDatasets(projectId));
                    resolve(datasets);

                case TreeItemType.Dataset:
                case TreeItemType.DatasetLink:

                    if (element.datasetId === null) { resolve([]); return; }

                    const treeItems = [];

                    const tablesPromise = this.getTables(projectId, datasetId);

                    if (treeItemType === TreeItemType.Dataset) {
                        const routinesPromise = this.getRoutines(projectId, datasetId);
                        const modelsPromise = this.getModels(projectId, datasetId);

                        await Promise.all([routinesPromise, modelsPromise, tablesPromise]);

                        const routines = await routinesPromise;

                        if (routines.length > 0) {
                            this.routineTreeItems.push(...routines);
                            const routinesTreeItem = new BigqueryTreeItem(TreeItemType.Routine, projectId, datasetId, null, `Routines (${routines.length})`, "", vscode.TreeItemCollapsibleState.Collapsed);
                            treeItems.push(routinesTreeItem);
                        }

                        const models = await modelsPromise;
                        if (models.length > 0) {
                            this.modelTreeItems.push(...models);
                            const modelTreeItem = new BigqueryTreeItem(TreeItemType.Model, projectId, datasetId, null, `Models (${models.length})`, "", vscode.TreeItemCollapsibleState.Collapsed);
                            treeItems.push(modelTreeItem);
                        }
                    }

                    treeItems.push(...(await tablesPromise));

                    resolve(treeItems);

                case TreeItemType.Routine:

                    const qRoutines = this.routineTreeItems
                        .filter(c => c.projectId === projectId && c.datasetId === datasetId)
                        .sort((a, b) => (a.description || '').toString().localeCompare((b.description || '').toString()));

                    resolve(qRoutines);

                    break;
                case TreeItemType.Model:

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
        const projects = await projectsClient.searchProjectsAsync();

        let listProjects = [];
        for await (const project of projects) {
            listProjects.push(project);
        }

        return listProjects
            .filter(c => c.state === 'ACTIVE')
            .map(c => {
                const projectId = c.projectId ?? 'xxx';
                return new BigqueryTreeItem(TreeItemType.Project, projectId, null, null, projectId, '', vscode.TreeItemCollapsibleState.Collapsed);
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

                        let treeItemType = TreeItemType.Dataset;
                        if (metadata[0].type === 'LINKED') {
                            treeItemType = TreeItemType.DatasetLink;
                        }

                        const datasetId = c.id ?? 'xxx';
                        return new BigqueryTreeItem(treeItemType, projectId, datasetId, null, datasetId, "", vscode.TreeItemCollapsibleState.Collapsed)

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

                let treeItemType = TreeItemType.Table;
                if (c.metadata.timePartitioning) {
                    treeItemType = TreeItemType.PartitionedTable;
                } else {
                    if (c.metadata.type === 'VIEW') {
                        treeItemType = TreeItemType.TableView;
                    }
                }
                return new BigqueryTreeItem(treeItemType, projectId, datasetId, tableId, tableId, "", vscode.TreeItemCollapsibleState.None);
            });

    }

    private async getRoutines(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const routines = await dataset.getRoutines();

        return routines[0]
            .map(c => {

                const routineId = c.id ?? 'xxx';

                return new BigqueryTreeItem(TreeItemType.Routine, projectId, datasetId, routineId, routineId, "", vscode.TreeItemCollapsibleState.None);
            });

    }

    private async getModels(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const models = await dataset.getModels();

        return models[0]
            .map(c => {

                const modelId = c.id ?? 'xxx';

                return new BigqueryTreeItem(TreeItemType.Model, projectId, datasetId, modelId, modelId, "", vscode.TreeItemCollapsibleState.None);
            });

    }

}