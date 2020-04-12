import React, {Component} from "react";
import PropTypes from "prop-types";
import {Route, Switch} from "react-router-dom";

import "style.scss";

class HelloWorld extends Component {
	constructor(props) {
		super(props);

		this.state = {
			hello: "World",
		};
	}

	componentDidMount() {
		this.setState({
			...this.state,
			hello: this.state.hello + "!",
		});
	}

	render() {
		return (
			<div>
				<h1>Hello, {this.state.hello}</h1>
				<a href="/api/hello">Go Here to test the api is working</a>
			</div>
		);
	}
}

HelloWorld.propTypes = {
};

export default HelloWorld;
