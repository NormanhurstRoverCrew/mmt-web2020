import React, {Component} from "react";
import {TextField} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import PropTypes from "prop-types";

import EXAMPLES from "rego/examples";

export class InputDiet extends Component {
	constructor(props) {
		super(props);

		this.state = {
			name: null,
		};

		this.onInputUpdate = this.onInputUpdate.bind(this);
	}

	onInputUpdate(e) {
		const name = e.target.value;
		this.props.onChange(e, e.target.value);
	}

	render() {
		const {classes, value} = this.props;
		return (
			<TextField
				error={typeof this.state.error !== "undefined"}
				id="diet"
				label="Diet"
				multiline
				className={classNames(classes.textField)}
				value={value}
				onChange={this.onInputUpdate}
				margin="normal"
				fullWidth
				helperText={this.props.error || this.state.error || EXAMPLES.diet}
			/>
		);
	}
}

InputDiet.propTypes = {
	onChange: PropTypes.func.isRequired,
	value: PropTypes.string.isRequired,
	error: PropTypes.string,
};

const styles = () => ({
	root: {

	},
	removeButton: {
		color: "white",
		backgroundColor: "red",
	},
	textField: {
		width: "25em",
	},
});

export default withStyles(styles)(InputDiet);
