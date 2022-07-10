import { BqsqlDocument, BqsqlDocumentItem } from "./bqsqlDocument";

export class BqsqlParser {

    public static parse(bsqlStatement: string): BqsqlDocument {
        const items: BqsqlDocumentItem[] = [];

        return { items };
    }

}