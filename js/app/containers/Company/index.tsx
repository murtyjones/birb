import {CompanyActions} from 'app/actions/companies';
import {CompanyModel} from 'app/models';
import {IFilingModel} from 'app/models/IFilingModel';
import {RootState} from 'app/reducers';
import {createLoadingSelector} from 'app/reducers/selectors/loading';
import {omit} from 'app/utils';
import * as React from 'react';
import {connect} from 'react-redux';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {bindActionCreators, Dispatch} from 'redux';
import * as style from './style.css';

const getFilingUrl = (filingId: string|number) =>
    `/filings/${filingId}`;

interface IMatchParams {
    shortCik: string;
}

export namespace Company {
    export interface IProps extends RouteComponentProps<IMatchParams> {
        actions: CompanyActions;
        isFetching: boolean;
        company: CompanyModel;
        companyFilings: IFilingModel[];
    }
}

const loadingSelector = createLoadingSelector([CompanyActions.Type.GET_COMPANY]);

@connect(
    (state: RootState, ownProps): Pick<Company.IProps, 'company' | 'companyFilings' | 'isFetching'> => {
        const shortCik = ownProps.match.params.shortCik;
        const company = state.companies.byShortCik[shortCik] || {};
        const companyFilings = company.filings || [];

        return {
            company,
            companyFilings,
            isFetching: loadingSelector(state),
        };
    },
    (dispatch: Dispatch): Pick<Company.IProps, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch),
    }),
)

export class Company extends React.PureComponent<Company.IProps> {
    constructor(props: Company.IProps, context?: any) {
        super(props, context);
    }

    public async componentDidUpdate(prevProps: Readonly<Company.IProps>, prevState: Readonly<{}>, snapshot?: any) {
        if (this.props.match.params.shortCik !== prevProps.match.params.shortCik) {
            await this.props.actions.getCompany(this.props.match.params.shortCik);
        }
    }

    public async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        await this.props.actions.getCompany(shortCik);
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

const DataTable: React.FC<IDataTableProps> = (props) =>
    <div className={style.allFilingsTable}>
        {
            props.data.map((each) =>
                <Link to={getFilingUrl(each.id)}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                        <span>{each.date_filed}</span>
                </Link>,
            )
        }
    </div>;
