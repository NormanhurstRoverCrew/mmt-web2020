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

const RawRegister = ({ classes }) => {
	const history = useHistory();
	const [addUser, { data, loading, error }] = useMutation(NEW_USER);
	const [errors, setErrors] = useState({});
	const [user, setUser] = useState({
		name: "Grant Perry",
		email: "grant42perry@gmail.com",
		mobile: "0439675452",
		crew: "Normanhurst Rovers",
	});
	const updateUser = (u) => setUser(_.extend({}, user, u));

	const { updateUserId } = useContext(BookingContext);

	useEffect(() => {
		console.log(loading, error, data);
		if (!loading && !error && data) {
			console.log(data.newUser);
			const { id } = data.newUser;
			updateUserId(id);
			history.push("/confirm_email");
		}
	}, [data]);

	if (error) return <>{error}</>;

	return (
			<>
				<Typography
					variant="h5"
					className={classNames(classes.text, classes.bold)}
				>
					Your details
				</Typography>
				<User user={user} updateUser={updateUser} errors={errors} />
				<div className={classes.actionsContainer}>
					<Button
						onClick={(e) => {
							addUser({ variables: { user } });
						}}
						color="primary"
						variant="contained"
						size="large"
						classes={{
							root: classes.buttonRoot,
							textPrimary: classes.buttonTextPrimary,
						}}
					>
						{!loading ? "Register" : "Loading"}
					</Button>
				</div>
			</>
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

export default withStyles(styles)(RawRegister);
