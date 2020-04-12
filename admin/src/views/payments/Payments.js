import React, {Component} from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import Title from "components/common/Title";
import StripePaymentTable from "views/payments/StripePaymentTable";
import BookingPaymentTable from "views/payments/BookingPaymentTable";

export class Payments extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes} = this.props;

		return (
			<>
			<Title>Payments</Title>
			<BookingPaymentTable />
			{/* <StripePaymentTable /> */}
			</>
		);
	}
}

Payments.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

Payments.propTypes = {
};

export default withStyles(styles)(Payments);
