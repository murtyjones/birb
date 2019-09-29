import { errorsReducer } from 'app/reducers/errors';
import { searchReducer } from 'app/reducers/search';
import { combineReducers } from 'redux';
import { companyReducer } from './companies';
import { loadingReducer } from './loading';
import { signedUrlsReducer } from './signedUrls';
import { IRootState } from './state';

export * from './selectors';

export { IRootState };

// NOTE: current type definition of Reducer in 'redux-actions' module
// doesn't go well with redux@4
export const rootReducer = combineReducers<IRootState>({
  companies: companyReducer as any,
  errors: errorsReducer as any,
  loading: loadingReducer as any,
  searchResults: searchReducer as any,
  signedUrls: signedUrlsReducer as any,
});
