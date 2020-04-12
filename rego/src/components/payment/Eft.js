import React, {Component, useContext, createRef} from "react";
import {Typography, Button, Grid, Paper, Table, TableBody, TableRow, TableCell, InputBase} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import Copy from "components/input/Copy";

import {BookingContext} from "context/BookingContext";

const Eft = ({classes}) => {
		const {tickets, bookingNo, price} = useContext(BookingContext);
		const nTickets = tickets.length;

		return (
			<div className={classes.root}>
				<Typography variant="h5"
					className={classNames(classes.bold, classes.padTop, classes.pad)}>
					Electronic Funds Transfer Bank Details
				</Typography>
				<Typography
					className={classNames(classes.pad)}>
					Please make sure to use enter the description so we know which payment was yours...
				</Typography>
				<Typography
					className={classNames(classes.pad)}>
					Click on each line to copy it to your clipboard!
				</Typography>
				<Grid container
					direction="column"
					className={classNames(classes.detailsContainer)}>
					<Grid item
						container
						direction="row"
						className={classNames(classes.row)}>
						<Grid item
							xs={3}>
							<Typography>Payee</Typography>
						</Grid>
						<Grid item>
							<Copy text="2nd Normanhurst Rover Crew" />
						</Grid>
					</Grid>
					<Grid item
						container
						direction="row"
						className={classNames(classes.row)}>
						<Grid item
							xs={3}>
							<Typography>BSB</Typography>
						</Grid>
						<Grid item>
							<Copy text="032-186" />
						</Grid>
					</Grid>
					<Grid item
						container
						direction="row"
						className={classNames(classes.row)}>
						<Grid item
							xs={3}>
							<Typography>Account No.</Typography>
						</Grid>
						<Grid item>
							<Copy text="811 413" />
						</Grid>
					</Grid>
					<Grid item
						container
						direction="row"
						className={classNames(classes.row)}>
						<Grid item
							xs={3}>
							<Typography>Description</Typography>
						</Grid>
						<Grid item>
							<Copy text={"MMT20-" + bookingNo} />
						</Grid>
					</Grid>
					<Grid item
						container
						direction="row"
						className={classNames(classes.row)}>
						<Grid item
							xs={3}>
							<Typography>Amount (AUD)</Typography>
						</Grid>
						<Grid item>
							<Copy text={new Intl.NumberFormat("en-AU", {style: "currency", currency: "AUD"}).format(price)} />
						</Grid>
					</Grid>
				</Grid>
			</div>
		);
};

const styles = (theme) => ({
	table: {
		width: "25em",
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1em",
	},
	pad: {
		paddingBottom: "1vw",
	},
	tableRowHeader: {
		fontWeight: "bold",
	},
	copy: {
		"width": "-webkit-fill-available",
		"-webkit-appearance": "none",
		"resize": "none",
		"cursor": "point",
		"border": "none",
		"padding": 0,
		"outline": "none",
		"fontFamily": theme.typography.fontFamily,
		"fontSize": "1em",
	},
	row: {
		padding: "0.5em 0",
	},
	detailsContainer: {
		paddingBottom: "2em",
	},
});

const mapStateToProps = (state) => {
	return {
		bookingID: state.booking.id,
	};
};

const mapDispatchToProps = (dispatch) => {
	return {

	};
};

export default withStyles(styles)(Eft);
