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
    (state: RootState, ownProps): Pick<Filing.IProps, 'signedUrl' | 'shortCik' | 'filingId' | 'isFetching'> => {
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

    public async componentDidUpdate(prevProps: Readonly<Filing.IProps>, prevState: Readonly<{}>, snapshot?: any) {
        if (this.props.signedUrl && !prevProps.signedUrl) {
            const response = await fetch(this.props.signedUrl);
            const content = await response.text() || '';
            const node = this.myRef.current;
            if (node && node.contentDocument) {
                const doc = node.contentDocument;
                doc.open();
                doc.write(content);
                doc.close();
            }
        }
    }

    public shouldComponentUpdate(
        nextProps: Readonly<Filing.IProps>, nextState: Readonly<Filing.IState>, nextContext: any,
    ) {
        return (
            nextProps.signedUrl !== this.props.signedUrl || nextState.content !== this.state.content
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
