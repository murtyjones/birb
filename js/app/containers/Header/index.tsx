import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {createLoadingSelector, RootState} from 'app/reducers';
import {connect} from 'react-redux';
import {bindActionCreators, Dispatch} from 'redux';
import {omit} from 'app/utils';
import {SearchActions} from 'app/actions/search';
import {CompanySearch} from 'app/containers/Header/search';

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
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
                        handleInput={this.props.actions.searchCompany}
                        results={this.props.searchResults}
                    />
                </div>
            </header>
        )
    }
}
