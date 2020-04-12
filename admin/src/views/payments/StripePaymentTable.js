import React from "react";
import {connect} from "react-redux";
import _ from "underscore";

import MaterialTable from "material-table";
import {Grid, Modal} from "@material-ui/core";
import PaymentTableDetails from "views/payments/PaymentTableDetails";

import {stripeActions} from "actions/stripe.actions";


export class StripePaymentTable extends React.Component {
	constructor(props) {
		super(props);

		this.state = {
			payments: [],
		};

		this.getCharges = this.getCharges.bind(this);
	}

	async getCharges() {
		const charges = await stripeActions.getAllCharges();

		const mappedCharges = _.map(charges, (charge) => {
			charge.booking = _.find(this.props.bookings, (booking) => {
				return booking.uid == charge.booking.uid;
			});

			return charge;
		});


		console.log(mappedCharges);

		this.setState({payments: mappedCharges});
	}

	async componentDidMount() {
		this.getCharges();
	}

	render() {
		return (
			<>
				<Grid item
					xs={12}>
					<MaterialTable
						columns={[
							{title: "Booking ID", field: "booking.id", type: "numeric"},
							{title: "Payment ID", field: "id"},
							{title: "Name", field: "booking.user.name"},
							{title: "crew", field: "booking.user.crew"},
							{title: "Total Paid", field: "amount", type: "currency"},
						]}
						data={this.state.payments}
						title="Stripe Payments"
						options={{
							pageSize: 20,
							pageSizeOptions: [20, 50, 100, 200, 250],
						}}
					/>
				</Grid>
			</>
		);
	}
}

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
		// take all the tickets from each booking...
		// this will return n(bookings) arrays of tickets.
		// flatten them into a linear array...
		bookings: _.flatten(state.bookings),
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
	};
};

export default connect(mapSToP, mapDispatchToProps)(StripePaymentTable);
