const { shallow } = require('enzyme');
import { companyReducer } from 'app/reducers/companies';
import { CompanyActions } from 'app/actions/companies';

describe('Companies Reducer', () => {
    it('Adds a company to empty state on successful retrieval', () => {
        // Arrange
        const action = {
            type: CompanyActions.Type.GET_COMPANY_SUCCESS as CompanyActions.Type.GET_COMPANY_SUCCESS,
            payload: { shortCik: '123', name: 'Tezzla', filings: [] }
        };

        // Act
        const newState = companyReducer(undefined, action);


        // Assert
        expect(newState).toEqual({
            byShortCik: {
                '123': { shortCik: '123', name: 'Tezzla', filings: [] }
            }
        })
    });

    it('Adds a company to existing state on successful retrieval', () => {
        // Arrange
        const initialState = {
            byShortCik: {
                '000': { shortCik: '000', name: 'NotTezzla', filings: [] }
            }
        };
        const action = {
            type: CompanyActions.Type.GET_COMPANY_SUCCESS as CompanyActions.Type.GET_COMPANY_SUCCESS,
            payload: { shortCik: '123', name: 'Tezzla', filings: [] }
        };

        // Act
        const newState = companyReducer(initialState, action);


        // Assert
        expect(newState).toEqual({
            byShortCik: {
                '000': { shortCik: '000', name: 'NotTezzla' },
                '123': { shortCik: '123', name: 'Tezzla', filings: [] }
            }
        })
    });
});
