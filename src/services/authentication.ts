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

    public static async serviceAccountLogin(filePath: string): Promise<AuthenticationUserLoginResponse> {
        const result = await this.runCommand(`gcloud auth activate-service-account --key-file=${filePath} --format="json"`);

        const typedResult = JSON.parse(result) as string[];
        if (typedResult.length === 0) {
            return { valid: true } as AuthenticationUserLoginResponse;
        }

        return { valid: false } as AuthenticationUserLoginResponse;
    }

    public static async list(): Promise<AuthenticationListItem[]> {
        const result = await this.runCommand('gcloud auth list --format="json"');
        return JSON.parse(result) as AuthenticationListItem[];
    }

    public static async activate(account: string): Promise<boolean> {
        const _ = await this.runCommand(`gcloud config set core/account "${account}" --format="json"`);
        return true;
    }

    public static async revoke(account: string): Promise<boolean> {
        const result = await this.runCommand(`gcloud auth revoke "${account}" --format="json"`);
        return (JSON.parse(result) as string[]).indexOf(account) >= 0;
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