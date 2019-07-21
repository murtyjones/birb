import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';
import {connect} from 'react-redux';
import {RootState} from 'app/reducers';
import {CompanyActions} from 'app/actions/companies';
import {bindActionCreators, Dispatch} from 'redux';
import {omit} from 'app/utils';

interface MatchParams {
    shortCik: string;
}

export namespace Company {
    export interface Props extends RouteComponentProps<MatchParams> {
        companies: RootState.CompanyState;
        actions: CompanyActions;
    }
}

@connect(
    (state: RootState, ownProps): Pick<Company.Props, 'companies'> => {
        return { companies: state.companies  };
    },
    (dispatch: Dispatch): Pick<Company.Props, 'actions'> => ({
        actions: bindActionCreators(omit(CompanyActions, 'Type'), dispatch)
    })
)

export class Company extends React.PureComponent<Company.Props> {
    constructor(props: Company.Props, context?: any) {
        super(props, context);
    }
    readonly state = { allFilings: { company_name: '', filings: [], short_cik: '' } };

    async componentDidMount() {
        const shortCik = this.props.match.params.shortCik;
        this.props.actions.getCompany(shortCik);
    }

    render() {
        console.log(this.props.companies.byShortCik);
        return (
            <div className={`${style.mainCompanyContents} container`}>

                {/*{ this.state.allFilings.filings.length > 0*/}
                {/*    ? AllFilingsTable(this.state.allFilings)*/}
                {/*    : 'Hello!'*/}
                {/*}*/}
            </div>
        )
    }
}

const AllFilingsTable = (allFilings: CompanyFilingData) => (
    <div className={style.allFilingsTable}>

        {
            allFilings.filings.map(each =>
                <Link to={`/filing?bucket=birb-edgar-filings&key=${each.filing_edgar_url}`}>
                        <span>{each.filing_name}</span>
                        <span>{each.filing_quarter}</span>
                        <span>{each.filing_year}</span>
                </Link>
            )
        }
    </div>
);
