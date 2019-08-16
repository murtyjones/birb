import { ICompanyModel } from 'app/models';
import {IFilingModel} from 'app/models/IFilingModel';
import {ISignedUrlModel} from 'app/models/ISignedUrlModel';
import { http } from 'app/utils/http';
import { createActionCreator } from 'deox';
import { Dispatch } from 'redux';

export namespace CompanyActions {
  export enum Type {
    GET_COMPANY = 'GET_COMPANY',
    GET_COMPANY_REQUEST = 'GET_COMPANY_REQUEST',
    GET_COMPANY_SUCCESS = 'GET_COMPANY_SUCCESS',
    GET_COMPANY_FAILURE = 'GET_COMPANY_FAILURE',


    GET_COMPANY_SIGNED_FILING_URL = 'GET_COMPANY_SIGNED_FILING_URL',
    GET_COMPANY_SIGNED_FILING_URL_REQUEST = 'GET_COMPANY_SIGNED_FILING_URL_REQUEST',
    GET_COMPANY_SIGNED_FILING_URL_SUCCESS = 'GET_COMPANY_SIGNED_FILING_URL_SUCCESS',
    GET_COMPANY_SIGNED_FILING_URL_FAILURE = 'GET_COMPANY_SIGNED_FILING_URL_FAILURE',
  }

  function fetchCompany(shortCik: string) {
    return async (dispatch: Dispatch) => {
      dispatch(getCompany.request());

      try {
        const request = new Request(`${process.env.BIRB_API_URL}/companies/${shortCik}/filings`, {
          method: 'GET',
        });
        const response = await http(request);

        dispatch(getCompany.success({
          filings: response.body.data.filings,
          name: response.body.data.company_name,
          shortCik: response.body.data.short_cik,
          signedUrl: null,
        }));
      } catch (error) {
        dispatch(getCompany.failure(error));
      }
    };
  }

  export const getCompany = Object.assign(fetchCompany, {
    failure: createActionCreator(Type.GET_COMPANY_FAILURE, (resolve) => (error) =>
        resolve(error),
    ),
    request: createActionCreator(Type.GET_COMPANY_REQUEST),
    success: createActionCreator(
        Type.GET_COMPANY_SUCCESS,
        (resolve) => (company: ICompanyModel) => resolve(company),
    ),
  });

  function fetchSignedUrl(shortCik: string, filingId: string) {
    return async (dispatch: Dispatch) => {
      dispatch(getSignedUrl.request());

      try {
        const request = new Request(
            `${process.env.BIRB_API_URL}/companies/${shortCik}/filings/${filingId}/raw-s3-link`, {
          method: 'GET',
        });
        const response = await http(request);

        dispatch(getSignedUrl.success({
          filingId,
          shortCik,
          signedUrl: response.body.data.signed_url,
        }));
      } catch (error) {
        dispatch(getSignedUrl.failure(error));
      }
    };
  }

  export const getSignedUrl = Object.assign(fetchSignedUrl, {
    failure: createActionCreator(Type.GET_COMPANY_SIGNED_FILING_URL_FAILURE, (resolve) => (error) =>
        resolve(error),
    ),
    request: createActionCreator(Type.GET_COMPANY_SIGNED_FILING_URL_REQUEST),
    success: createActionCreator(
        Type.GET_COMPANY_SIGNED_FILING_URL_SUCCESS,
        (resolve) => (signedUrl: ISignedUrlModel) => resolve(signedUrl),
    ),
  });
}

export type CompanyActions = Omit<typeof CompanyActions, 'Type'>;


