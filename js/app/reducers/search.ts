import { SearchActions } from 'app/actions/search';
import { createReducer } from 'deox';
import { ISearchResultsState } from './state';

export interface ISearchResult {
    short_cik: string;
    company_name: string;
}

export const defaultSearchResultsState: ISearchResultsState = {
    data: [], hasMore: false,
};

export const searchReducer = createReducer(defaultSearchResultsState, (handleAction) => [
    handleAction(SearchActions.searchCompany.success, (state, { payload }) => {
        return {
            data: payload.data,
            hasMore: payload.has_more,
        };
    }),
]);
