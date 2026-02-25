import { CancellationToken, Hover, HoverProvider, Position, ProviderResult, TextDocument } from "vscode";

export class BqsqlHoverProvider implements HoverProvider {

    provideHover(_document: TextDocument, _position: Position, _token: CancellationToken): ProviderResult<Hover> {
        // Not yet implemented
        return undefined;
    }

}
