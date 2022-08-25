import { CancellationToken, Event, InlayHint, InlayHintsProvider, Position, ProviderResult, Range, TextDocument } from "vscode";


export class BqsqlInlayHintsProvider implements InlayHintsProvider<InlayHint>{

    onDidChangeInlayHints?: Event<void> | undefined;
    provideInlayHints(document: TextDocument, range: Range, token: CancellationToken): ProviderResult<InlayHint[]> {

        return [];
        //     {
        //     label: "label123",
        //     position: new Position(0, 33),

        // } as InlayHint

    }

}