import {CompanyActions} from 'app/actions';
import {ICompanyModel} from 'app/models';
import {IFilingModel} from 'app/models/IFilingModel';
import {createLoadingSelector, RootState} from 'app/reducers';
import {omit} from 'app/utils';
import * as React from 'react';
import {connect} from 'react-redux';
import {RouteComponentProps} from 'react-router';
import {bindActionCreators, Dispatch} from 'redux';
import * as style from './style.css';
import {http} from "app/utils/http";

interface IMatchParams {
    shortCik: string;
    filingId: string;
}

export namespace FilingView {
    export interface IProps extends RouteComponentProps<void> {
        actions: CompanyActions;
        isFetching: boolean;
        shortCik: string;
        filingId: string;
        signedUrl: string|null;
    }


    export interface IState {
        filingContents?: string;
    }
}

const loadingSelector = createLoadingSelector([CompanyActions.Type.GET_COMPANY_SIGNED_FILING_URL]);

@connect(
    (state: RootState, ownProps): Pick<FilingView.IProps, 'signedUrl' | 'shortCik' | 'filingId' | 'isFetching'> => {
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
    (dispatch: Dispatch): Pick<FilingView.IProps, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch),
    }),
)

export class FilingView extends React.Component<FilingView.IProps, FilingView.IState> {
    constructor(props: FilingView.IProps, context?: any) {
        super(props, context);
        this.state = {
            filingContents: undefined,
        };
    }

    public async componentDidMount() {
        const shortCik = this.props.shortCik;
        const filingId = this.props.filingId;
        await this.props.actions.getSignedUrl(shortCik, filingId);
    }

    public async componentDidUpdate(prevProps: Readonly<FilingView.IProps>, prevState: Readonly<{}>, snapshot?: any) {
        if (this.props.signedUrl && !prevProps.signedUrl) {
            const request = new Request(this.props.signedUrl, {
                method: 'GET',
            });
            const response = await http(request);
            console.log(response);
        }
    }

    public render() {
        return (
            <div style={{
                display: 'grid',
                gridTemplateColumns: '200px auto',
                height: '100vh',
            }}>
                <div>sidebar</div>
                { this.state.filingContents
                    ?
                    (
                        <iframe
                            sandbox='allow-scripts'
                            style={{
                                border: 0,
                                height: '100%',
                                width: '100%',
                            }}
                            srcDoc={this.state.filingContents}
                        />
                    )
                    : 'Loading...'
                }
            </div>
        );
    }
}
