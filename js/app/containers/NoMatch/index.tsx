import * as React from 'react';
import * as style from './style.css';

export const NoMatch: React.FC<{}> = (props) =>
    <div className={`${style.main} container`}>
        <h1 className={style.h2}>Oops!</h1>
        <h3 className={style.h5}>Sorry, this page doesn't seem to exist!</h3>
    </div>;
