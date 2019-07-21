import { Dispatch } from 'redux';
import { createAction } from 'redux-actions';
import { CompanyModel } from 'app/models';

interface CompanyFilingData {
  company_name: string;
  filings: Array<any>;
  short_cik: string;
}

interface CompanyFilingDataResponse {
  data: CompanyFilingData
}

const http = async (request: RequestInfo): Promise<CompanyFilingDataResponse> => {
  return new Promise(resolve => {
    fetch(request)
        .then(response => {
          return response.text()})
        .then(text => {
          resolve(text ? JSON.parse(text) : {});
        })
  });
};

export namespace CompanyActions {
  export enum Type {
    GET_COMPANY_REQUEST = 'GET_COMPANY_REQUEST',
    GET_COMPANY_SUCCESS = 'GET_COMPANY_SUCCESS',
    GET_COMPANY_FAILURE = 'GET_COMPANY_FAILURE',
  }

  const getCompanyRequest = createAction('GET_COMPANY_REQUEST');
  const getCompanySuccess = createAction<CompanyModel>('GET_COMPANY_SUCCESS');
  const getCompanyFailure = createAction('GET_COMPANY_FAILURE');

  export const getCompany = (shortCik: string) => async (dispatch: Dispatch) => {
    dispatch(getCompanyRequest());
    try {
      const request = new Request(`http://localhost:8000/api/companies/${shortCik}/filings`, {
        method: 'GET'
      });
      const result: CompanyFilingDataResponse = await http(request);
      dispatch(getCompanySuccess({
        shortCik: result.data.short_cik, name: result.data.company_name
      }));
    } catch (e) {
      dispatch(getCompanyFailure());
    }
  };
}

export type CompanyActions = Omit<typeof CompanyActions, 'Type'>;


