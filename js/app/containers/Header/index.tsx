import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';
import {Link} from 'react-router-dom';

const Logo = () => (
    <div className={style.logo}>
        <Link to='/'>birb</Link>
    </div>
);

const CompanySearch = () => (
    <div className={style.companySearch}>
        <input type='text' />
    </div>
);

export namespace Header {
    export interface Props extends RouteComponentProps<void> {}
}

export class Header extends React.PureComponent<Header.Props> {
    static defaultProps: Partial<Header.Props> = {};

    render() {
        return (
            <header>
                <div className={style['header-background']}/>
                <div className={style['header-contents']}>
                    <Logo />
                    <CompanySearch />
                </div>
            </header>
        )
    }
}
