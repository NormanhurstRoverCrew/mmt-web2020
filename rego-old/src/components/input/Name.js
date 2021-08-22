import React, {Component} from "react";
import {TextField} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import PropTypes from "prop-types";

export class InputName extends Component {
	constructor(props) {
		super(props);

		this.state = {
			name: null,
		};

		this.onInputUpdate = this.onInputUpdate.bind(this);
	}

	onInputUpdate(e) {
		const name = e.target.value;
		const split = name.split(" ");
		if (split.length < 2) {
			this.setState({error: "Please type your Full Name"});
		} else {
			this.setState({error: undefined});
		}

		this.props.onChange(e, e.target.value);
	}

	render() {
		const {classes, value} = this.props;
		return (
			<TextField
				error={typeof this.state.error !== "undefined"}
				id="name"
				label="Name"
				className={classNames(classes.textField)}
				value={value}
				onChange={this.onInputUpdate}
				margin="normal"
				helperText={this.props.error || this.state.error || ""}
			/>
		);
	}
}

InputName.propTypes = {
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
		maxWidth: "25em",
		width: "-webkit-fill-available",
	},
});

export default withStyles(styles)(InputName);
