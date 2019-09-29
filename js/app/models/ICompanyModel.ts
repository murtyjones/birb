import {IFilingModel} from 'app/models/IFilingModel';

export interface ICompanyModel {
  shortCik: string;
  name: string;
  filings: IFilingModel[];
}

