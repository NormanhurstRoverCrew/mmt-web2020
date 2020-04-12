import React, {Component} from "react";
import {TextField} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import PropTypes from "prop-types";
import _ from "underscore";

import CREWS from "rego/crews";
import EXAMPLES from "rego/examples";

export class InputCrew extends Component {
	constructor(props) {
		super(props);

		this.state = {error: undefined};

		this.onInputUpdate = this.onInputUpdate.bind(this);
	}

	onInputUpdate(e) {
		const crew = e.target.value;
		if (_.indexOf(CREWS, crew) == -1) {
			this.setState({error: crew + " is not a valid NSW Crew"});
		} else {
			this.setState({error: undefined});
		}

		this.props.onChange(e, e.target.value);
	}

	render() {
		const {classes, value} = this.props;
		return (
			<TextField
				id="crew"
				select
				error={typeof this.state.error !== "undefined"}
				label="Crew"
				className={classes.textField}
				value={value}
				onChange={this.onInputUpdate}
				SelectProps={{
					native: true,
					MenuProps: {
						className: classes.menu,
					},
				}}
				helperText={this.props.error || this.state.error || ""}
				margin="normal"
			>
				<option key=""
					value="" />
				{CREWS.map((option) => (
					<option key={option}
						value={option}>
						{option}
					</option>
				))}
			</TextField>
		);
	}
}

InputCrew.propTypes = {
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

export default withStyles(styles)(InputCrew);
