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
	const history = useHistory();
	const { search } = useLocation();
	const [
		verifyUser,
		{ data: verifyUserData, loading: verifyUserLoading },
	] = useMutation(VERIFY_USER);
	const { userId, updateUserId } = useContext(BookingContext);
	const [name, setName] = useState("Loading");
	const [email, setEmail] = useState("Loading");
	const [errors, setErrors] = useState();

	useEffect(() => {
		const { uid, code } = queryString.parse(search);
		updateUserId(uid);
		if (uid && code) {
			verifyUser({ variables: { uid, code } });
		}
	}, [search]);

	useEffect(() => {
		if (verifyUserData && verifyUserData.verifyUser.emailVerified) {
			history.push("/purchase");
		}
	}, [verifyUserData, verifyUserLoading]);

	const { error: getUserError, data: getUserData } = useQuery(GET_USER, {
		variables: {
			id: userId,
		},
	});

	useEffect(() => {
		if (
			getUserData &&
			getUserData.bookingFromUser &&
			getUserData.bookingFromUser.user
		) {
			setName(getUserData.bookingFromUser.user.name.split(" ")[0]);
			setEmail(getUserData.bookingFromUser.user.email);
		}
	}, [getUserData]);

	useEffect(() => {
		if (getUserError) {
			setErrors(JSON.stringify(getUserError));
		}
	}, [getUserError]);

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
					<Paper className={classNames(classes.fullHeight, classes.root)}>
						{errors}
						<Typography
							variant="h5"
							className={classNames(
								classes.text,
								classes.bold,
								classes.paragraphGap
							)}
						>
							Hello {name}, Please confirm your email
						</Typography>
						<Typography className={classNames(classes.paragraphGap)}>
							Thanks for registering. We’ve sent you an email from{" "}
							<b>bookings@normorovers.com</b> to <b>{email}</b>. Please click on
							the link in your email to finalise your tickets.
						</Typography>
						<Typography
							className={classNames(classes.bold, classes.paragraphGap)}
						>
							Haven’t received an email?
						</Typography>
						<Typography className={classNames(classes.paragraphGap)}>
							Please check your Junk/Spam folder. If the email does not appear
							in your inbox within the next 5 min, please contact Grant Perry.
						</Typography>
						<Typography>
							You may close this window if you have received your email.
						</Typography>
					</Paper>
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
