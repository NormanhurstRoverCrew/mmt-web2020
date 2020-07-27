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
import {Thankyou} from "views/register/Checkout";

import { BookingContext } from "context/BookingContext";

import { Elements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import Stripe from "components/payment/Stripe";

// Make sure to call `loadStripe` outside of a component’s render to avoid
// recreating the `Stripe` object on every render.
const stripePromise = loadStripe("pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap");

const Checkout = ({ classes }) => {
	const { checkoutMethod, updateCheckoutMethod, tickets, bookingId } = useContext(
		BookingContext
	);

	const [billing_details, updateBD] = useState();

	useEffect(() => {
		if (tickets && tickets.length > 0) {
			console.log(tickets);
			const { name, email, mobile: phone } = tickets[0].user;
			updateBD({
				name,
				email,
				phone,
			});
		}
	}, [tickets]);

	const history = useHistory();

	const gotoHome = (e) => {
		history.push("/");
	};

	const handlePaymentOptionChange = (e) => {
		updateCheckoutMethod(e.target.value);
	};

	return (
		<div className={classNames(classes.root)}>
			<Norse
				variant="h4"
				className={classNames(classes.text, classes.bold, classes.title)}
			>
				Checkout
			</Norse>
			<Typography className={classNames(classes.padBottom)}>
				Checkout with your ticket or add more tickets to your
				registration.
			</Typography>
			<Typography
				variant="h5"
				className={classNames(classes.bold, classes.padBottom)}
			>
				Select an option
			</Typography>
			<RadioGroup
				aria-label="Payment method"
				name="payment_method"
				className={classNames(classes.group)}
				value={checkoutMethod}
				onChange={handlePaymentOptionChange}
			>
				<FormControlLabel
					value="stripe"
					control={<Radio />}
					label="Card payment via Stripe"
				/>
				<FormControlLabel
					value="eft"
					control={<Radio />}
					label="Electronic Funds Transfer (EFT)"
					/* disabled */
				/>
				<FormControlLabel
					value="cash"
					control={<Radio />}
					label={"Cash on the day"}
				/>
			</RadioGroup>
			<hr className={classNames(classes.hr)} />
			{checkoutMethod == "eft" && (
				<Grid item>
					<Eft tickets={tickets && tickets.length} />
					<Thankyou classes={classes} gotoHome={gotoHome} />
				</Grid>
			)}
			{checkoutMethod == "stripe" && (
				<Grid item>
					<Elements stripe={stripePromise}>
						<Stripe billing_details={billing_details} bookingId={bookingId} />
					</Elements>
					<Thankyou classes={classes} gotoHome={gotoHome} />
				</Grid>
			)}
			{checkoutMethod == "cash" && (
				<Grid item>
					<Cash tickets={tickets && tickets.length} />
					<Thankyou classes={classes} gotoHome={gotoHome} />
				</Grid>
			)}
		</div>
	);
};

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
		backgroundColor: "white",
		[theme.breakpoints.down("xs")]: {
			padding: "3em 2em",
		},
	},
	text: {
		color: theme.palette.text.primary,
	},
	bold: {
		fontWeight: "bold",
	},
	padBottom: {
		paddingBottom: "1em",
	},
	title: {
		paddingBottom: "0.8em",
	},
	group: {
		paddingBottom: "2em",
	},
});

export default withStyles(styles)(Checkout);
