import * as style from 'app/containers/Header/style.css';
import {RootState} from 'app/reducers';
import {Result} from 'app/reducers/search';
import cns from 'classnames';
import { History } from 'history';
import * as React from 'react';
import {Link} from 'react-router-dom';


const getCompanyWithFilingsLink = (shortCik: string) => {
    return `/companies/${shortCik}`;
};

interface ICompanySearchResult {
    onResultClick: (e: React.MouseEvent) => void;
    result: Result;
    isActive: boolean;
}

const Result: React.FC<ICompanySearchResult> = (props) => (
    <Link
        to={getCompanyWithFilingsLink(props.result.short_cik)}
        data-short-cik={props.result.short_cik}
        onClick={props.onResultClick}
        className={cns(style.companySearchResult, {
            [style.activeResult]: props.isActive,
        })}
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
    onResultClick: (e: React.MouseEvent) => void;
    results: RootState.SearchResultsState;
    activeIndex: number;
    history: History;
}

const CompanySearchResults: React.FC<ICompanySearchResults> = (props) => (
    <div className={cns(style.companySearchResults)}>
        {props.results.data.map((each, i: number) =>
            <Result
                onResultClick={props.onResultClick}
                key={i}
                isActive={i === props.activeIndex}
                result={each}
            />,
        )}
    </div>
);

interface ICompanySearchInput {
    handleInput: (pat: string) => void;
    handleInputClick: () => void;
    handleKeyDown: (event: React.KeyboardEvent<HTMLInputElement>) => void;
    isInputActive: boolean;
    inputValue: string;
}

const CompanySearchInput: React.FC<ICompanySearchInput> = (props) => (
    <div className={cns(style.companySearchInput)}>
        <input
            value={props.inputValue}
            className={cns('companySearchInput')}
            autoFocus={props.isInputActive} /* TODO only autofocus on the index/landing page */
            placeholder='Type a company name'
            type='text'
            onInput={async (event: React.ChangeEvent<HTMLInputElement>) => {
                const pat: string = event.target.value;
                props.handleInput(pat);
            }}
            onKeyDown={props.handleKeyDown}
            onClick={props.handleInputClick}
        />
        <button className={cns('companySearchInput')}>Search</button>
    </div>
);

export namespace CompanySearch {
    export interface IProps {
        history: History;
        handleInput: (pat: string) => void;
        results: RootState.SearchResultsState;
    }
}

interface IState {
    activeIndex: -1|0|1|2|3|4|5|6|7|8|9;
    inputValue: string;
    isInputActive: boolean;
}

export class CompanySearch extends React.Component<CompanySearch.IProps> {
    public state: Readonly<IState> = {
        activeIndex: -1, // -1 indicates no active item
        inputValue: '',
        isInputActive: true,
    };


    private node = React.createRef<HTMLDivElement>();

    constructor(props: CompanySearch.IProps, context?: any) {
        super(props, context);
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleInput = this.handleInput.bind(this);
        this.handleInputClick = this.handleInputClick.bind(this);
        this.forceBlur = this.forceBlur.bind(this);
        this.navigate = this.navigate.bind(this);
        this.handleKeyboardSelect = this.handleKeyboardSelect.bind(this);
        this.maybeCloseTypeaheadFromOutsideClick = this.maybeCloseTypeaheadFromOutsideClick.bind(this);
        this.handleResultClick = this.handleResultClick.bind(this);
    }


    public componentDidMount(): void {
        window.addEventListener('click', this.maybeCloseTypeaheadFromOutsideClick);
    }

    public componentWillUnmount(): void {
        window.removeEventListener('click', this.maybeCloseTypeaheadFromOutsideClick);
    }

    public maybeCloseTypeaheadFromOutsideClick(event: MouseEvent) {
        if (
            event.target instanceof HTMLElement &&
            this.node &&
            this.node.current &&
            !this.node.current.contains(event.target)) {
            this.forceBlur(false);
        }
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
            event.preventDefault();
            this.navigate(1);
        } else if ((navDownKeys as any)[event.key]) {
            event.preventDefault();
            this.navigate(-1);
        } else if ((blurKeys as any)[event.key]) {
            event.preventDefault();
            this.forceBlur();
        } else if ((selectKeys as any)[event.key]) {
            event.preventDefault();
            this.handleKeyboardSelect();
        }
    }

    public handleInput(pat: string) {
        this.setState({
            inputValue: pat,
            isInputActive: true,
        });
        this.props.handleInput(pat);
    }

    public handleInputClick() {
        this.setState({
            isInputActive: !!this.state.inputValue,
        });
    }

    public handleResultClick(event: React.MouseEvent) {
        event.preventDefault();
        const companySearchResultElem = event.currentTarget as HTMLInputElement;
        const shortCik: string = companySearchResultElem.dataset.shortCik || '';
        this.forceBlur(true);
        this.props.history.push(getCompanyWithFilingsLink(shortCik));
    }

    public forceBlur(resetInputContent = false) {
        this.setState({
            activeIndex: -1,
            inputValue: resetInputContent ? '' : this.state.inputValue,
            isInputActive: false,
        });
    }

    public navigate(direction: -1|1) {
        if (this.props.results.data.length > 0) {
            const lastItemIndex = this.props.results.data.length - 1;
            let newActiveItemIndex = this.state.activeIndex - direction;
            if (newActiveItemIndex < 0) {
                newActiveItemIndex = lastItemIndex;
            } else if (newActiveItemIndex > lastItemIndex) {
                newActiveItemIndex = 0;
            }
            this.setState({
                activeIndex: newActiveItemIndex,
            });
        } else {
            this.setState({
                activeIndex: -1,
            });
        }
    }

    public handleKeyboardSelect() {
        // If no item selected when the user presses enter,
        // just go to the first one in the list.
        if (this.props.results.data && this.props.results.data.length) {
            const indexToVisit = Math.max(this.state.activeIndex, 0);
            const shortCik = this.props.results.data[indexToVisit].short_cik;
            this.props.history.push(`/companies/${shortCik}`);
            this.forceBlur(true);
        }
    }

    public render() {
        return (
            <div
              ref={this.node}
              className={style.companySearch}
            >
                <CompanySearchInput
                    inputValue={this.state.inputValue}
                    handleInput={this.handleInput}
                    handleInputClick={this.handleInputClick}
                    handleKeyDown={this.handleKeyDown}
                    isInputActive={this.state.isInputActive}
                />
                {
                    this.state.isInputActive &&
                        <CompanySearchResults
                            onResultClick={this.handleResultClick}
                            results={this.props.results}
                            activeIndex={this.state.activeIndex}
                            history={this.props.history}
                        />
                }
            </div>
        );
    }
}
