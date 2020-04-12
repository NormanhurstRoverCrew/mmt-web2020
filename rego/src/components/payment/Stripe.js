import React, { Component } from "react";
import { Typography, Paper } from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
import { injectStripe, PaymentRequestButtonElement, CardElement } from "react-stripe-elements";
import axios from "axios";


export class Stripe extends Component {
	constructor(props) {
		super(props);

		axios.post(`/stripe/checkout/${this.props.bookingUid}`)
			.then((resp) => {
				console.log(resp);

				const {data: {sessionId}} = resp;

				props.stripe.redirectToCheckout({
					sessionId,
				}).then(function (result) {
					// If `redirectToCheckout` fails due to a browser or network
					// error, display the localized error message to your customer
					// using `result.error.message`.
					console.log(result);
				});
			})
			.catch((err) => {
				console.log("Caught Error", err);
				dispatch(push("/"));
			});


	}

	render() {
		return (
			<></>
		);
	}
}

const styles = (theme) => ({
	root: {

	},
	bold: {
		fontWeight: "bold",
	},
});

export default injectStripe(Stripe);
