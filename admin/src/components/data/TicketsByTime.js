import React, {Component} from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles, withTheme} from "@material-ui/core/styles";
import {XAxis, YAxis, CartesianGrid, Tooltip, BarChart, Bar, Legend, ResponsiveContainer} from "recharts";

import _ from "underscore";
import {Typography, Paper, Grid} from "@material-ui/core";

import countCreatedAt from "lib/data/countCreatedAt";

export class TicketsByTime extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		console.log(this.props.ticketsGraph);
		const {classes, theme} = this.props;
		return (
			<Grid item
				md={12}
				lg={6}
				xl={4}>
				<Typography variant="h6"
					component="h1"
				>
					Tickets
				</Typography>
				<Paper className={classNames(classes.paper)}
					width="100%"
					height="100%"
				>
					<ResponsiveContainer
						aspect={16.0/9.0}
					>
						<BarChart
							data={this.props.ticketsGraph}
							margin={{right: 25}}
							height={50}
							width={50}
						>
							<CartesianGrid strokeDasharray="3 3" />
							{/* <XAxis dataKey="bookings"/> */}
							<YAxis width={25} />
							<XAxis dataKey="name"
								height={20}
								allowDuplicatedCategory={false}
								interval="preserveStartEnd" />
							<Tooltip />
							{/* <Legend /> */}
							<Bar dataKey="tickets"
								fill={theme.palette.secondary.dark} />
						</BarChart>
					</ResponsiveContainer>
				</Paper>
			</Grid>
		);
	}
}

TicketsByTime.propTypes = {
};

TicketsByTime.propTypes = {
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
		width: "auto",
	},
});

export const mapStateToProps = (state) => {
	const {bookings} = state;

	return {
		ticketsGraph: countCreatedAt(_.flatten(bookings.map((booking) => booking.tickets)), "tickets"),
	};
};

export const styled = withTheme(withStyles(styles)(TicketsByTime));
export default connect(mapStateToProps)(styled);
