import React, { Component, useEffect, useContext, useState } from "react";
import {
	Typography,
	Button,
	Grid,
	Paper,
	RadioGroup,
	FormControlLabel,
	Radio,
} from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";
import { useHistory } from "react-router-dom";
import _ from "underscore";

import Theme from "components/theme/Theme";
import Norse from "components/theme/Norse";
import TicketBasics from "components/tickets/TicketBasics";
import Eft from "components/payment/Eft";
import Cash from "components/payment/Cash";
import CheckoutSummary from "components/booking/CheckoutSummary";

import CheckoutSelect from "views/raw/CheckoutSelect";
import { BookingContext } from "context/BookingContext";

import { Elements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import Stripe from "components/payment/Stripe";

// Make sure to call `loadStripe` outside of a component’s render to avoid
// recreating the `Stripe` object on every render.
const stripePromise = loadStripe("pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap");

const Checkout = ({ classes }) => (
	<Theme noBG>
		<Grid container direction="row">
			<Grid item sm={7} xs={12}>
				<CheckoutSelect/>
			</Grid>
			<Grid
				item
				sm={5}
				xs={12}
				className={classNames(classes.checkoutSummaryArea)}
				container
				justify="flex-start"
			>
				<CheckoutSummary />
			</Grid>
		</Grid>
	</Theme>
);

export const Thankyou = (props) => (
	<>
		<Typography variant="h5" className={classNames(props.classes.thankyouText)}>
			Thank you for registering for MMT 2019.
		</Typography>
		<Button
			className={classNames(
				props.classes.buttonRoot,
				props.classes.thankyouButton
			)}
			onClick={props.gotoHome}
		>
			Return Home
		</Button>
	</>
);

const styles = (theme) => ({
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	checkoutSummaryArea: {
		backgroundColor: "#f4f5f7",
		padding: "25vh 10vw 10vh 5vw",
		minHeight: "100vh",
		[theme.breakpoints.down("xs")]: {
			paddingTop: "5vh",
		},
	},
	thankyouText: {
		fontWeight: "bold",
		paddingTop: "1em",
		paddingBottom: "1em",
	},
	thankyouButton: {
		color: "white",
	},
});

export default withStyles(styles)(Checkout);
