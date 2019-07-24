import { CompanyModel } from 'app/models';
import {Result} from "app/reducers/search";

export interface RootState {
  companies: RootState.CompanyState;
  loading: RootState.LoadingState;
  errors: RootState.ErrorsState;
  searchResults: RootState.SearchResultsState;
  router?: any;
}

export namespace RootState {
  export type CompanyState = { byShortCik: Record<CompanyModel['shortCik'], CompanyModel> };
  export type LoadingState = { [requestName: string]: boolean };
  export type ErrorsState = { [requestName: string]: boolean };
  export type SearchResultsState = { data: Result[], hasMore: boolean };
}
