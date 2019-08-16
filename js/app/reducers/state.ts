import { ICompanyModel } from 'app/models';
import {Result} from 'app/reducers/search';

export interface RootState {
  companies: RootState.CompanyState;
  loading: RootState.LoadingState;
  errors: RootState.ErrorsState;
  searchResults: RootState.SearchResultsState;
  router?: any;
}

export namespace RootState {
  export interface CompanyState { byShortCik: Record<ICompanyModel['shortCik'], ICompanyModel>; }
  export interface LoadingState { [requestName: string]: boolean; }
  export interface ErrorsState { [requestName: string]: boolean; }
  export interface SearchResultsState { data: Result[]; hasMore: boolean; }
}
