declare enum ResultsGridRenderRequestV2Type {
    ExecuteQuery = 1,
  }

export interface ResultsGridRenderRequestV2 {
    requestType: ResultsGridRenderRequestV2Type;
    maxResults: number;
}