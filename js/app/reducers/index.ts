import { combineReducers } from 'redux';
import { RootState } from './state';
import { todoReducer } from './todos';
import { companyReducer } from './companies';
import { loadingReducer } from './loading';

export * from './selectors';

export { RootState };

// NOTE: current type definition of Reducer in 'redux-actions' module
// doesn't go well with redux@4
export const rootReducer = combineReducers<RootState>({
  todos: todoReducer as any,
  companies: companyReducer as any,
  loading: loadingReducer as any,
});
