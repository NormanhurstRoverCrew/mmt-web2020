import React, { Component, useEffect, useContext } from "react";
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
import Stripe from "components/payment/Stripe";
import { Elements } from "react-stripe-elements";
import Cash from "components/payment/Cash";
import CheckoutSummary from "components/booking/CheckoutSummary";

import { BookingContext } from "context/BookingContext";

const Checkout = ({ classes, tickets }) => {
	const { checkoutMethod, updateCheckoutMethod } = useContext(BookingContext);

	const history = useHistory();

	const gotoHome = (e) => {
		history.push("/");
	};

	const handlePaymentOptionChange = (e) => {
		updateCheckoutMethod(e.target.value);
	};

	return (
		<Theme noBG>
			<Grid container direction="row">
				<Grid item sm={7} xs={12}>
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
							{/* <FormControlLabel value="stripe"
									control={<Radio />}
									label="Card payment / Apple Pay / Google Pay via Stripe" /> */}
							<FormControlLabel
								value="eft"
								control={<Radio />}
								label="Electronic Funds Transfer (EFT) ( Presale finished 5pm Wed 4th of September )"
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
								<Elements>
									<Stripe />
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
};

const Thankyou = (props) => (
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
	root: {
		padding: "6em 4em",
		backgroundColor: "white",
		[theme.breakpoints.down("xs")]: {
			padding: "3em 2em",
		},
	},
	red: {
		color: "red",
	},
	heading: {
		textAlign: "center",
	},
	fullHeight: {
		height: "fit-content",
		minHeight: "100vh",
		borderRadius: "0",
		bottom: 0,
		top: 0,
		overflow: "auto",
	},
	text: {
		color: theme.palette.text.primary,
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1em",
	},
	padBottom: {
		paddingBottom: "1em",
	},
	title: {
		paddingBottom: "0.8em",
	},
	paragraphGap: {
		paddingBottom: "1.5em",
	},
	buttonTextPrimary: {
		color: "white",
	},
	group: {
		paddingBottom: "2em",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	addTicketButton: {
		color: theme.palette.primary.main,
	},
	backButton: {
		color: "white",
	},
	label: {
		fontSize: "0.7em",
		marginBottom: "-0.6em",
		color: theme.palette.text.secondary,
	},
	data: {
		fontSize: "1.0em",
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
