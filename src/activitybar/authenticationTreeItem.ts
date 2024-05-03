import * as vscode from 'vscode';
import * as commands from '../extensionCommands';

export enum AuthenticationTreeItemType {
    none,
    authenticatedParent,
    addAuthenticationParent,
    user,
    error,
    problemsAuthenticatingParent,
    userLogin,
    userLoginPlusGoogleDrive,
    serviceAccount,
    troubleshoot,
    gcloudInit,
    userLoginNoBrowserLaunch
}

export class AuthenticationTreeItem extends vscode.TreeItem {

    constructor(
        public readonly treeItemType: AuthenticationTreeItemType,

        public readonly label: string,
        public readonly description: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    ) {
        super(description, collapsibleState);

        switch (treeItemType) {
            case AuthenticationTreeItemType.user:
                this.contextValue = 'gcp-user';
                break;
            case AuthenticationTreeItemType.userLogin:
                this.command = { command: commands.COMMAND_USER_LOGIN, arguments: [this] } as vscode.Command;
                break;
            case AuthenticationTreeItemType.userLoginPlusGoogleDrive:
                this.command = { command: commands.COMMAND_USER_LOGIN_WITH_DRIVE, arguments: [this] } as vscode.Command;
                break;
            case AuthenticationTreeItemType.userLoginNoBrowserLaunch:
                this.command = { command: commands.COMMAND_USER_LOGIN_NO_LAUNCH_BROWSER, arguments: [this] } as vscode.Command;
                break;
            case AuthenticationTreeItemType.serviceAccount:
                this.command = { command: commands.COMMAND_SERVICE_ACCOUNT_LOGIN, arguments: [this] } as vscode.Command;
                break;
            case AuthenticationTreeItemType.gcloudInit:
                this.command = { command: commands.COMMAND_GCLOUD_INIT, arguments: [this] } as vscode.Command;
                break;
            case AuthenticationTreeItemType.troubleshoot:
                this.command = { command: commands.AUTHENTICATION_TROUBLESHOOT, arguments: [this] } as vscode.Command;
                break;
        }

        this.description = this.description;
    }
}