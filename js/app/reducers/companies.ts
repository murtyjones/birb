import { handleActions } from 'redux-actions';
import { RootState } from './state';
import { CompanyActions } from 'app/actions/companies';
import { CompanyModel } from 'app/models';

const initialState: RootState.CompanyState = {
    byShortCik: {}
};

export const companyReducer = handleActions<RootState.CompanyState, CompanyModel>(
    {
        [CompanyActions.Type.GET_COMPANY]: (state, action) => {
            const c = action.payload;
            return state;
        },
    },
    initialState
);
