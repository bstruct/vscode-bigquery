import * as vscode from 'vscode';
import { Authentication } from '../services/authentication';
import { AuthenticationTreeItem, AuthenticationTreeItemType } from './authenticationTreeItem';

export class GcpAuthenticationTreeDataProvider implements vscode.TreeDataProvider<AuthenticationTreeItem> {

    constructor() {
    }

    private _onDidChangeTreeData = new vscode.EventEmitter<void | AuthenticationTreeItem | AuthenticationTreeItem[] | null | undefined>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    getTreeItem(element: AuthenticationTreeItem): AuthenticationTreeItem | Thenable<AuthenticationTreeItem> {
        return element;
    }

    getChildren(element?: AuthenticationTreeItem | undefined): vscode.ProviderResult<AuthenticationTreeItem[]> {

        if (element === null || element === undefined) {
            const item1 = new AuthenticationTreeItem(AuthenticationTreeItemType.authenticatedParent, "authenticated", "", vscode.TreeItemCollapsibleState.Expanded);
            const item2 = new AuthenticationTreeItem(AuthenticationTreeItemType.addAuthenticationParent, "add authentication", "", vscode.TreeItemCollapsibleState.Expanded);
            const item3 = new AuthenticationTreeItem(AuthenticationTreeItemType.problemsAuthenticatingParent, "problems authenticating?", "", vscode.TreeItemCollapsibleState.Expanded);
            return [item1, item2, item3];
        }

        if (element !== null && element !== undefined && element.treeItemType === AuthenticationTreeItemType.addAuthenticationParent) {
            const item1 = new AuthenticationTreeItem(AuthenticationTreeItemType.userLogin, "user login", "", vscode.TreeItemCollapsibleState.None);
            const item2 = new AuthenticationTreeItem(AuthenticationTreeItemType.userLoginPlusGoogleDrive, "user login plus google drive", "", vscode.TreeItemCollapsibleState.None);
            const item3 = new AuthenticationTreeItem(AuthenticationTreeItemType.userLoginNoBrowserLaunch, "user login no browser launch", "via command line", vscode.TreeItemCollapsibleState.None);
            const item4 = new AuthenticationTreeItem(AuthenticationTreeItemType.serviceAccount, "service account", "", vscode.TreeItemCollapsibleState.None);
            return [item1, item2, item3, item4];
        }

        if (element !== null && element !== undefined && element.treeItemType === AuthenticationTreeItemType.problemsAuthenticatingParent) {
            const item1 = new AuthenticationTreeItem(AuthenticationTreeItemType.troubleshoot, "troubleshoot", "", vscode.TreeItemCollapsibleState.None);
            const item2 = new AuthenticationTreeItem(AuthenticationTreeItemType.gcloudInit, "gcloud init", "", vscode.TreeItemCollapsibleState.None);
            return [item1, item2];
        }

        if (element !== null && element !== undefined && element.treeItemType === AuthenticationTreeItemType.authenticatedParent) {
            return new Promise(async (resolve, reject) => {

                await Authentication
                    .list(false)
                    .then(result => {

                        let list: AuthenticationTreeItem[] = [];

                        for (let index = 0; index < result.length; index++) {
                            const element = result[index];

                            let item = new AuthenticationTreeItem(AuthenticationTreeItemType.user, element.account, "", vscode.TreeItemCollapsibleState.None);
                            if (element.status === "ACTIVE") {
                                item = new AuthenticationTreeItem(AuthenticationTreeItemType.user, `${element.account} - [ACTIVE]`, "", vscode.TreeItemCollapsibleState.None);
                            }
                            list.push(item);
                        }

                        resolve(list);
                    })
                    .catch(error=>{
                        const item1 = new AuthenticationTreeItem(AuthenticationTreeItemType.error, "error loading user list", "", vscode.TreeItemCollapsibleState.None);
                        resolve([item1]);
                    });
            });

        } else {
            return [];
        }
    }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

}