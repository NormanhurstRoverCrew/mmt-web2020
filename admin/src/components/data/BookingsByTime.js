import React, {Component} from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles, withTheme} from "@material-ui/core/styles";
import {XAxis, YAxis, CartesianGrid, Tooltip, BarChart, Bar, Legend, ResponsiveContainer} from "recharts";

import _ from "underscore";
import {Typography, Paper, Grid} from "@material-ui/core";

import countCreatedAt from "lib/data/countCreatedAt";

export const BookingsByTime =({classes, theme}) => {
		return <></>;

		return (
			<Grid item
				md={12}
				lg={6}
				xl={4}>
				<Typography variant="h6"
					component="h1">Bookings vs Tickets</Typography>
				<Paper className={classNames(classes.paper)}
					width="100%"
					height="100%"
				>
					<ResponsiveContainer
						aspect={16.0/9.0}
					>
						<BarChart
							data={graphData}
							margin={{right: 25}}
							height={50}
							width={50}
						>
							<CartesianGrid strokeDasharray="3 3"/>
							{/* <XAxis dataKey="bookings"/> */}
							<YAxis width={25} />
							<XAxis dataKey="name"
								height={20}
								allowDuplicatedCategory={false}
								interval="preserveStartEnd"/>
							<Tooltip/>
							{/* <Legend /> */}
							<Bar dataKey="bookings"
								fill={theme.palette.secondary.dark} />
							<Bar dataKey="tickets"
								fill="rgb(197, 114, 17)" />
						</BarChart>
					</ResponsiveContainer>
				</Paper>
			</Grid>
		);
};

BookingsByTime.propTypes = {
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
		width: "auto",
		minWidth: 50,
	},
});

const mapStateToProps = (state) => {
	const {bookings} = state;

	const cbookings = countCreatedAt(bookings, "bookings");
	const ctickets = countCreatedAt(_.flatten(bookings.map((booking) => booking.tickets)), "tickets");
	const out_bookings = _.map(cbookings, (booking) => {
		return _.extend(booking, _.find(ctickets, (ticket) => ticket.name === booking.name));
	});

	return {
		graphData: out_bookings,
	};
};

export default withTheme(withStyles(styles)(BookingsByTime));
