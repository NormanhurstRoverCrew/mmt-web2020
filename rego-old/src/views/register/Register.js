import _ from "underscore";
import React, { Component, useState, useEffect, useContext } from "react";
import { Typography, Button, Grid, Paper } from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";

import Theme from "components/theme/Theme";
import User from "components/input/User";
import Norse from "components/theme/Norse";
import { useMutation } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { useHistory } from "react-router-dom";

import RawRegister from "views/raw/Register.js";
import { BookingContext } from "context/BookingContext";

const NEW_USER = gql`
	mutation NewUser($user: BasicUser!) {
		newUser(user: $user) {
			id
			name
			email
			crew
			mobile
		}
	}
`;

const Register = ({ classes }) => {
	return (
		<Theme shiftLeft>
			<Grid container direction="row-reverse">
				<Grid item lg={6} sm={10} xs={12}>
					<Paper className={classNames(classes.fullHeight, classes.root)}>
						<Norse
							className={classNames(classes.text, classes.bold, classes.regnow)}
							variant="h3"
						>
							Register Now
						</Norse>
						<RawRegister/>
					</Paper>
				</Grid>
			</Grid>
			{/* <Typography component="pre">{JSON.stringify(this.props.booking, undefined, 8)}</Typography> */}
		</Theme>
	);
};

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
		[theme.breakpoints.down("xs")]: {
			padding: "4em 1em",
		},
		overflow: "auto",
	},
	heading: {
		textAlign: "center",
	},
	fullHeight: {
		height: "100vh",
		borderRadius: "0",
		bottom: 0,
		top: 0,
	},
	text: {
		color: theme.palette.text.primary,
	},
	bold: {
		fontWeight: "bold",
	},
	buttonTextPrimary: {
		color: "white",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	regnow: {
		paddingBottom: "0.5em",
	},
});

export default withStyles(styles)(Register);
