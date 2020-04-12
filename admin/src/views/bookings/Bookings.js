import React, { Component } from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";
import Title from "components/common/Title";
import BookingTable from "views/bookings/BookingTable";
import { Table, TableHead, TableCell, Tab, TableRow, TableBody } from "@material-ui/core";
import { bookingsActions } from "../../actions/bookings.actions";

export const Bookings = () => {
		return (
			<>
				<Title>Bookings</Title>
				<BookingTable />
			</>
		);
};

Bookings.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

export default withStyles(styles)(Bookings);
