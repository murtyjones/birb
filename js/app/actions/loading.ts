import { createActionCreator } from 'deox'

export namespace LoadingActions {
  export enum Type {
    REQUEST = 'REQUEST',
    SUCCESS = 'SUCCESS',
    FAILURE = 'FAILURE',
  }

  // success('GET_COMPANY') //=> { type: 'SUCCESS', payload: 'GET_COMPANY' }

  export const request = createActionCreator(Type.REQUEST,
      resolve => (requestName: string) => resolve(requestName)
  );

  export const success = createActionCreator(Type.SUCCESS,
      resolve => (requestName: string) => resolve(requestName)
  );

  export const failure = createActionCreator(Type.FAILURE,
      resolve => (requestName: string) => resolve(requestName)
  );
}

export type LoadingActions = Omit<typeof LoadingActions, 'Type'>;


