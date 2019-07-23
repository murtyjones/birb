import { RootState } from './state';
import { CompanyActions } from 'app/actions/companies';
import { createReducer } from 'deox'

export const defaultCompanyState: RootState.CompanyState = {
    byShortCik: {}
};

export const companyReducer = createReducer(defaultCompanyState, handleAction => [
    handleAction(CompanyActions.getCompany.success, (state, { payload }) => {
        const newState = Object.assign(state, {
            byShortCik: {
                ...state.byShortCik,
                [payload.shortCik]: payload
            }
        });
        return newState;
    }),
]);
