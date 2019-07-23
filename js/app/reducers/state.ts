import { TodoModel, CompanyModel } from 'app/models';

export interface RootState {
  todos: RootState.TodoState;
  companies: RootState.CompanyState;
  loading: RootState.LoadingState;
  router?: any;
}

export namespace RootState {
  export type TodoState = TodoModel[];
  export type CompanyState = { byShortCik: Record<CompanyModel['shortCik'], CompanyModel> };
  export type LoadingState = { [requestName: string]: boolean };
}
