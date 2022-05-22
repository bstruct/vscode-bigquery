import * as vscode from 'vscode';
import { BigqueryTreeItem, TreeItemType } from './bigquery-tree-item';
import { BigQuery } from '@google-cloud/bigquery';

import { ProjectsClient } from '@google-cloud/resource-manager';

export class BigQueryTreeDataProvider implements vscode.TreeDataProvider<BigqueryTreeItem> {

    constructor() {
    }

    onDidChangeTreeData?: vscode.Event<void | BigqueryTreeItem | BigqueryTreeItem[] | null | undefined> | undefined;

    getTreeItem(element: BigqueryTreeItem): vscode.TreeItem | Thenable<vscode.TreeItem> {


        const treeItem = new vscode.TreeItem(element.label, element.collapsibleState);


        return treeItem;
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
                    resolve(this.getDatasets(projectId));
                case TreeItemType.Dataset:

                    if (element.datasetId === null) { resolve([]); return; }

                    resolve(this.getTables(projectId, datasetId));
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

    private async getDatasets(projectId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const datasets = await bigqueryClient.getDatasets({ all: true, filter: '' });

        return datasets[0]
            .filter(c => c.id !== null && (!c.id?.startsWith('_')))
            .map(c => {
                const datasetId = c.id ?? 'xxx';
                return new BigqueryTreeItem(TreeItemType.Dataset, projectId, datasetId, null, datasetId, "", vscode.TreeItemCollapsibleState.Collapsed)
            });

    }

    private async getTables(projectId: string, datasetId: string) {

        const bigqueryClient = new BigQuery({ projectId: projectId });

        const dataset = bigqueryClient.dataset(datasetId);
        const tables = await dataset.getTables();

        return tables[0]
            .filter(c => c.id !== null && (!c.id?.startsWith('_')))
            .map(c => {
                const tableId = c.id ?? 'xxx';
                return new BigqueryTreeItem(TreeItemType.Table, projectId, datasetId, tableId, tableId, "", vscode.TreeItemCollapsibleState.None);
            });

    }

}