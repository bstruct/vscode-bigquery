export enum ResultsGridRenderRequestV2Type {
  clear = "clear",
  executeQuery = "execute_query",
}

export interface ResultsGridRenderRequestV2 {
  requestType: String;
  projectId: String | null;
  token: String | null;
  query: String | null;
}