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
    GET_COMPANY = 'GET_COMPANY',
  }

  export const getCompany = createAction (Type.GET_COMPANY, async (shortCik: string): Promise<CompanyModel> => {
    const request = new Request(`http://localhost:8000/api/companies/${shortCik}/filings`, {
      method: 'GET'
    });
    const result: CompanyFilingDataResponse = await http(request);
    return { shortCik: result.data.short_cik, name: result.data.company_name };
  });

}

export type CompanyActions = Omit<typeof CompanyActions, 'Type'>;


