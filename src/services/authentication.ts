import * as vscode from 'vscode';
import * as cp from 'child_process';
import { AuthenticationListItem } from './authenticationListItem';
import { AuthenticationUserLoginResponse } from './authenticationUserLoginResponse';
import { CustomTerminal } from './customTerminal';

//https://cloud.google.com/sdk/docs/cheatsheet#credentials

export class Authentication {

    //https://cloud.google.com/sdk/gcloud/reference/auth/login
    public static async userLogin(): Promise<AuthenticationUserLoginResponse> {
        const result = await this.runCommand('gcloud auth login --update-adc --add-quota-project-to-adc --quiet --verbosity warning --format="json"', true);

        //set default project if needed
        await Authentication.setDefaultProject();

        return JSON.parse(result) as AuthenticationUserLoginResponse;
    }

    public static async userLoginWithDrive(): Promise<AuthenticationUserLoginResponse> {
        const result = await this.runCommand('gcloud auth login --update-adc --add-quota-project-to-adc --quiet --enable-gdrive-access --verbosity warning --format="json"', true);

        //set default project if needed
        await Authentication.setDefaultProject();

        return JSON.parse(result) as AuthenticationUserLoginResponse;
    }

    public static async serviceAccountLogin(fileUri: vscode.Uri): Promise<AuthenticationUserLoginResponse> {

        try {

            const result = await this.runCommand(`gcloud auth activate-service-account --key-file="${fileUri.fsPath}" --format="json"`, true);

            const typedResult = JSON.parse(result) as string[];
            if (typedResult.length === 0) {

                //change default credentials
                //https://cloud.google.com/docs/authentication/application-default-credentials#personal
                if (process.platform === 'win32') {
                    await this.runCommand(`copy "${fileUri.fsPath}" %APPDATA%\\gcloud\\application_default_credentials.json`, true);
                } else {
                    await this.runCommand(`cp "${fileUri.fsPath}" $HOME/.config/gcloud/application_default_credentials.json`, true);
                }

                //set default project if needed
                await Authentication.setDefaultProject();

                return { valid: true } as AuthenticationUserLoginResponse;
            }

        } catch (error) {
            console.info(JSON.stringify(error));
        }

        return { valid: false } as AuthenticationUserLoginResponse;
    }

    private static async setDefaultProject() {
        const defaultProject = await this.runCommand('gcloud config get project', true);
        if (!defaultProject) {
            const projectsString = await this.runCommand('gcloud projects list --format="json"', true);

            // console.info('projectsString');
            // console.info(projectsString);
            const projects = JSON.parse(projectsString);
            if (projects && projects.length && projects.length > 0) {
                const projectId = projects[0].projectId;
                if (projectId) {
                    await this.runCommand(`gcloud config set project "${projectId}"`, true);
                }
            }
        }
    }

    public static async list(forceShowConsole: boolean): Promise<AuthenticationListItem[]> {
        const result = await this.runCommand('gcloud auth list --format="json"', forceShowConsole);
        return JSON.parse(result) as AuthenticationListItem[];
    }

    public static async activate(account: string): Promise<boolean> {
        const _ = await this.runCommand(`gcloud config set core/account "${account}" --format="json"`, true);
        return true;
    }

    public static async revoke(account: string): Promise<boolean> {
        const result = await this.runCommand(`gcloud auth revoke "${account}" --format="json"`, true);
        return (JSON.parse(result) as string[]).indexOf(account) >= 0;
    }

    public static async getDefaultProjectId(): Promise<string> {
        const result = await this.runCommand(`gcloud config get-value project`, false);
        return result.trim();
    }

    public static async setDefaultProjectId(projectId: string): Promise<void> {
        await this.runCommand(`gcloud config set project ${projectId}`, true);
    }

    //https://cloud.google.com/sdk/gcloud/reference/auth/revoke

    private static runCommand(command: string, forceShow: boolean): Promise<string> {

        const terminalName = 'gcloud authentication';

        const qTerminal = vscode.window.terminals.find(c => c.name === terminalName);
        let terminal: vscode.Terminal;
        if (qTerminal) {
            terminal = qTerminal;
        } else {

            const customTerminal = new CustomTerminal();

            const terminalOptions = {
                name: terminalName,
                pty: customTerminal,
                isTransient: true,
            } as vscode.ExtensionTerminalOptions;

            terminal = vscode.window.createTerminal(terminalOptions);
        }

        // Black: 30
        // Blue: 34
        // Cyan: 36
        // Green: 32
        // Purple: 35
        // Red: 31
        // White: 37
        // Yellow: 33
        // terminal.sendText('\x1b[1m\x1b[34mHello world\x1b[0m');
        terminal.sendText(`\x1b[1m\x1b[34m# ${command}\x1b[0m`);

        if (forceShow) { terminal.show(); }

        const commandOptions = {} as cp.ExecOptions;

        return new Promise((resolve, reject) => {

            cp.exec(command, commandOptions, (error, stdout, stderr) => {
                if (error) {

                    terminal.sendText(stderr);

                    reject({ error, stdout, stderr });
                } else {

                    terminal.sendText(stdout);

                    resolve(stdout);
                }
            });

        });

    }

}