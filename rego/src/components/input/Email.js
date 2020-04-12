import React, {Component} from "react";
import {FormControl, InputLabel, Input, FormHelperText} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import PropTypes from "prop-types";

import MaskedInput from "react-text-mask";
import emailMask from "text-mask-addons/dist/emailMask";

import EXAMPLES from "rego/examples";


export class InputEmail extends Component {
	constructor(props) {
		super(props);

		this.state = {error: undefined};

		this.onInputUpdate = this.onInputUpdate.bind(this);
	}

	onInputUpdate(e) {
		const email = e.target.value;
		// const splitAt = email.split("@");
		// if (splitAt[0].length < 3) {
		// 	this.setState({error: "Your email username is too short"});
		// } else {
		// 	const splitDot = splitAt[1].split(".");
		// 	if (splitDot[0].length < 2 || splitDot[1].length < 2) {
		// 		this.setState({error: "Your email host is not correct"});
		// 	} else {
		// 		this.setState({error: undefined});
		// 	}
		// }

		this.props.onChange(e, e.target.value);
	}

	render() {
		const {classes, value} = this.props;
		return (
			<FormControl className={classes.textField}
				margin="normal"
				error={typeof this.state.error !== "undefined"}
			>
				<InputLabel>Email</InputLabel>
				<Input
					id="email"
					value={value}
					onChange={this.onInputUpdate}
				/>
				<FormHelperText>{this.props.error || this.state.error || ""}</FormHelperText>
			</FormControl>
		);
	}
}

InputEmail.propTypes = {
	onChange: PropTypes.func.isRequired,
	value: PropTypes.string.isRequired,
	error: PropTypes.string,
};

const styles = () => ({
	textField: {
		width: "25em",
	},
});

export default withStyles(styles)(InputEmail);
