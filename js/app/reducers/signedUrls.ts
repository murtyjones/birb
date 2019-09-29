import { SignedUrlActions } from 'app/actions';
import { createReducer } from 'deox';
import { ISignedUrlState } from './state';

export const defaultCompanyState: ISignedUrlState = {
    byFilingId: {},
};

export const signedUrlsReducer = createReducer(defaultCompanyState, (handleAction) => [
    handleAction(SignedUrlActions.getSignedUrl.success, (state, { payload }) => {
        const newState = Object.assign(state, {
            byFilingId: {
                ...state.byFilingId,
                [payload.filingId]: payload,
            },
        });
        return newState;
    }),
]);
