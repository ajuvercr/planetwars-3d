import * as React from 'react';

import * as styles from '../style.css'

export interface RouterProps {
    routes: Route[],
    active: string,
}

export interface Route {
    name: string;
    element: JSX.Element;
}

interface State {
    active: string,
    routes: Route[],
}

export class Router extends React.Component<RouterProps, State> {
    constructor(props: RouterProps) {
        super(props);

        this.state = {
            active: props.active,
            routes: props.routes
        };

        this.setRoute.bind(this);
        this.buildRoute.bind(this);
    }

    setRoute(active: string) {
        console.log("Setting route to "+active);
        this.setState({active});
    }

    buildRoute(name: string) {
        return <li key={name} onClick={() => this.setRoute(name)}>{name}</li>
    }

    render() {
        const active = this.state.routes.find((o) => o.name == this.state.active).element;
        return (
            <div className={styles.router}>
                <ul>
                    {this.state.routes.map((route) => this.buildRoute(route.name))}
                </ul>
                <div>
                    {active}
                </div>
            </div>
        )
    }
}
