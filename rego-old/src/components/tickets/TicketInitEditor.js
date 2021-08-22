import React, {Component} from "react";
import {connect} from "react-redux";
import {Button, Grid, Typography} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {bookingActions} from "actions/booking.actions";

import TicketBasicInput from "components/tickets/TicketBasicInput";

/**
 * The editor for adding/removing tickets and editing their users
 */
export class TicketInitEditor extends Component {
	constructor(props) {
		super(props);

		this.addTicket = this.addTicket.bind(this);
		this.remove = this.remove.bind(this);
	}

	/**
	 * Add a new ticket to the booking
	 */
	addTicket() {
		this.props.addTicket();
	}

	/**
	 * Remove
	 */
	remove() {
		this.props.removeTicket(this.props.index);
	}

	render() {
		const {classes, tickets, error} = this.props;
		if (tickets) {
			return (
				<>
					<Grid container
						spacing={2}>
						<Grid item
							xs={11}
						>
							<Grid container
								spacing={2}>
								{tickets.map((ticket, i) => (
									<TicketBasicInput key={i}
										remove
										index={i}
										ticket={ticket}
										updateUser={this.props.updateUser}
									/>
								))}
							</Grid>
						</Grid>

					</Grid>
					<Button
						onClick={(e) => this.addTicket(e)}
						className={classNames(classes.addButton)}
					>
						Add Another Ticket
					</Button>
				</>
			);
		} else if (error) {
			return (
				<Typography variant="h5"
					className={classNames(classes.error)}>
					Error: Either your booking has been deleted, or you have not verified your email. Please check your email to verify. Please contact Grant Perry for assistance If you see this error again.
				</Typography>
			);
		} else {
			return (<></>);
		}
	}
}

const styles = (theme) => ({
	root: {

	},
	addButton: {
		color: "white",
		backgroundColor: "#00cc00",
		margin: 16,
	},
	error: {
		color: "red",
	},
});

const mapStateToProps = (state) => {
	return {
		error: state.booking.error,
	};
};

const mapDispatchToProps = (dispatch) => {
	return {
		updateUser: (index, user) => dispatch(bookingActions.updateTicket(index, user)),
		createUser: () => dispatch(bookingActions.createUser()),
		addTicket: (uid) => dispatch(bookingActions.addTicket(uid)),
		removeTicket: (index) => dispatch(bookingActions.removeTicket(index)),
	};
};

export default connect(mapStateToProps, mapDispatchToProps)(withStyles(styles)(TicketInitEditor));
