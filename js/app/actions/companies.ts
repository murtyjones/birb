import { ICompanyModel } from 'app/models';
import { http } from 'app/utils/http';
import { createActionCreator } from 'deox';
import { Dispatch } from 'redux';

export namespace CompanyActions {
  export enum Type {
    GET_COMPANY = 'GET_COMPANY',
    GET_COMPANY_REQUEST = 'GET_COMPANY_REQUEST',
    GET_COMPANY_SUCCESS = 'GET_COMPANY_SUCCESS',
    GET_COMPANY_FAILURE = 'GET_COMPANY_FAILURE',
  }

  function fetchCompany(shortCik: string) {
    return async (dispatch: Dispatch) => {
      dispatch(getCompanyWithFilings.request());

      try {
        const request = new Request(`${process.env.BIRB_API_URL}/companies/${shortCik}/filings`, {
          method: 'GET',
        });
        const response = await http(request);

        dispatch(getCompanyWithFilings.success({
          filings: response.body.data.filings,
          name: response.body.data.company_name,
          shortCik: response.body.data.short_cik,
        }));
      } catch (error) {
        dispatch(getCompanyWithFilings.failure(error));
      }
    };
  }

  export const getCompanyWithFilings = Object.assign(fetchCompany, {
    failure: createActionCreator(Type.GET_COMPANY_FAILURE, (resolve) => (error) =>
        resolve(error),
    ),
    request: createActionCreator(Type.GET_COMPANY_REQUEST),
    success: createActionCreator(
        Type.GET_COMPANY_SUCCESS,
        (resolve) => (company: ICompanyModel) => resolve(company),
    ),
  });
}

export type CompanyActions = Omit<typeof CompanyActions, 'Type'>;


