import * as _ from 'lodash';
import * as React from 'react';
import { render } from 'react-dom';
import { sub } from '../pw/pkg/';

import * as styles from './style.css'
import { Router } from './routing/router';

console.log(sub(5, 3));

const Hello = (props: { who: string }) => (
  <p>Hello, {props.who}</p>
);

class MyList extends React.Component<{}, { inner: number[] }> {
  constructor(props: {}) {
    super(props);

    this.state = { inner: [1, 2, 3] };
  }

  render() {
    return (
      <ul className={styles.list}>
        {this.state.inner.map(x => <li key={x.toString()}>Number: {x}</li>)}
      </ul>
    )
  }
}

render(<Router
  active="Home"
  routes={[{ element: <MyList />, name: "Home" }, { element: <Hello who="me" />, name: "Hello" }]} />,
  document.getElementById('root'));
