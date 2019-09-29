import {CompanyActions} from 'app/actions/companies';
import {ICompanyModel} from 'app/models';
import {IFilingModel} from 'app/models/IFilingModel';
import {IRootState} from 'app/reducers';
import {createLoadingSelector} from 'app/reducers/selectors/loading';
import {omit} from 'app/utils';
import * as React from 'react';
import {connect} from 'react-redux';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {bindActionCreators, Dispatch} from 'redux';
import * as style from './style.css';

const getFilingUrl = (shortCik: string, filingId: string|number) =>
    `/companies/${shortCik}/filings/${filingId}`;

interface IMatchParams {
    shortCik: string;
}

export interface IProps extends RouteComponentProps<IMatchParams> {
    actions: CompanyActions;
    isFetching: boolean;
    company: ICompanyModel;
    companyFilings: IFilingModel[];
}

const loadingSelector = createLoadingSelector([CompanyActions.Type.GET_COMPANY]);

@connect(
    (state: IRootState, ownProps): Pick<IProps, 'company' | 'companyFilings' | 'isFetching'> => {
        const shortCik = ownProps.match.params.shortCik;
        const company = state.companies.byShortCik[shortCik] || {};
        const companyFilings = company.filings || [];

        return {
            company,
            companyFilings,
            isFetching: loadingSelector(state),
        };
    },
    (dispatch: Dispatch): Pick<IProps, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch),
    }),
)

export class Company extends React.PureComponent<IProps> {
    constructor(props: IProps, context?: any) {
        super(props, context);
    }

    public async componentDidUpdate(prevProps: Readonly<IProps>, prevState: Readonly<{}>, snapshot?: any) {
        if (this.props.match.params.shortCik !== prevProps.match.params.shortCik) {
            await this.props.actions.getCompanyWithFilings(this.props.match.params.shortCik);
        }
    }

    public async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        await this.props.actions.getCompanyWithFilings(shortCik);
    }

    public render() {
        const content =
            this.props.isFetching
                ? <div>Loading...</div>
                : this.props.companyFilings.length === 0
                    ? <div>Sorry, we don't have any filings for this company yet.</div>
                    : <DataTable data={this.props.companyFilings} />;

        return (
            <div className={`${style.mainCompanyContents} container`}>
                {content}
            </div>
        );
    }
}

interface IDataTableProps {
    data: IFilingModel[]; // Change the required prop to an optional prop.
}

const DataTable: React.FC<IDataTableProps> = (props) => (
    <div className={style.allFilingsTable}>
        {
            props.data.map((each) =>
                <Link key={each.id} to={getFilingUrl(each.company_short_cik, each.id)}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                        <span>{each.date_filed}</span>
                </Link>,
            )
        }
    </div>
);
