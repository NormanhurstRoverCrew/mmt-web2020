import React, {Component, useContext} from "react";
import {Typography, Paper, Table, TableRow, TableCell, TableBody, Grid} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import {BookingContext} from "context/BookingContext";


const TICKET_PRICE = 40.0;

/**
 * Coverts a float to a string in the Australian money format
 * @param {float} num Number to be converted to a Doller value '$000.00'
 * @return {string}
 */
function moneyFormat(num) {
	return `$${num.toFixed(2)}`;
}

/**
 * A summary of the current booking. Shows the number of tickets, the price of each ticket and the order total.
 */
const BookingOrderTotal = ({classes}) => {
		const {tickets, price} = useContext(BookingContext);
		const nTickets = tickets.length;

		const ticketsTotal = price;

		const finalTotal = ticketsTotal;

		if (tickets === 0) return (<div />);
		return (
			<div
				className={classNames(classes.root)}>
				<Typography variant="h5"
					className={classNames(classes.title)}>Booking Summary</Typography>
				<Table padding="none">
					<TableBody>
						<TableRow>
							<TableCell className={classNames(classes.noBorder)}>MMT 2020 Ticket</TableCell>
							<TableCell className={classNames(classes.noBorder)}
								align="right"
								id="ticketprice">Amount</TableCell>
						</TableRow>
						<TableRow>
							<TableCell className={classNames(classes.noBorder, classes.bold)}>{moneyFormat(TICKET_PRICE)}</TableCell>
							<TableCell className={classNames(classes.noBorder, classes.bold)}
								align="right">{nTickets}</TableCell>
						</TableRow>
					</TableBody>
				</Table>
				<hr className={classNames(classes.hr)}/>
				<Typography>Total</Typography>
				<Typography variant="h3"
					className={classNames(classes.grandTotal)}>{moneyFormat(finalTotal)}</Typography>
			</div>
		);
};

export const styles = (theme) => ({
	root: {
		// paddingTop: "25vh",
		// paddingLeft: theme.spacing(8),
		fontWeight: "bold",
		width: "-webkit-fill-available",
	},
	title: {
		fontWeight: "bold",
		paddingBottom: "1em",
	},
	grandTotal: {
		fontWeight: "bold",
		color: "black",
	},
	noBorder: {
		border: "none",
	},
	hr: {
		borderColor: "black",
		marginTop: "4em",
		marginBottom: "2em",
	},
	bold: {
		fontWeight: "bold",
	},
});

export default withStyles(styles)(BookingOrderTotal);
