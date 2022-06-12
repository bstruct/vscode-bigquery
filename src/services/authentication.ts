import * as cp from 'child_process';
import { AuthenticationListItem } from './authentication-list-item';
import { AuthenticationUserLoginResponse } from './authentication-user-login-response';

//https://cloud.google.com/sdk/docs/cheatsheet#credentials

export class Authentication {

    //https://cloud.google.com/sdk/gcloud/reference/auth/login
    public static async userLogin(): Promise<AuthenticationUserLoginResponse> {
        const result = await this.runCommand('gcloud auth login --format="json"');
        return JSON.parse(result) as AuthenticationUserLoginResponse;
    }

    public static async list(): Promise<AuthenticationListItem[]> {
        const result = await this.runCommand('gcloud auth list --format="json"');
        return JSON.parse(result) as AuthenticationListItem[];
    }

    //https://cloud.google.com/sdk/gcloud/reference/auth/revoke

    private static runCommand(command: string): Promise<string> {

        // const command = 'gcloud auth list --format="json"';
        const commandOptions = {} as cp.ExecOptions;

        return new Promise((resolve, reject) => {

            cp.exec(command, commandOptions, (error, stdout, stderr) => {
                if (error) {
                    reject({ error, stdout, stderr });
                }

                resolve(stdout);
            });

        });

    }

}