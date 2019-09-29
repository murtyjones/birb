import {CompanyActions} from 'app/actions';
import {createLoadingSelector, IRootState} from 'app/reducers';
import {omit} from 'app/utils';
import * as React from 'react';
import {connect} from 'react-redux';
import {RouteComponentProps} from 'react-router';
import {bindActionCreators, Dispatch} from 'redux';

interface IMatchParams {
    shortCik: string;
    filingId: string;
}

export namespace Filing {
    export interface IProps extends RouteComponentProps<void> {
        actions: CompanyActions;
        isFetching: boolean;
        shortCik: string;
        filingId: string;
        signedUrl: string|null;
    }

    export interface IState {
        content?: string;
    }
}

const loadingSelector = createLoadingSelector([CompanyActions.Type.GET_COMPANY_SIGNED_FILING_URL]);


@connect(
    (state: IRootState, ownProps): Pick<Filing.IProps, 'signedUrl' | 'shortCik' | 'filingId' | 'isFetching'> => {
        const shortCik = ownProps.match.params.shortCik;
        const filingId = ownProps.match.params.filingId;
        const company = state.companies.byShortCik[shortCik];

        return {
            filingId,
            isFetching: loadingSelector(state),
            shortCik,
            signedUrl: company && company.signedUrl ? company.signedUrl : null,
        };
    },
    (dispatch: Dispatch): Pick<Filing.IProps, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch),
    }),
)

export class Filing extends React.Component<Filing.IProps, Filing.IState> {
    private myRef = React.createRef<HTMLIFrameElement>();

    constructor(props: Filing.IProps, context?: any) {
        super(props, context);
        this.state = {
            content: undefined,
        };
    }


    public async componentDidMount() {
        const shortCik = this.props.shortCik;
        const filingId = this.props.filingId;
        await this.props.actions.getSignedUrl(shortCik, filingId);
    }

    public shouldComponentUpdate(
        nextProps: Readonly<Filing.IProps>, nextState: Readonly<Filing.IState>, nextContext: any,
    ) {
        return (
            nextProps.signedUrl !== this.props.signedUrl
        );
    }

    public render() {
        return (
            <div
                style={{
                    display: 'grid',
                    gridTemplateColumns: '200px auto',
                    height: '100vh',
                }}
            >
                <div>sidebar</div>
                <iframe
                    ref={this.myRef}
                    src={this.props.signedUrl || ''}
                    seamless={true}
                    style={{
                        border: 0,
                        height: '100%',
                        width: '100%',
                    }}
                />
            </div>
        );
    }
}
