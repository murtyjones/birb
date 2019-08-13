import {IFilingModel} from 'app/models/IFilingModel';

export interface CompanyModel {
  shortCik: string;
  name: string;
  filings: IFilingModel[]
}

