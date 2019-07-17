import * as React from 'react';
import { Route, Switch } from 'react-router';
import { App as Birb } from 'app/containers/App';
import { Header } from 'app/containers/Header';
import { Company } from 'app/containers/Company';
import { CompanyNav } from 'app/components/CompanyNav';
import { hot } from 'react-hot-loader';

const ActiveRoute = () => (
    <Switch>
        <Route path='/' exact component={Birb} />
        <Route path='/companies/:shortCik' exact component={Company} />
        {/* TODO add 404 route */}
    </Switch>
);

export const App = hot(module)(() => (
    <>
        {/* Show the header on all routes */}
        <Route path='/' component={Header} />
        <Route path='/companies/:shortCik/:activeTab?' component={CompanyNav} />
        <ActiveRoute />
    </>
));
