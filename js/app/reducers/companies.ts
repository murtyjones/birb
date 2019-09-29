import { CompanyActions } from 'app/actions/companies';
import { createReducer } from 'deox';
import { ICompanyState } from './state';

export const defaultCompanyState: ICompanyState = {
    byShortCik: {},
};

export const companyReducer = createReducer(defaultCompanyState, (handleAction) => [
    handleAction(CompanyActions.getCompanyWithFilings.success, (state, { payload }) => {
        const newState = Object.assign(state, {
            byShortCik: {
                ...state.byShortCik,
                [payload.shortCik]: payload,
            },
        });
        return newState;
    }),
]);
