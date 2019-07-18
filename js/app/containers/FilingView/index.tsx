import * as React from 'react';
import * as style from './style.css';
import {RouteComponentProps} from 'react-router';

export namespace FilingView {
    export interface Props extends RouteComponentProps<void> {}
}

export class FilingView extends React.Component<FilingView.Props> {
    constructor(props: FilingView.Props, context?: any) {
        super(props, context);
    }
    readonly state = { results: { data: [], has_more: false } };

    static defaultProps: Partial<FilingView.Props> = {};

    shouldComponentUpdate(nextProps: Readonly<FilingView.Props>, nextState: Readonly<{}>, nextContext: any): boolean {
        return false;
    }

    render() {
        return (
            <div style={{
                display: 'grid',
                'grid-template-columns': '200px auto',
                height: '100vh'
            }}>
                <div>sidebar</div>
                <iframe
                    sandbox='allow-scripts'
                    style={{
                        width: '100%',
                        height: '100%',
                        border: 0
                    }}
                    src='http://127.0.0.1:8887/crates/filing-parser/examples/10-Q/output/wow.html'
                />
            </div>
        )
    }
}
