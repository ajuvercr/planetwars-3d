import * as _ from 'lodash';
import * as React from 'react';
import { render } from 'react-dom';
import { sub } from '../pw/pkg/';

import * as styles from './style.css'

console.log(sub(5, 3));

const Hello = (props: { who: string }) => (
  <p>Hello, {props.who}</p>
);

class MyList extends React.Component<{}, number[]> {
  constructor(props: {}) {
    super(props);

    this.state = [1,2,3];
  }

  render() {
    return (
      <ul className={styles.list}>
        {this.state.map(x => <li key={x.toString()}>Number: {x}</li>)}
      </ul>
    )
  }
}

render((<div><Hello who="me"/> <MyList/></div>), document.getElementById('root'));
