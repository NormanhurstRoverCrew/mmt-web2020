import React, {Component, useContext} from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import Parade from "components/data/Parade";
import ParadeItem from "components/data/ParadeItem";
import _ from "underscore";

export const DashboardParade = ({classes}) => {
		return <></>;

		return (
			<Parade>
				<ParadeItem
					title="Bookings Today"
				>
					{bookings.today}
				</ParadeItem>
				<ParadeItem
					title="Bookings Week"
				>
					{bookings.week}
				</ParadeItem>
				<ParadeItem
					title="Bookings Month"
				>
					{bookings.month}
				</ParadeItem>
				<ParadeItem
					title="Bookings All"
				>
					{bookings.all}
				</ParadeItem>
				<ParadeItem
					title="Tickets Today"
				>
					{tickets.today}
				</ParadeItem>
				<ParadeItem
					title="Tickets Week"
				>
					{tickets.week}
				</ParadeItem>
				<ParadeItem
					title="Tickets Month"
				>
					{tickets.month}
				</ParadeItem>
				<ParadeItem
					title="Tickets All"
				>
					{tickets.all}
				</ParadeItem>
			</Parade>
		);
};

DashboardParade.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
		height: "300px",
	},
});

DashboardParade.propTypes = {
};

const convertStringToDate = (input) => {
	return _.map(input, (obj) => {
		obj.created_at = new Date(obj.created_at);
		obj.updated_at = new Date(obj.updated_at);
		return obj;
	});
};

const createdWithinDays = (input, days = 0) => {
	if (days == 0) return input;
	return _.filter(input, (obj) => obj.created_at > (new Date() - days*86400000));
};

const mapSToP = (state) => {
	const tickets = convertStringToDate(_.flatten(_.map(state.bookings, (booking) => booking.tickets)));
	const bookings = convertStringToDate(state.bookings);
	return {
		bookings: {
			today: createdWithinDays(bookings, 1).length,
			week: createdWithinDays(bookings, 7).length,
			month: createdWithinDays(bookings, 28).length,
			all: createdWithinDays(bookings).length,
		},
		tickets: {
			today: createdWithinDays(tickets, 1).length,
			week: createdWithinDays(tickets, 7).length,
			month: createdWithinDays(tickets, 28).length,
			all: createdWithinDays(tickets).length,
		},
	};
};

export default withStyles(styles)(DashboardParade);
