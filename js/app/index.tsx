import * as React from 'react';
import { Route, Switch } from 'react-router';
import { App as Birb } from 'app/containers/App';
import { Header } from 'app/containers/Header';
import { hot } from 'react-hot-loader';

const ActiveRoute = () => (
    <Switch>
        <Route path='/' component={Birb} />
    </Switch>
);

export const App = hot(module)(() => (
    <>
        <Route path='/' component={Header} />
        <ActiveRoute/>
    </>
));
