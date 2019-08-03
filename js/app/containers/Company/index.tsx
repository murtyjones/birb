import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {connect} from 'react-redux';
import {RootState} from 'app/reducers';
import {CompanyActions} from 'app/actions/companies';
import {bindActionCreators, Dispatch} from 'redux';
import {omit} from 'app/utils';
import {createLoadingSelector} from 'app/reducers/selectors/loading';
import {FilingModel} from "app/models/FilingModel";
import {CompanyModel} from "app/models";

interface MatchParams {
    shortCik: string;
}

export namespace Company {
    export interface Props extends RouteComponentProps<MatchParams> {
        actions: CompanyActions;
        isFetching: boolean;
        company: CompanyModel;
        companyFilings: FilingModel[];
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
            isFetching: loadingSelector(state)
        };
    },
    (dispatch: Dispatch): Pick<Company.Props, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch)
    })
)

export class Company extends React.PureComponent<Company.Props> {
    constructor(props: Company.Props, context?: any) {
        super(props, context);
    }

    async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        await this.props.actions.getCompany(shortCik);
    }

    render() {

        if (this.props.isFetching) {
            return <div>Loading...</div>
        }



        return (
            <div className={`${style.mainCompanyContents} container`}>
                <DataTable
                    data={this.props.companyFilings}
                />
            </div>
        )
    }
}

interface IDataTableProps {
    data: FilingModel[], // Change the required prop to an optional prop.
}

const DataTable: React.FC<IDataTableProps> = (props) =>
    <div className={style.allFilingsTable}>
        {
            props.data.map(each =>
                <Link to={`/filing?bucket=birb-edgar-filings&key=${each.filing_edgar_url}`}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                </Link>
            )
        }
    </div>;
