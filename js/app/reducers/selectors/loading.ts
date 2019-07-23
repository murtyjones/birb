import { RootState } from 'app/reducers';
/**
 * Return `true` when all actions are not loading.
 *
 * @param actions
 * @returns {Function}
 */
export const createLoadingSelector = (actions: string[]) => (state: RootState) => {
    return actions.reduce((acc, each) => {
        if (!state.loading[each]) {
            acc = false;
        }
        return acc;
    }, true);
};
