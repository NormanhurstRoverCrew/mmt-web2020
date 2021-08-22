import _ from "underscore";
import React, { useState, useContext } from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import { connect } from "react-redux";
import { withStyles } from "@material-ui/core/styles";
import { Typography, Paper, Grid, Button, Modal, Card, TextField } from "@material-ui/core";
import Ticket from "views/tickets/Ticket";
import VehicleEditor from "../../components/VehicleEditor";
import { teamsActions } from "../../actions/teams.actions";

import {BookingContext} from 'context/BookingContext';
import {VehicleContext} from 'context/VehicleContext';

export const Vehicle = ({classes, vehicle}) => {
	var {tickets} = useContext(BookingContext);
	var {approveTicket, denyTicket} = useContext(VehicleContext);
		const [editorOpen, setEditorOpen] = useState(false);

		const handleVehicleClosed = () => {
				console.log("Handle Vehicle Closed");
		};
		
		console.log(vehicle, tickets);

		const renderTickets = (t, renderer) => (
						_.chain(t)
						.map((tick) => (_.findWhere(tickets, {id: tick.id})))
						.map(renderer)
						.value()
				);

		const rTickets = renderTickets(vehicle.tickets, (ticket) => (
						<Grid item xs={6} lg={4} md={3} key={ticket.id}>
								<Ticket ticket={ticket} />
						</Grid>
		));
		const rRequests = renderTickets(vehicle.requests, (ticket) => (
						<Grid item xs={6} lg={4} md={3} key={ticket.id}>
								<Ticket ticket={ticket} />
								<Button onClick={() => approveTicket(vehicle.id, ticket.id)}>Approve</Button>
								<Button onClick={() => denyTicket(vehicle.id, ticket.id)}>Deny</Button>
						</Grid>
		));

		return (
			<>
				<Paper className={classNames(classes.paper)}>
					<Grid container
						spacing={2}
						alignItems="flex-start"
						justify="space-between">
					</Grid>
						<Grid container spacing={2}>
								{rRequests}
								{rTickets}
						</Grid>
				</Paper>
				<Modal open={editorOpen}
					onClose={handleVehicleClosed}>
					<VehicleEditor vehicle={vehicle}
						onSave={handleVehicleClosed} />
				</Modal>
			</>
		);
};

Vehicle.propTypes = {
	classes: PropTypes.object.isRequired,
	vehicle: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		margin: theme.spacing(1),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
	deleteButton: {
		color: "red",
	},
	newTicket: {
		backgroundColor: "#ffc80038"
	}
});

export default withStyles(styles)(Vehicle);
