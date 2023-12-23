export enum ResultsGridRenderRequestV2Type {
  clear = "clear",
  executeQuery = "execute_query",
  error = "error"
}

export interface ResultsGridRenderRequestV2Error {
  message: String,
  reason: String | null
}

export interface ResultsGridRenderRequestV2 {
  requestType: String;
  projectId: String | null;
  token: String | null;
  query: String | null;
  error: ResultsGridRenderRequestV2Error | null;
}