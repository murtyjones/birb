import * as React from 'react';
import * as style from './style.css';
import { RouteComponentProps } from 'react-router';

export namespace App {
  export interface Props extends RouteComponentProps<void> {
    // thing1: IRootState.Thing1State;
    // actions: SomeActions;
  }
}

// @connect(
//   (state: IRootState, ownProps): Pick<App.Props, 'thing1' | 'thing2'> => {},
//   (dispatch: Dispatch): Pick<App.Props, 'actions'> => ({})
// )
export class App extends React.Component<App.Props> {
  static defaultProps: Partial<App.Props> = {

  };

  constructor(props: App.Props, context?: any) {
    super(props, context);
  }

  render() {
    return (
      <div className={style.normal}></div>
    );
  }
}
