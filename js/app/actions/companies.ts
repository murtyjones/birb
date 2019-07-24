import { Dispatch } from 'redux';
import { createActionCreator } from 'deox'
import { CompanyModel } from 'app/models';
import { http } from 'app/utils/http';

export namespace CompanyActions {
  export enum Type {
    GET_COMPANY = 'GET_COMPANY',
    GET_COMPANY_REQUEST = 'GET_COMPANY_REQUEST',
    GET_COMPANY_SUCCESS = 'GET_COMPANY_SUCCESS',
    GET_COMPANY_FAILURE = 'GET_COMPANY_FAILURE',
  }


  function fetchCompany(shortCik: string) {
    return async (dispatch: Dispatch) => {
      dispatch(getCompany.request());

      try {
        const request = new Request(`http://localhost:8000/api/companies/${shortCik}/filings`, {
          method: 'GET'
        });
        const response = await http(request);

        dispatch(getCompany.success({
          shortCik: response.body.data.short_cik,
          name: response.body.data.company_name,
        }));
      } catch (error) {
        dispatch(getCompany.failure(error));
      }
    }
  }

  export const getCompany = Object.assign(fetchCompany, {
    request: createActionCreator(Type.GET_COMPANY_REQUEST),
    success: createActionCreator(
        Type.GET_COMPANY_SUCCESS,
        resolve => (company: CompanyModel) => resolve(company)
    ),
    failure: createActionCreator(Type.GET_COMPANY_FAILURE, resolve => error =>
        resolve(error)
    ),
  })
}

export type CompanyActions = Omit<typeof CompanyActions, 'Type'>;


