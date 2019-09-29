import { SearchActions } from 'app/actions/search';
import { createReducer } from 'deox';
import { IRootState } from './state';

export interface Result {
    short_cik: string;
    company_name: string;
}

export const defaultSearchResultsState: IRootState.SearchResultsState = {
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
