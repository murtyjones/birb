import * as React from 'react';
import { RouteComponentProps } from 'react-router';
import * as style from './style.css';

export interface IProps extends RouteComponentProps<void> {
  // thing1: IRootState.Thing1State;
  // actions: SomeActions;
}

// @connect(
//   (state: IRootState, ownProps): Pick<IProps, 'thing1' | 'thing2'> => {},
//   (dispatch: Dispatch): Pick<IProps, 'actions'> => ({})
// )
export class App extends React.Component<IProps> {
  public static defaultProps: Partial<IProps> = {

  };

  constructor(props: IProps, context?: any) {
    super(props, context);
  }

  public render() {
    return (
      <div className={style.normal}/>
    );
  }
}
