import { Action } from 'redux';
import { IRootState } from './state';

export const defaultLoadingState: IRootState.ILoadingState = {

};

export const errorsReducer = (state = defaultLoadingState, action: Action) => {
    const { type } = action;
    const matches = /(.*)_(REQUEST|SUCCESS|FAILURE)/.exec(type);

    // not a *_REQUEST / *_SUCCESS /  *_FAILURE actions, so we ignore them
    if (!matches) { return state; }

    const [, requestName, requestState] = matches;
    return {
        ...state,
        // Store whether a request has failed
        // e.g. will be true when receiving GET_TODOS_FAILURE
        //      and false when receiving GET_TODOS_REQUEST / GET_TODOS_SUCCESS
        [requestName]: requestState === 'FAILED',
    };
};
