import * as style from 'app/containers/Header/style.css';
import {RootState} from 'app/reducers';
import {Result} from 'app/reducers/search';
import cns from 'classnames';
import { History } from 'history';
import * as React from 'react';
import {Link} from 'react-router-dom';


interface ICompanySearchResult {
    result: Result;
    isActive: boolean;
}

/**
 * The classname that will be used when determining
 * whether to blur the typeahead on click.
 */
const eventListenerClassName = 'companySearchInput';

const Result: React.FC<ICompanySearchResult> = (props) => (
    <Link
        to={`/companies/${props.result.short_cik}`}
        className={cns(style.companySearchResult, {
            [style.activeResult]: props.isActive,
        }, eventListenerClassName)}
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
    history: History;
}

const CompanySearchResults: React.FC<ICompanySearchResults> = (props) => (
    <div className={cns(style.companySearchResults, eventListenerClassName)}>
        {props.results.data.map((each, i: number) =>
            <Result
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
    <div className={cns(style.companySearchInput, eventListenerClassName)}>
        <input
            value={props.inputValue}
            className={cns('companySearchInput', eventListenerClassName)}
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
        <button className={cns('companySearchInput', eventListenerClassName)}>Search</button>
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
    isInputActive: boolean;
    inputValue: string;
}

export class CompanySearch extends React.PureComponent<CompanySearch.IProps> {
    public state: Readonly<IState> = {
        activeIndex: -1, // -1 indicates no active item
        isInputActive: true,
        inputValue: '',
    };

    constructor(props: CompanySearch.IProps, context?: any) {
        super(props, context);
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleInput = this.handleInput.bind(this);
        this.handleInputClick = this.handleInputClick.bind(this);
        this.forceBlur = this.forceBlur.bind(this);
        this.navigate = this.navigate.bind(this);
        this.handleKeyboardSelect = this.handleKeyboardSelect.bind(this);
        this.maybeCloseTypeaheadFromOutsideClick = this.maybeCloseTypeaheadFromOutsideClick.bind(this);
    }


    componentDidMount(): void {
        window.addEventListener('click', this.maybeCloseTypeaheadFromOutsideClick);
    }

    componentWillUnmount(): void {
        window.removeEventListener('click', this.maybeCloseTypeaheadFromOutsideClick)
    }

    public maybeCloseTypeaheadFromOutsideClick (event: MouseEvent) {
        if (event && event.target) {
            const elem = event.target as HTMLInputElement;
            if (!(elem && elem.className && elem.className.includes(eventListenerClassName))) {
                this.forceBlur(false);
            }
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
            isInputActive: true,
            inputValue: pat,
        });
        this.props.handleInput(pat);
    }

    public handleInputClick() {
        this.setState({
            isInputActive: !!this.state.inputValue
        });
    }

    public forceBlur(resetInputContent = false) {
        this.setState({
            activeIndex: -1,
            isInputActive: false,
            inputValue: resetInputContent ? '' : this.state.inputValue,
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
        const shortCik = this.props.results.data[this.state.activeIndex].short_cik;
        this.props.history.push(`/companies/${shortCik}`);
        this.forceBlur(true);
    }

    public render() {
        return (
            <div className={style.companySearch}>
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
                            results={this.props.results}
                            activeIndex={this.state.activeIndex}
                            history={this.props.history}
                        />
                }
            </div>
        );
    }
}
