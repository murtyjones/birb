import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';

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

interface CompanyFilingData {
    company_name: string;
    filings: Array<any>;
    short_cik: string;
}

interface CompanyFilingDataResponse {
    data: CompanyFilingData
}

interface MatchParams {
    shortCik: string;
}

export namespace Company {
    export interface Props extends RouteComponentProps<MatchParams> {}
}

export class Company extends React.PureComponent<Company.Props> {
    constructor(props: Company.Props, context?: any) {
        super(props, context);
        this.setFilingData = this.setFilingData.bind(this);
    }
    readonly state = { allFilings: { company_name: '', filings: [], short_cik: '' } };

    setFilingData(result: CompanyFilingDataResponse) {
        this.setState({
            allFilings: result.data
        })
    }

    async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        const request = new Request(`http://localhost:8000/api/companies/${shortCik}/filings`, {
            method: 'GET'
        });
        const result: CompanyFilingDataResponse = await http(request);
        console.log(result);
        this.setFilingData(result)
    }

    render() {
        return (
            <div className={`${style.mainCompanyContents} container`}>
                { this.state.allFilings.filings.length > 0
                    ? AllFilingsTable(this.state.allFilings)
                    : 'Hello!'
                }
            </div>
        )
    }
}

const AllFilingsTable = (allFilings: CompanyFilingData) => (
    <div className={style.allFilingsTable}>

        {
            allFilings.filings.map(each =>
                <Link to={`/filing?bucket=birb-edgar-filings&key=${each.filing_edgar_url}`}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                </Link>
            )
        }
    </div>
);
