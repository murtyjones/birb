import * as React from 'react';
import {Link, RouteComponentProps} from 'react-router-dom';
import * as style from './style.css';
import * as classNames from 'classnames';

enum tabs {
  all = 'All Filings',
  financials = 'Financials',
}

enum tabLinks {
  all = '',
  financials = 'financials',
}

interface MatchParams {
  activeTab: string;
  shortCik: string;
}

export namespace CompanyNav {
  export interface Props extends RouteComponentProps<MatchParams> {}
}

export class CompanyNav extends React.Component<CompanyNav.Props> {
  render() {
    const activeTab = tabs[this.props.match.params.activeTab as keyof typeof tabs] || tabs.all;
    const baseTabUrl = `/companies/${this.props.match.params.shortCik}`;
    return (
      <nav id={style.nav}>
        <div className={`${style.navContents} container`}>
          <div>Company info section</div>
          <div className={style.navigation}>
            {
              Object.keys(tabs).map((key: string) =>
                    <Link
                        to={`${baseTabUrl}/${tabLinks[key as keyof typeof tabLinks]}`}
                        className={tabs[key as keyof typeof tabs] === activeTab ? style.active : ''}
                    >
                      <span>
                        {tabs[key as keyof typeof tabs]}
                      </span>
                    </Link>
              )
            }
          </div>
        </div>
      </nav>
    );
  }
}
