import React, { Component, useContext, useState, useEffect } from "react";
import { Typography, Button, Grid, Paper, Hidden } from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";
import _ from "underscore";
import { gql } from "apollo-boost";
import { useQuery } from "@apollo/react-hooks";
import { useMutation } from "@apollo/react-hooks";
import { useHistory } from "react-router-dom";

import Theme from "components/theme/Theme";
import TicketBasicInput from "components/tickets/TicketBasicInput";
import Norse from "components/theme/Norse";
import TicketBasics from "components/tickets/TicketBasics";
import BookingOrderTotal from "components/booking/BookingOrderTotal";

import { BookingContext } from "context/BookingContext";

import plusicon from "img/plus.svg";

const ADD_UPDATE_TICKETS = gql`
	mutation AddUpdateTickets(
		$update_tickets: [TicketUpdate!]!
		$add_booking_id: String!
		$add_users: [BasicUser!]!
	) {
		updateTickets(tickets: $update_tickets) {
			id
		}
		addTicketsToBooking(bookingId: $add_booking_id, users: $add_users) {
			id
			tickets {
				id
				user {
					id
					name
					email
					mobile
					crew
				}
			}
		}
	}
`;

const Purchase = ({ classes }) => {
	const {
		userId,
		bookingId,
		updateBookingId,
		tickets,
		updateTickets,
		addTicket,
			removeTicket,
	} = useContext(BookingContext);

	const [error, setError] = useState(false);

	const [
		pushTickets,
		{ data: ticketData, loading: ticketLoading, ticketError },
	] = useMutation(ADD_UPDATE_TICKETS);

	const history = useHistory();

	const updateUser = (e, id) => {
		var ticket = _.clone(tickets);
		const t = _.findIndex(ticket, (t) => id === t.id);
		const { id: tid, value } = e.target;
		ticket[t].user[tid] = value;
		ticket[t].updated = true;
		updateTickets(ticket);

		// this.props.updateUser({
		// 	[e.target.id]: e.target.value,
		// });
	};

	const checkoutButton = () => {
		setError(false);
		const newTickets = _.filter(tickets, (t) => t.newTicket);
		const udTickets = _.filter(tickets, (t) => !t.newTicket && t.updated);

		_.each(newTickets, (ticket) => {
			const { user } = ticket;
			const strippedMobile = user.mobile && user.mobile.replace(/\s/g, "");
			if (
				(strippedMobile && strippedMobile.length < 10) ||
				strippedMobile == ""
			) {
				setError({ error: "You must enter a valid mobile for all tickets" });
			}
		});

		if (udTickets.length > 0 || newTickets.length > 0) {
			pushTickets({
				variables: {
					add_booking_id: bookingId,
					add_users: _.map(newTickets, (t) => t.user),
					update_tickets: _.map(udTickets, (t) => {
						const { user: u } = t;
						return {
							id: t.id,
							user: {
								name: u.name,
								mobile: u.mobile,
								email: u.email,
								crew: u.crew,
							},
						};
					}),
				},
			});
		}

		history.push("/checkout");
	};

	return (
		<Paper className={classNames(classes.fullHeight, classes.root)}>
			<Norse
				variant="h4"
				className={classNames(
					classes.text,
					classes.bold,
					classes.purchaseTicket
				)}
			>
				Purchase Tickets
			</Norse>
			<Typography>
				Checkout with your ticket or add more tickets to your
				registration.
			</Typography>
			<Grid container direction="column">
				{_.map(tickets, (ticket, i) => {
					const title = "Ticket " + (i + 1);
					return (
						<Grid item key={ticket.id}>
							{i == 0 ? (
								<>
									<TicketBasics title={title} ticket={ticket} />
									<hr className={classNames(classes.divider)} />
								</>
							) : (
								<TicketBasicInput
									key={ticket.id}
									tid={ticket.id}
									title={title}
									remove
									index={i}
									ticket={ticket}
									removeTicket={ticket.newTicket && removeTicket}
									updateUser={updateUser}
								/>
							)}
						</Grid>
					);
				})}
				{tickets && tickets.length == 0 && (
					<Typography className={classNames(classes.red)}>
						Error: There are no tickets in your booking
					</Typography>
				)}
			</Grid>
			<Grid container alignItems="flex-start" direction="column">
				<Button
					className={classNames(classes.addTicketButton, classes.padTop)}
					onClick={addTicket}
				>
					<img src="/client/img/plus.svg" className={classes.plusIcon} />
					Add another ticket
				</Button>
				<Hidden xsDown>
					<Typography className={classNames(classes.red)}>
						{error}
					</Typography>
					<Button
						className={classNames(
							classes.buttonRoot,
							classes.checkoutButton
						)}
						onClick={checkoutButton}
					>
						Checkout
					</Button>
				</Hidden>
			</Grid>
		</Paper>
	);
};

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
		[theme.breakpoints.down("xs")]: {
			padding: "3em 2em",
		},
	},
	red: {
		color: "red",
	},
	fullHeight: {
		height: "fit-content",
		minHeight: "100vh",
		[theme.breakpoints.down("xs")]: {
			minHeight: "fit-content",
		},
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
	padTop: {
		paddingTop: "1em",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	addTicketButton: {
		color: theme.palette.primary.main,
		paddingLeft: 0,
		marginBottom: "2em",
	},
	checkoutButton: {
		color: "white",
	},
	plusIcon: {
		width: "16px",
		height: "16px",
		objectFit: "contain",
		marginRight: "0.2em",
	},
	purchaseTicket: {
		paddingBottom: "0.5em",
	},
	divider: {
		width: "30em",
		margin: 0,
		marginTop: "2em",
	},
});

export default withStyles(styles)(Purchase);
