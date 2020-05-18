import React, { useState, useEffect } from "react";
import { Typography, Paper, Button } from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
// import { injectStripe, PaymentRequestButtonElement, CardElement } from "react-stripe-elements";
import {
	useStripe,
	useElements,
	CardElement,
	PaymentRequestButtonElement,
} from "@stripe/react-stripe-js";
import axios from "axios";
import { useMutation } from "@apollo/react-hooks";
import gql from "graphql-tag";

const CARD_ELEMENT_OPTIONS = {
	style: {
		base: {
			color: "#32325d",
			fontFamily: '"Helvetica Neue", Helvetica, sans-serif',
			fontSmoothing: "antialiased",
			fontSize: "16px",
			"::placeholder": {
				color: "#aab7c4",
			},
		},
		invalid: {
			color: "#fa755a",
			iconColor: "#fa755a",
		},
	},
};

const ATTACH_PAYMENT_METHOD = gql`
	mutation AttachPaymentMethod($bookingId: String!, $paymentMethodId: String!) {
		attachStripePaymentMethodToBooking(
			bookingId: $bookingId
			paymentMethodId: $paymentMethodId
		)
	}
`;

const ConfirmStripe = ({ onClick }) => {
	return (
		<>
			<Button onClick={onClick}>Confirm Payment</Button>
		</>
	);
};

export const Stripe = ({ billing_details, bookingId }) => {
	const stripe = useStripe();
	const elements = useElements();
	const [error, setError] = useState();

	const [attachPM, { data, loading, error: attachPMError }] = useMutation(
		ATTACH_PAYMENT_METHOD
	);

	const handleSubmit = async (event) => {
		// We don't want to let default form submission happen here,
		// which would refresh the page.
		event.preventDefault();

		if (!stripe || !elements) {
			// Stripe.js has not yet loaded.
			// Make  sure to disable form submission until Stripe.js has loaded.
			return;
		}

		const card = elements.getElement(CardElement);
		const pm = await stripe.createPaymentMethod({
			type: "card",
			card,
			billing_details,
		});

		if (pm.error) {
			// Show error to your customer.
			setError(pm.error.message);
		} else {
			// Send the token to your server.
			// This function does not exist yet; we will define it in the next step.
			console.log(pm);
			attachPM({
				variables: {
					bookingId,
					paymentMethodId: pm.paymentMethod.id,
				},
			});
		}
	};

	const [capturePayment, updateCapturePayment] = useState(false);
	const [paymentIntentId, updatePaymentIntentId] = useState();
	useEffect(() => {
		if (data) {
			updatePaymentIntentId(data.attachStripePaymentMethodToBooking);
		}
	}, [data]);

	useEffect(() => {
		if (paymentIntentId) {
			updateCapturePayment(true);
		}
	}, [paymentIntentId]);

	const confirmPayment = () => {
		stripe.confirmCardPayment(paymentIntentId).then((result) => {
			console.log(result);
		});
	};

	return (
		<>
			<form onSubmit={handleSubmit}>
				<Typography>Card details</Typography>
				<CardElement options={CARD_ELEMENT_OPTIONS} />
				<Typography>{error}</Typography>
				<Button disabled={!stripe} type="submit">Confirm order</Button>
			</form>
			{capturePayment && <ConfirmStripe onClick={() => confirmPayment()} />}
		</>
	);
};

export default Stripe;
