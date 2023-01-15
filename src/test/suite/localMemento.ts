import * as vscode from 'vscode';

// interface LocalMementoItem {
//     [key: string]: number;
// }

export class LocalMemento implements vscode.Memento {

    local: Map<string, any> = new Map<string, any>();

    keys(): readonly string[] {
        return [...this.local.keys()];
    }

    get<T>(key: string): T | undefined { return this.local.get(key) as T | undefined; }

    // get<T>(key: string, defaultValue: T): T {
    //     throw new Error('Method not implemented.');
    // }

    update(key: string, value: any): Thenable<void> {

        this.local.set(key, value);

        return new Promise((resolve, reject) => { resolve(); });
    }

}