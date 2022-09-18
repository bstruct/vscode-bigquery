import { Event, EventEmitter, Pseudoterminal, TerminalDimensions } from "vscode";
import * as cp from 'child_process';

export class CustomTerminal implements Pseudoterminal {

    private writeEmitter = new EventEmitter<string>();

    constructor() {

        // this.writeEmitter.event((e: string) => {
        //     if (this.pressKeyToClose) {
        //         this.close();
        //     }
        // });

        this.onDidWrite = this.writeEmitter.event;
    }

    onDidWrite: Event<string>;
    onDidOverrideDimensions?: Event<TerminalDimensions | undefined> | undefined;
    onDidClose?: Event<number | void> | undefined;
    onDidChangeName?: Event<string> | undefined;
    open(initialDimensions: TerminalDimensions | undefined): void {
    }
    close(): void {
    }
    handleInput?(data: string): void {

        const lines = data.split('\r');
        for (let index = 0; index < lines.length; index++) {
            const line = lines[index];
            
            this.writeEmitter.fire(line);
            this.writeEmitter.fire("\r\n");

        }
    }
    setDimensions?(dimensions: TerminalDimensions): void {
    }

}