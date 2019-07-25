import {http} from "app/utils/http";
import {Dispatch} from "redux";
import {createActionCreator} from "deox";
import {Result} from "app/reducers/search";

export interface SearchResults {
    data: Result[];
    has_more: boolean;
}

export namespace SearchActions {
    export enum Type {
        SEARCH_COMPANY = 'SEARCH_COMPANY',
        SEARCH_COMPANY_REQUEST = 'SEARCH_COMPANY_REQUEST',
        SEARCH_COMPANY_SUCCESS = 'SEARCH_COMPANY_SUCCESS',
        SEARCH_COMPANY_FAILURE = 'SEARCH_COMPANY_FAILURE',
    }

    function fetchCompanySearchResults(pat: string) {
        return async (dispatch: Dispatch) => {
            // If no query, no results to show:
            if (pat === '') {
                dispatch(searchCompany.success({ data: [], has_more: false }));
            }
            dispatch(searchCompany.request());
            try {
                const request = new Request(`http://localhost:8000/api/autocomplete/${pat}`, {
                    method: 'GET'
                });
                const response = await http(request);

                const results = {
                    data: response.body.data,
                    has_more: response.body.has_more,
                };

                dispatch(searchCompany.success(results));
            } catch (error) {
                dispatch(searchCompany.failure(error));
            }
        }
    }

    export const searchCompany = Object.assign(fetchCompanySearchResults, {
        request: createActionCreator(Type.SEARCH_COMPANY_REQUEST),
        success: createActionCreator(
            Type.SEARCH_COMPANY_SUCCESS,
            resolve => (results: SearchResults) => resolve(results)
        ),
        failure: createActionCreator(Type.SEARCH_COMPANY_FAILURE, resolve => error =>
            resolve(error)
        ),
    })
}

export type SearchActions = Omit<typeof SearchActions, 'Type'>;
