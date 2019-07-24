import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {createLoadingSelector, RootState} from "app/reducers";
import {Result} from "app/reducers/search";
import {connect} from "react-redux";
import {bindActionCreators, Dispatch} from "redux";
import {omit} from "app/utils";
import {SearchActions} from "app/actions/search";

interface CompanySearchResult {
    result: Result
}

const Result: React.FC<CompanySearchResult> = props => (
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
    results: RootState.SearchResultsState
}

const CompanySearchResults: React.FC<CompanySearchResults> = props => (
    <div className={style.companySearchResults}>
        {props.results.data.map((each, i: number) =>
            <Result key={i} result={each} />
        )}
    </div>
);

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
    </div>
);

interface CompanySearchInput {
    search: (pat: string) => void,
}

const CompanySearchInput: React.FC<CompanySearchInput> = props => (
    <div className={style.companySearchInput}>
        <input
            autoFocus /* TODO only autofocus on the index/landing page */
            placeholder='Type a company name'
            type='text'
            onInput={async (event: React.ChangeEvent<HTMLInputElement>) => {
                const pat: string = event.target.value;
                props.search(pat);

            }}
        />
        <button>Search</button>
    </div>
);



interface CompanySearch {
    search: (pat: string) => void,
    results: RootState.SearchResultsState,
}

const CompanySearch: React.FC<CompanySearch> = props => (
    <div className={style.companySearch}>
        <CompanySearchInput
            search={props.search}
        />
        <CompanySearchResults
            results={props.results}
        />
    </div>
);

export namespace Header {
    export interface Props extends RouteComponentProps<void> {
        actions: SearchActions;
        isFetching: boolean;
        searchResults: RootState.SearchResultsState;
    }
}

const loadingSelector = createLoadingSelector([SearchActions.Type.SEARCH_COMPANY]);

@connect(
    (state: RootState, ownProps): Pick<Header.Props, 'isFetching' | 'searchResults'> => {
        return {
            isFetching: loadingSelector(state),
            searchResults: state.searchResults
        };
    },
    (dispatch: Dispatch): Pick<Header.Props, 'actions'> => ({
        actions: bindActionCreators(omit(SearchActions, 'Type'), dispatch)
    })
)

export class Header extends React.PureComponent<Header.Props> {
    constructor(props: Header.Props, context?: any) {
        super(props, context);
    }

    static defaultProps: Partial<Header.Props> = {};

    render() {
        return (
            <header className={style.headerBackground}>
                <div className={`${style.headerContents} container`}>
                    <Logo />
                    <CompanySearch
                        search={this.props.actions.searchCompany}
                        results={this.props.searchResults}
                    />
                </div>
            </header>
        )
    }
}
