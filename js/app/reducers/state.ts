import {ICompanyModel , IFilingModel, ISignedUrlModel} from 'app/models';
import {ISearchResult} from 'app/reducers/search';

export interface IRootState {
  companies: ICompanyState;
  loading: ILoadingState;
  errors: IErrorsState;
  searchResults: ISearchResultsState;
  signedUrls: ISignedUrlState;
  router?: any;
}

export interface ICompanyState { byShortCik: Record<ICompanyModel['shortCik'], ICompanyModel>; }
export interface ILoadingState { [requestName: string]: boolean; }
export interface IErrorsState { [requestName: string]: boolean; }
export interface ISearchResultsState { data: ISearchResult[]; hasMore: boolean; }
export interface ISignedUrlState { byFilingId: Record<IFilingModel['id'], ISignedUrlModel>; }
