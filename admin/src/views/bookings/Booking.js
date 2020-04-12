import React, {Component} from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import {withStyles} from '@material-ui/core/styles';
import Title from 'components/common/Title';
import BookingTable from 'views/bookings/BookingTable';
import {
	Table,
	TableHead,
	TableCell,
	Tab,
	TableRow,
	TableBody,
	Typography,
	Paper,
	Grid,
	Button,
} from '@material-ui/core';
import Ticket from 'views/tickets/Ticket';

import {bookingsActions} from 'actions/bookings.actions';

export const Booking = ({classes, booking, deleteBooking}) => {
	const {tickets, user} = booking;

	const deleteBookingHandler = e => {
		e.stopPropagation();
		e.preventDefault();
		deleteBooking(booking);
	};

	const renderedTickets = tickets.map(ticket => {
		return (
			<Grid item xs={6} lg={3} md={2} key={ticket.id}>
				<Ticket ticket={ticket} />
			</Grid>
		);
	});

	return (
		<Paper className={classNames(classes.paper)}>
			<Grid container spacing={2} alignItems="flex-start">
				<Grid item>
					<Typography variant="h5">{(user && user.name) || ' - '}</Typography>
					<Typography variant="subtitle1">
						{(tickets && tickets.length) || ' # '} Tickets
					</Typography>
				</Grid>
				<Grid item>
					<Grid container justify="flex-end" alignItems="flex-start">
						<Grid item>
							<Button
								className={classNames(classes.deleteButton)}
								onClick={deleteBookingHandler}>
								Delete
							</Button>
						</Grid>
					</Grid>
				</Grid>
			</Grid>
			<Grid container spacing={2}>
				{renderedTickets}
			</Grid>
		</Paper>
	);
};

Booking.propTypes = {
	classes: PropTypes.object.isRequired,
	booking: PropTypes.object.isRequired,
};

const styles = theme => ({
	paper: {
		padding: theme.spacing(2),
		margin: theme.spacing(1),
		textAlign: 'justify',
		color: theme.palette.text.secondary,
	},
	deleteButton: {
		color: 'red',
	},
});

export default withStyles(styles)(Booking);
