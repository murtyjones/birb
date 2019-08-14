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

interface MatchParams {
    shortCik: string;
}

export namespace Company {
    export interface Props extends RouteComponentProps<MatchParams> {
        actions: CompanyActions;
        isFetching: boolean;
        company: CompanyModel;
        companyFilings: IFilingModel[];
    }
}

const loadingSelector = createLoadingSelector([CompanyActions.Type.GET_COMPANY]);

@connect(
    (state: RootState, ownProps): Pick<Company.Props, 'company' | 'companyFilings' | 'isFetching'> => {
        const shortCik = ownProps.match.params.shortCik;
        const company = state.companies.byShortCik[shortCik] || {};
        const companyFilings = company.filings || [];

        return {
            company,
            companyFilings,
            isFetching: loadingSelector(state),
        };
    },
    (dispatch: Dispatch): Pick<Company.Props, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch),
    }),
)

export class Company extends React.PureComponent<Company.Props> {
    constructor(props: Company.Props, context?: any) {
        super(props, context);
    }

    public async componentDidUpdate(prevProps: Readonly<Company.Props>, prevState: Readonly<{}>, snapshot?: any): void {
        if (this.props.match.params.shortCik !== prevProps.match.params.shortCik) {
            await this.props.actions.getCompany(this.props.match.params.shortCik);
        }
    }

    public async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        await this.props.actions.getCompany(shortCik);
    }

    public render() {

        if (this.props.isFetching) {
            return <div>Loading...</div>;
        }



        return (
            <div className={`${style.mainCompanyContents} container`}>
                <DataTable
                    data={this.props.companyFilings}
                />
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
                <Link to={`/filing?bucket=birb-edgar-filings&key=${each.filing_edgar_url}`}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                        <span>{each.date_filed}</span>
                </Link>,
            )
        }
    </div>;
