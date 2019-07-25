import {SearchActions} from 'app/actions/search';
import * as style from 'app/containers/Header/style.css';
import {RootState} from 'app/reducers';
import {Result} from 'app/reducers/search';
import * as React from 'react';
import {Link, RouteComponentProps} from 'react-router-dom';

interface ICompanySearchResult {
    result: Result;
    isActive: boolean;
}

const Result: React.FC<ICompanySearchResult> = (props) => (
    <Link
        to={`/companies/${props.result.short_cik}`}
        className={style.companySearchResult}
        style={props.isActive ? {background: 'red'} : {}}
    >
        <span className={style.companyName}>
            {props.result.company_name}
        </span>
        <span className={style.shortCik}>
            CIK: {props.result.short_cik}
        </span>
    </Link>
);

interface ICompanySearchResults {
    results: RootState.SearchResultsState;
    activeIndex: number;
}

const CompanySearchResults: React.FC<ICompanySearchResults> = (props) => (
    <div className={style.companySearchResults}>
        {props.results.data.map((each, i: number) =>
            <Result key={i} isActive={i === props.activeIndex} result={each} />,
        )}
    </div>
);

interface ICompanySearchInput {
    handleInput: (pat: string) => void;
    handleBlur: () => void;
    handleKeyDown: (event: React.KeyboardEvent<HTMLInputElement>) => void;
    isInputActive: boolean;
}

const CompanySearchInput: React.FC<ICompanySearchInput> = (props) => (
    <div className={style.companySearchInput}>
        <input
            autoFocus={props.isInputActive} /* TODO only autofocus on the index/landing page */
            placeholder='Type a company name'
            type='text'
            onInput={async (event: React.ChangeEvent<HTMLInputElement>) => {
                const pat: string = event.target.value;
                props.handleInput(pat);
            }}
            onBlur={() => props.handleBlur()}
            onKeyDown={props.handleKeyDown}
        />
        <button>Search</button>
    </div>
);

export namespace CompanySearch {
    export interface IProps {
        handleInput: (pat: string) => void;
        results: RootState.SearchResultsState;
    }
}

interface IState {
    activeIndex: -1|0|1|2|3|4|5|6|7|8|9;
    isInputActive: boolean;
}

export class CompanySearch extends React.PureComponent<CompanySearch.IProps> {
    public state: Readonly<IState> = {
        activeIndex: -1, // -1 indicates no active item
        isInputActive: true,
    };
    constructor(props: CompanySearch.IProps, context?: any) {
        super(props, context);
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleInput = this.handleInput.bind(this);
        this.handleClick = this.handleClick.bind(this);
        this.handleBlur = this.handleBlur.bind(this);
        this.forceBlur = this.forceBlur.bind(this);
        this.navigate = this.navigate.bind(this);
    }

    public handleKeyDown(event: React.KeyboardEvent<HTMLInputElement>) {
        const navUpKeys = {
            ArrowUp: true,
            Up: true,
        };
        const navDownKeys = {
            ArrowDown: true,
            Down: true,
        };
        const selectKeys = {
            Enter: true,
        };
        const blurKeys = {
            Esc: true,
            Escape: true,
        };

        if ((navUpKeys as any)[event.key]) {
            this.navigate(1);
        } else if ((navDownKeys as any)[event.key]) {
            this.navigate(-1);
        } else if ((blurKeys as any)[event.key]) {
            this.forceBlur();
        } else if ((selectKeys as any)[event.key]) {

        }
    }

    public handleInput(pat: string) {
        this.setState({ isInputActive: true });
        this.props.handleInput(pat);
    }

    public handleClick(e) {
        this.setState({ isInputActive: true });
    }

    public handleBlur() {
        this.setState({ isInputActive: false });
    }

    public forceBlur() {
        this.setState({ isInputActive: false });
    }

    public navigate(direction: -1|1) {
        if (this.props.results.data.length > 0) {
            const jumpToTopOfList = this.state.activeIndex === this.props.results.data.length - 1 && direction < 0;
            const jumpToEndOfList = this.state.activeIndex === 0 && direction > 0;
            this.setState({
                activeIndex:
                    jumpToTopOfList
                        ? 0
                        : jumpToEndOfList
                        ? this.props.results.data.length - 1
                        : this.state.activeIndex - direction,

            });
        } else {
            this.setState({
                activeIndex: -1,
            });
        }
    }

    public render() {
        console.log(this.state.activeIndex);
        return (
            <div className={style.companySearch}>
                <CompanySearchInput
                    handleInput={this.handleInput}
                    handleClick={this.handleClick}
                    handleBlur={this.handleBlur}
                    handleKeyDown={this.handleKeyDown}
                    isInputActive={this.state.isInputActive}
                />
                {
                    this.state.isInputActive &&
                        <CompanySearchResults
                            results={this.props.results}
                            activeIndex={this.state.activeIndex}
                        />
                }
            </div>
        );
    }
}
