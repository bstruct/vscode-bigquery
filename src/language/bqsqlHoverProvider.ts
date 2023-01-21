import { CancellationToken, Hover, HoverProvider, Position, ProviderResult, TextDocument } from "vscode";
import { parse } from "@bstruct/bqsql-parser";
import { BqsqlDocument } from "./bqsqlDocument";

export class BqsqlHoverProvider implements HoverProvider{

    provideHover(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Hover> {
        
        if (document.languageId !== 'bqsql') { return; }
        
        const documentContent = document.getText();
        const parsed = parse(documentContent) as BqsqlDocument;


        throw new Error("let's leave this for later");
    }

}