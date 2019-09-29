import {ISignedUrlModel} from 'app/models/ISignedUrlModel';
import { http } from 'app/utils/http';
import { createActionCreator } from 'deox';
import { Dispatch } from 'redux';

export namespace SignedUrlActions {
  export enum Type {
    GET_SIGNED_FILING_URL = 'GET_SIGNED_FILING_URL',
    GET_SIGNED_FILING_URL_REQUEST = 'GET_SIGNED_FILING_URL_REQUEST',
    GET_SIGNED_FILING_URL_SUCCESS = 'GET_SIGNED_FILING_URL_SUCCESS',
    GET_SIGNED_FILING_URL_FAILURE = 'GET_SIGNED_FILING_URL_FAILURE',
  }

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
    failure: createActionCreator(Type.GET_SIGNED_FILING_URL_FAILURE, (resolve) => (error) =>
        resolve(error),
    ),
    request: createActionCreator(Type.GET_SIGNED_FILING_URL_REQUEST),
    success: createActionCreator(
        Type.GET_SIGNED_FILING_URL_SUCCESS,
        (resolve) => (signedUrl: ISignedUrlModel) => resolve(signedUrl),
    ),
  });
}

export type SignedUrlActions = Omit<typeof SignedUrlActions, 'Type'>;


