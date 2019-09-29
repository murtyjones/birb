import { CompanyActions } from 'app/actions/companies';
import { createReducer } from 'deox';
import { IRootState } from './state';

export const defaultCompanyState: IRootState.CompanyState = {
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
    handleAction(CompanyActions.getSignedUrl.success, (state, { payload }) => {
        const newState = Object.assign(state);
        newState.byShortCik[payload.shortCik].signedUrl = payload.signedUrl;
        return newState;
    }),
]);
