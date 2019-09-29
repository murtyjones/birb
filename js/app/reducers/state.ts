import { ICompanyModel } from 'app/models';
import {Result} from 'app/reducers/search';

export interface IRootState {
  companies: IRootState.CompanyState;
  loading: IRootState.LoadingState;
  errors: IRootState.ErrorsState;
  searchResults: IRootState.SearchResultsState;
  router?: any;
}

export namespace IRootState {
  export interface CompanyState { byShortCik: Record<ICompanyModel['shortCik'], ICompanyModel>; }
  export interface LoadingState { [requestName: string]: boolean; }
  export interface ErrorsState { [requestName: string]: boolean; }
  export interface SearchResultsState { data: Result[]; hasMore: boolean; }
}
