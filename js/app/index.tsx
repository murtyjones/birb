import * as React from 'react';
import { Route, Switch } from 'react-router';
import { App as Birb } from 'app/containers/App';
import { Header } from 'app/containers/Header';
import { Company } from 'app/containers/Company';
import { CompanyNav } from 'app/components/CompanyNav';
import { FilingView } from 'app/containers/FilingView';
import { NoMatch } from 'app/containers/NoMatch';
import { hot } from 'react-hot-loader';

const ActiveRoute = () => (
    <Switch>
        <Route path='/' exact component={Birb} />
        <Route path='/companies/:shortCik' exact component={Company} />
        <Route path='/companies/:shortCik/filings/:filingId' exact component={FilingView} />
        <Route component={NoMatch} />
    </Switch>
);

/*
 * Show the header on all routes except those blacklisted below
 */
const MaybeRenderHeader = () => (
    <Switch>
        /* Routes that should not show the header (not exact): */
        <Route
            path={[
                '/companies/:shortCik/filings',
            ]}
            children={null}
        />
        /* All other routes ought to show the header: */
        <Route path='/' component={Header} />
    </Switch>
);

/*
 * Show the company nav on all routes except those blacklisted below
 */
const MaybeRenderCompanyNav = () => (
    <Switch>
        /* Routes that should not show the company nav (not exact): */
        <Route
            path={[
                '/companies/:shortCik/filings',
            ]}
            children={null}
        />
        /* All other routes ought to show the header: */
        <Route path='/companies/:shortCik/:activeTab?' component={CompanyNav} />
    </Switch>
);

export const App = hot(module)(() => (
    <>

        <MaybeRenderHeader />

        <MaybeRenderCompanyNav />

        <ActiveRoute />
    </>
));
