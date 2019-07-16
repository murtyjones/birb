import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';

interface SearchResponseResult {
    companyName: string;
    shortCik: string;
}

const fakeResults: Array<SearchResponseResult> = [
    { companyName: 'wow', shortCik: '123456' },
    { companyName: 'yes', shortCik: '567890' },
];

interface Result {
    result: SearchResponseResult,
}

const Result: React.FC<Result> = props => (
    <Link to='/hi' className={style.companySearchResult}>
        <span className={style.companyName}>
            {props.result.companyName}
        </span>
        <span className={style.shortCik}>
            CIK: {props.result.shortCik}
        </span>
    </Link>
);

const CompanySearchResults = () => (
    <div className={style.companySearchResults}>
        {fakeResults.map((each: SearchResponseResult, i) =>
            <Result key={i} result={each} />
        )}
    </div>
);

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
    </div>
);

const CompanySearch = () => (
    <div className={style.companySearch}>
        <CompanySearchInput/>
        <CompanySearchResults/>
    </div>
)

const CompanySearchInput = () => (
    <div className={style.companySearchInput}>
        <input
            autoFocus /* TODO only autofocus on the index/landing page */
            placeholder='Type a company name'
            type='text'
            onInput={async (event: React.ChangeEvent<HTMLInputElement>) => {
                console.log(event.target.value);
            }}
        />
        <button>Search</button>
    </div>
);

export namespace Header {
    export interface Props extends RouteComponentProps<void> {}
}

export class Header extends React.PureComponent<Header.Props> {
    static defaultProps: Partial<Header.Props> = {};

    render() {
        return (
            <header>
                <div className={style['header-background']}/>
                <div className={style['header-contents']}>
                    <Logo />
                    <CompanySearch />
                </div>
            </header>
        )
    }
}
