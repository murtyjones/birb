import { logger } from 'app/middleware';
import { rootReducer, IRootState } from 'app/reducers';
import { applyMiddleware, createStore, Store } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';
import thunk from 'redux-thunk';

export function configureStore(initialState?: IRootState): Store<IRootState> {
  let middleware = applyMiddleware(thunk, logger);

  if (process.env.NODE_ENV !== 'production') {
    middleware = composeWithDevTools(middleware);
  }

  const store = createStore(
      rootReducer as any,
      initialState as any,
      middleware) as Store<
    IRootState
  >;

  if (module.hot) {
    module.hot.accept('app/reducers', () => {
      const nextReducer = require('app/reducers');
      store.replaceReducer(nextReducer);
    });
  }

  return store;
}
