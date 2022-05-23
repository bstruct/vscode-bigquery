
export interface SimpleQueryRowsResponseErrorItem {

    locationType: string;
    
    message: string;

    reason: string;

}

export interface SimpleQueryRowsResponseError {

    code: number;

    message: string;

    errors: SimpleQueryRowsResponseErrorItem[];

}