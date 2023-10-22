export enum ResultsGridRenderRequestV2Type {
  executeQuery = 1,
}

export interface ResultsGridRenderRequestV2 {
  requestType: String;
  projectId: String;
  token: String;
  query: String;
}