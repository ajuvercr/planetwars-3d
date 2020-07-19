import * as _ from 'lodash';
import * as React from 'react';
import { render } from 'react-dom';
import { sub } from '../pw/pkg/';

console.log(sub(5, 3));

const Hello = (props: { who: string }) => (
  <p>Hello, {props.who}</p>
);

render(<Hello who="me"/>, document.getElementById('root'));
