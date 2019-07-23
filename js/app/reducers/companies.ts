import { RootState } from './state';
import { CompanyActions } from 'app/actions/companies';
import { createReducer } from 'deox'

export const defaultTodosState: RootState.CompanyState = {
    byShortCik: {}
};

export const companyReducer = createReducer(defaultTodosState, handleAction => [
    handleAction(CompanyActions.getCompany.success, (state, { payload }) => {
        const newState = Object.assign(state, {
            byShortCik: {
                ...state.byShortCik,
                [payload.shortCik]: payload
            }
        });
        console.log(newState);
        return newState;
    }),
]);
