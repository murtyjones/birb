import {SearchActions} from 'app/actions/search';
import {CompanySearch} from 'app/containers/Header/search';
import {createLoadingSelector, IRootState, ISearchResultsState} from 'app/reducers';
import {omit} from 'app/utils';
import * as React from 'react';
import {connect} from 'react-redux';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {bindActionCreators, Dispatch} from 'redux';
import * as style from './style.css';

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
    </div>
);

export interface IProps extends RouteComponentProps<void> {
    actions: SearchActions;
    isFetching: boolean;
    searchResults: ISearchResultsState;
}

const loadingSelector = createLoadingSelector([SearchActions.Type.SEARCH_COMPANY]);

@connect(
    (state: IRootState, ownProps): Pick<IProps, 'isFetching' | 'searchResults'> => {
        return {
            isFetching: loadingSelector(state),
            searchResults: state.searchResults,
        };
    },
    (dispatch: Dispatch): Pick<IProps, 'actions'> => ({
        actions: bindActionCreators(omit(SearchActions, 'Type'), dispatch),
    }),
)

export class Header extends React.PureComponent<IProps> {

    public static defaultProps: Partial<IProps> = {};
    constructor(props: IProps, context?: any) {
        super(props, context);
    }

    public render() {
        return (
            <header className={style.headerBackground}>
                <div className={`${style.headerContents} container`}>
                    <Logo />
                    <CompanySearch
                        history={this.props.history}
                        handleInput={this.props.actions.searchCompany}
                        results={this.props.searchResults}
                    />
                </div>
            </header>
        );
    }
}
