import { RootState } from './state';
import { SearchActions } from "app/actions/search";
import { createReducer } from 'deox'

export interface Result {
    short_cik: string;
    company_name: string;
}

export const defaultSearchResultsState: RootState.SearchResultsState = {
    data: [], hasMore: false
};

export const searchReducer = createReducer(defaultSearchResultsState, handleAction => [
    handleAction(SearchActions.searchCompany.success, (state, { payload }) => {
        return {
            data: payload.data,
            hasMore: payload.has_more,
        };
    }),
]);
