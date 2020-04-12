import React, {Component} from "react";
import PropTypes from "prop-types";
import {Route, Switch} from "react-router-dom";
import {ConnectedRouter} from "connected-react-router";
import {connect} from "react-redux";

import HelloWorld from "components/HelloWorld";

import "style.scss";

class App extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<ConnectedRouter history={this.props.history}>
				<Switch>
					<Route path="/"
						component={HelloWorld} />
				</Switch>
			</ConnectedRouter>
		);
	}
}

App.propTypes = {
	history: PropTypes.object.isRequired,
};

const mapStateToProps = (state) => {
	return {

	};
};
const mapDispatchToProps = (dispatch) => {
	return {

	};
};

export default connect(mapStateToProps, mapDispatchToProps)(App);
