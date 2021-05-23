import React, {Component} from "react";
import PropTypes from "prop-types";
import {Button, Grid, Typography} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";

import Email from "components/input/Email";
import Rego from "components/input/Rego";

import minusicon from "img/minus.svg";

const Checkin = ({checkinState, updateCheckinState, errors, classes}) => (
	<Grid container
		item
		xs={12}
		spacing={2}
		direction="column">
		<Grid item>
			<Email
				onChange={(e) => {
					updateCheckinState({email: e.target.value});
				}}
				value={checkinState.email}
			/>
			// <Error>{errors.email || ""}</Error>
		</Grid>

		<Grid item>
			<Rego
				onChange={(e) => {
					updateCheckinState({rego: e.target.value});
				}}
				value={checkinState && checkinState.rego}
			/>
			// <Error>{errors.rego || ""}</Error>
		</Grid>
	</Grid>
);

const styles = (theme) => ({
	root: {
		paddingTop: "1em",
	},
	removeButton: {
		color: theme.palette.primary.main,
		marginTop: "1.3em",
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1vw",
	},
	minusIcon: {
		width: "16px",
		height: "16px",
		objectFit: "contain",
		marginRight: "0.2em",
	},
});

export default withStyles(styles)(Checkin);
