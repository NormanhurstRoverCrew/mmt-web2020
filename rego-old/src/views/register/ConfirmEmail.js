import React, { Component, useContext, useEffect, useState } from "react";
import { Typography, Button, Grid, Paper } from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";
import queryString from "query-string";
import { useMutation } from "@apollo/react-hooks";

import Theme from "components/theme/Theme";
import TicketBasicInput from "components/tickets/TicketBasicInput";
import Norse from "components/theme/Norse";
import { useQuery } from "@apollo/react-hooks";
import { gql } from "apollo-boost";
import { useLocation, useHistory } from "react-router-dom";

import RawConfirmEmail from "views/raw/ConfirmEmail";
import { BookingContext } from "context/BookingContext";

const GET_USER = gql`
	query GetName($id: String!) {
		bookingFromUser(id: $id) {
			user {
				name
				email
			}
		}
	}
`;

const VERIFY_USER = gql`
	mutation VerifyUser($uid: String!, $code: String!) {
		verifyUser(id: $uid, code: $code) {
			emailVerified
		}
	}
`;

const ConfirmEmail = ({ classes }) => {
	return (
		<Theme shiftLeft>
			<Grid container direction="row-reverse">
				<Grid
					item
					lg={6}
					// md={6}
					sm={10}
					xs={12}
				>
					<RawConfirmEmail/>
				</Grid>
			</Grid>
		</Theme>
	);
};

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
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
	red: {
			color: "red",
	},
	paragraphGap: {
		paddingBottom: "1.5em",
	},
	buttonTextPrimary: {
		color: "white",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
});

const mapStateToProps = (state) => {
	return {
		booking: state.booking,
		user: state.booking.user,
		activeStep: state.steppers.home,
	};
};

const mapDispatchToProps = (dispatch) => {
	return {
		updateUser: (i, user) => dispatch(bookingActions.updateUser(user)),
		createUser: () => dispatch(bookingActions.createUser()),
		resetStepper: () => dispatch(stepperActions.resetStepper("home")),
		incrementStepper: () => dispatch(stepperActions.incrementStepper("home")),
		decrementStepper: () => dispatch(stepperActions.decrementStepper("home")),
		checkout: () => dispatch(bookingActions.checkout()),
		buy: () => dispatch(push("/buy")),
	};
};

export default withStyles(styles)(ConfirmEmail);
