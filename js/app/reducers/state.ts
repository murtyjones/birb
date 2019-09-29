import { ICompanyModel } from 'app/models';
import {Result} from 'app/reducers/search';

export interface IRootState {
  companies: IRootState.ICompanyState;
  loading: IRootState.ILoadingState;
  errors: IRootState.IErrorsState;
  searchResults: IRootState.ISearchResultsState;
  router?: any;
}

export namespace IRootState {
  export interface ICompanyState { byShortCik: Record<ICompanyModel['shortCik'], ICompanyModel>; }
  export interface ILoadingState { [requestName: string]: boolean; }
  export interface IErrorsState { [requestName: string]: boolean; }
  export interface ISearchResultsState { data: Result[]; hasMore: boolean; }
}
