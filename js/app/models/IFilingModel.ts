import * as moment from 'moment';

export interface IFilingModel {
  collected: boolean;
  company_short_cik: string;
  filing_edgar_url: string;
  filing_name: string;
  date_filed: moment.Moment;
  filing_quarter: number;
  filing_year: number;
  id: number;
  created_at: moment.Moment;
  updated_at: moment.Moment;
}

