import { CompanyModel } from 'app/models';

export interface RootState {
  companies: RootState.CompanyState;
  loading: RootState.LoadingState;
  router?: any;
}

export namespace RootState {
  export type CompanyState = { byShortCik: Record<CompanyModel['shortCik'], CompanyModel> };
  export type LoadingState = { [requestName: string]: boolean };
}
