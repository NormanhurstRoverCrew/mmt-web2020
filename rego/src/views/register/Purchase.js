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

import PurchaseTicketsForm from "views/raw/PurchaseTicketsForm";
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
	return (
		<Theme>
			<Grid container direction="row" spacing={0}>
				<Grid item sm={7} xs={12} className={classNames(classes.scroll)}>
					<PurchaseTicketsForm/>
				</Grid>
				<Grid
					item
					sm={5}
					xs={12}
					className={classNames(classes.orderTotalArea)}
				>
					<BookingOrderTotal />
				</Grid>
			</Grid>
		</Theme>
	);
};

const styles = (theme) => ({
	orderTotalArea: {
		backgroundColor: "#f4f5f7",
		padding: "25vh 10vw 10vh 5vw",
		[theme.breakpoints.down("xs")]: {
			paddingTop: "5vh",
		},
	},
	scroll: {
		overflow: "auto",
		height: "100vh",
		[theme.breakpoints.down("xs")]: {
			height: "unset",
		},
	},
});

export default withStyles(styles)(Purchase);
