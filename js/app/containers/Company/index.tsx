import * as React from 'react';
// import * as style from './style.css';
import {RouteComponentProps} from 'react-router';

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
    readonly state = { results: { company_name: '', filings: [], short_cik: '' } };

    setFilingData(result: CompanyFilingDataResponse) {
        this.setState({
            results: result.data
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
        const results: Array<any> = this.state.results.filings || [];
        return (
            <div>
                { results.length > 0
                    ? results.map(each => <div>{each.filing_name}</div>)
                    : 'Hello!'
                }
            </div>
        )
    }
}
