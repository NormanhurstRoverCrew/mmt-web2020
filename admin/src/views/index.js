import React, {Component} from "react";
import {Route, Switch} from "react-router-dom";
import {withRouter} from "react-router-dom";

import Dashboard from "views/Dashboard";

import routingDefinitions from "views/routes";
import {Typography} from "@material-ui/core";

export class Router extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const root = (
			<Route exact
				path="/"
				render={(props) => (
					<Dashboard/>
				)} />
		);

		const routes = routingDefinitions.map((route) => {
			return (
				<Route
					key={route.path}
					path={route.path}
					component={route.view}
				/>
			);
		});

		return (
			<Switch>
				{routes}

				{/* Must always be last */}
				{root}

				<Route
					render={() => (
						<div>
							<Typography variant="h3"
								component="h1">
								404
							</Typography>
							<Typography variant="h4"
								component="h2">
								No Route Matches
							</Typography>
							<Typography variant="h5"
								component="p">
								{location.pathname}
							</Typography>
						</div>
					)}
				/>
			</Switch>
		);
	}
}

Router.propTypes = {
};

export default withRouter(Router);
