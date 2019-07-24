import { combineReducers } from 'redux';
import { RootState } from './state';
import { companyReducer } from './companies';
import { loadingReducer } from './loading';
import { errorsReducer } from 'app/reducers/errors';
import { searchReducer } from 'app/reducers/search';

export * from './selectors';

export { RootState };

// NOTE: current type definition of Reducer in 'redux-actions' module
// doesn't go well with redux@4
export const rootReducer = combineReducers<RootState>({
  companies: companyReducer as any,
  loading: loadingReducer as any,
  errors: errorsReducer as any,
  searchResults: searchReducer as any,
});
