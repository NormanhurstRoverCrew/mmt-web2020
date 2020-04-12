import React, { Component, useContext } from "react";
import {
	Typography,
	Paper,
	Table,
	TableRow,
	TableCell,
	TableBody,
	Grid,
	Button,
} from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
import BookingOrderTotal from "components/booking/BookingOrderTotal";
import _ from "underscore";
import { useHistory } from "react-router-dom";

import { BookingContext } from "context/BookingContext";

import minusicon from "img/minus.svg";

/**
 * A summary of the current booking. Shows the number of tickets, the price of each ticket and the order total.
 */
const CheckoutSummary = ({ classes }) => {
	const { tickets } = useContext(BookingContext);

	const history = useHistory();

	const editButton = () => {
		history.push("/purchase");
	};

	return (
		<div className={classNames(classes.root)}>
			<BookingOrderTotal className={classNames(classes.orderTotal)} />
			<hr className={classNames(classes.hr)} />
			<Typography variant="h5" className={classNames(classes.title)}>
				Details
			</Typography>
			<Grid
				container
				direction="column"
				className={classNames(classes.tickets)}
			>
				{_.map(tickets, (ticket, i) => {
					const { user } = ticket;
					return (
						<Grid item key={i} className={classNames(classes.ticketItem)}>
							<Typography className={classNames(classes.ticketTitle)}>
								Ticket {i + 1}
							</Typography>
							<Typography className={classNames(classes.userItemGrey)}>
								{user.name}
							</Typography>
							<Typography className={classNames(classes.userItemGrey)}>
								{user.email}
							</Typography>
							<Typography className={classNames(classes.userItemGrey)}>
								{user.mobile}
							</Typography>
							<Typography className={classNames(classes.userItemGrey)}>
								{user.crew}
							</Typography>
						</Grid>
					);
				})}
			</Grid>
			<Button className={classNames(classes.editButton)} onClick={editButton}>
				<img src="/client/img/minus.svg" className={classes.minusIcon} />
				EDIT
			</Button>
		</div>
	);
};

export const styles = (theme) => ({
	root: {
		fontWeight: "bold",
		width: "-webkit-fill-available",
	},
	title: {
		fontWeight: "bold",
		paddingBottom: "0.8em",
	},
	orderTotal: {
		paddingBottom: "2em",
	},
	noBorder: {
		border: "none",
	},
	minusIcon: {
		width: "16px",
		height: "16px",
		objectFit: "contain",
		marginRight: "0.2em",
	},
	editButton: {
		color: theme.palette.primary.main,
		marginTop: "1em",
	},
	hr: {
		borderColor: "black",
		marginTop: "2em",
		marginBottom: "2em",
	},
	ticketTitle: {
		fontWeight: "bold",
		paddingBottom: "0.5em",
	},
	ticketItem: {
		paddingBottom: "1em",
	},
	userItemGrey: {
		color: "#555555",
	},
	tickets: {},
});

export default withStyles(styles)(CheckoutSummary);
