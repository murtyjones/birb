import {FilingModel} from 'app/models/FilingModel';

export interface CompanyModel {
  shortCik: string;
  name: string;
  filings: FilingModel[]
}

