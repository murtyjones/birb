import { SignedUrlActions } from 'app/actions';
import { createReducer } from 'deox';
import { IRootState } from './state';

export const defaultCompanyState: IRootState.ISignedUrlState = {
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
