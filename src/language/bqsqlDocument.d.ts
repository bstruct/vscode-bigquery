export interface BqsqlDocument {

    items: BqsqlDocumentItem[];
}

export interface BqsqlDocumentItem {
    // eslint-disable-next-line @typescript-eslint/naming-convention
    item_type: string;
    range: number[];
    items: BqsqlDocumentItem[]
}