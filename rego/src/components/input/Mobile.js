import React, {Component} from "react";
import {FormControl, InputLabel, Input, FormHelperText} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import PropTypes from "prop-types";

import MaskedInput from "react-text-mask";

import EXAMPLES from "rego/examples";

function TextMaskMobile(props) {
	const {inputRef, ...other} = props;

	return (
		<MaskedInput
			{...other}
			ref={(ref) => {
				inputRef(ref ? ref.inputElement : null);
			}}
			mask={[/\d/, /\d/, /\d/, /\d/, " ", /\d/, /\d/, /\d/, " ", /\d/, /\d/, /\d/]}
			placeholderChar={"\u2000"}
		/>
	);
}

export class InputMobile extends Component {
	constructor(props) {
		super(props);

		this.state = {error: undefined};

		this.onInputUpdate = this.onInputUpdate.bind(this);
	}

	onInputUpdate(e) {
		const mob = e.target.value;
		const mobile = mob.replace(/\s/g, "");

		if (mobile.length != 10) {
			if (mobile.length > 10) {
				this.setState({error: "Mobile number is too long"});
			} else {
				this.setState({error: "Mobile number is too short"});
			}
		} else {
			this.setState({error: undefined});
		}

		this.props.onChange(e, e.target.value);
	}

	render() {
		const {classes, value} = this.props;
		return (
			<FormControl className={classes.textField}
				margin="normal"
				error={typeof this.state.error !== "undefined"}
			>
				<InputLabel>Mobile</InputLabel>
				<Input
					id="mobile"
					value={value}
					onChange={this.onInputUpdate}
					inputComponent={TextMaskMobile}
				/>
				<FormHelperText>{this.props.error || this.state.error || ""}</FormHelperText>
			</FormControl>
		);
	}
}

InputMobile.propTypes = {
	onChange: PropTypes.func.isRequired,
	value: PropTypes.string.isRequired,
	error: PropTypes.string,
};

const styles = () => ({
	textField: {
		width: "25em",
	},
});

export default withStyles(styles)(InputMobile);
