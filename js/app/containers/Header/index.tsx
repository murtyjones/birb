import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';

const http = async (request: RequestInfo): Promise<SearchResponseResults> => {
    return new Promise(resolve => {
        fetch(request)
            .then(response => {
                console.log(response);
                return response.text()})
            .then(text => {
                resolve(text ? JSON.parse(text) : {});
            })
    });
};



interface SearchResponseResult {
    company_name: string;
    short_cik: string;
}


interface SearchResponseResults {
    data: Array<SearchResponseResult>;
    has_more: boolean;
}

const fakeResults: Array<SearchResponseResult> = [
    { company_name: 'wow', short_cik: '123456' },
    { company_name: 'yes', short_cik: '567890' },
];

interface Result {
    result: SearchResponseResult,
}

const Result: React.FC<Result> = props => (
    <Link to={`/companies/${props.result.short_cik}`} className={style.companySearchResult}>
        <span className={style.companyName}>
            {props.result.company_name}
        </span>
        <span className={style.shortCik}>
            CIK: {props.result.short_cik}
        </span>
    </Link>
);

interface CompanySearchResults {
    results: SearchResponseResults
}

const CompanySearchResults: React.FC<CompanySearchResults> = props => (
    <div className={style.companySearchResults}>
        {props.results.data.map((each: SearchResponseResult, i) =>
            <Result key={i} result={each} />
        )}
    </div>
);

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
    </div>
);

interface CompanySearch {
    results: SearchResponseResults,
    onRetrievedSearchResults: (results: SearchResponseResults) => void,
}

interface CompanySearchInput {
    onRetrievedSearchResults: (results: SearchResponseResults) => void,
}

const CompanySearchInput: React.FC<CompanySearchInput> = props => (
    <div className={style.companySearchInput}>
        <input
            autoFocus /* TODO only autofocus on the index/landing page */
            placeholder='Type a company name'
            type='text'
            onInput={async (event: React.ChangeEvent<HTMLInputElement>) => {
                const pat: string = event.target.value;
                const request = new Request(
                    `http://localhost:8000/api/autocomplete/${pat}`, {
                        method: 'GET'
                    });
                const result: SearchResponseResults = await http(request);
                props.onRetrievedSearchResults(result);

            }}
        />
        <button>Search</button>
    </div>
);

const CompanySearch: React.FC<CompanySearch> = props => (
    <div className={style.companySearch}>
        <CompanySearchInput
            onRetrievedSearchResults={props.onRetrievedSearchResults}
        />
        <CompanySearchResults
            results={props.results}
        />
    </div>
);

export namespace Header {
    export interface Props extends RouteComponentProps<void> {}
}

export class Header extends React.PureComponent<Header.Props> {
    constructor(props: Header.Props, context?: any) {
        super(props, context);
        this.setResults = this.setResults.bind(this);
    }
    readonly state = { results: { data: [], has_more: false } };

    static defaultProps: Partial<Header.Props> = {};

    setResults(value: SearchResponseResults): void {
        this.setState({ results: value });
    };

    render() {
        return (
            <header className={style.headerBackground}>
                <div className={`${style.headerContents} container`}>
                    <Logo />
                    <CompanySearch
                        results={this.state.results}
                        onRetrievedSearchResults={this.setResults}
                    />
                </div>
            </header>
        )
    }
}
