import React, {Component} from "react";
import {withStyles} from "@material-ui/core/styles";

import queryString from "query-string";

export class Home extends Component {
	constructor(props) {
		super(props);
	}

	componentDidMount() {
		const values = queryString.parse(this.props.location.search);
		const {uid, code} = values;

		if (uid && code) {
			this.props.verify(uid, code);
		} else {
			this.props.home();
			return;
		}
	}

	render() {
		return (
			<>
			</>
		);
	}
}

const styles = (theme) => ({
	root: {

	},
	heading: {
		textAlign: "center",
	},
});

export default withStyles(styles)(Home);
