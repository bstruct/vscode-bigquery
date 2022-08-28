export interface BigqueryJobError {
    code: number,
    errors: BigqueryJobErrorItem[]
    message: string
}

export interface BigqueryJobErrorItem {
    domain: string,
    location: string,
    locationType: string,
    message: string,
    reason: string
}