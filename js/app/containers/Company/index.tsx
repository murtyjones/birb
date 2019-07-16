import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';

interface GetCompanyResult {
    company_name: string;
    short_cik: string;
}

const http = async (request: RequestInfo): Promise<GetCompanyResult> => {
    return new Promise(resolve => {
        fetch(request)
            .then(response => {
                return response.text()})
            .then(text => {
                resolve(text ? JSON.parse(text) : {});
            })
    });
};

interface MatchParams {
    short_cik: string;
}

export namespace Company {
    export interface Props extends RouteComponentProps<MatchParams> {}
}

export class Company extends React.PureComponent<Company.Props> {
    async componentDidMount() {
        const short_cik = this.props.match.params.short_cik;
        const request = new Request(`http://localhost:8000/api/companies/${short_cik}`, {
            method: 'GET'
        });
        const r = await http(request);
        console.log(r);
    }

    render() {
        return (
            <div>
                Hello!
            </div>
        )
    }
}
