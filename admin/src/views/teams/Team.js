import _ from "underscore";
import React, { Component } from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import { connect } from "react-redux";
import { withStyles } from "@material-ui/core/styles";
import { Typography, Paper, Grid, Button, Modal, Card, TextField } from "@material-ui/core";
import Ticket from "views/tickets/Ticket";
import TeamEditor from "../../components/TeamEditor";
import { teamsActions } from "../../actions/teams.actions";

export class Team extends Component {
	constructor(props) {
		super(props);

		this.onEditButtonClicked = this.onEditButtonClicked.bind(this);
		this.handleTeamClosed = this.handleTeamClosed.bind(this);
		this.onAddTicketButtonClicked = this.onAddTicketButtonClicked.bind(this);
		this.onSearchUpdate = this.onSearchUpdate.bind(this)

		this.state = {
			editor_open: false,
			ticket_selector: false,
			search: "",
			search_selected: null,
		};
	}

	handleTeamClosed(e) {
		this.setState({ editor_open: false });
	}

	onEditButtonClicked() {
		this.setState({ editor_open: true });
	}

	onAddTicketButtonClicked() {
		this.setState({ ticket_selector: true });
	}

	onSearchUpdate(e) {
		const search = e.target.value.toLowerCase()
		this.setState({ search })

		if (search.length > 0) {
			const search_selected = _.first(_.filter(this.props.tickets, (ticket) => {
				return ticket.user && (ticket.id && String(ticket.id).includes(search))
					|| (ticket.user.name && ticket.user.name.toLowerCase().includes(search))
					|| (ticket.user.email && ticket.user.email.toLowerCase().includes(search))
					|| (ticket.user.mobile && ticket.user.mobile.toLowerCase().includes(search))
					|| (ticket.user.crew && ticket.user.crew.toLowerCase().includes(search))
			}), 5)

			this.setState({ search_selected })
		} else {
			this.setState({ search_selected: null })
		}
	}

	render() {
		const { classes, team, tickets } = this.props;


		const teamTickets = _.map(team.tickets, (ticketUid) => {
			return _.findWhere(tickets, { uid: ticketUid })
		})

		const renderedTickets = teamTickets.map((ticket) => {
			return (
				<Grid item
					xs={12}
					sm={6}
					md={4}
					lg={3}
					key={ticket.uid}>
					<Ticket ticket={ticket} noEmail noDelete noVerified />
				</Grid>
			);
		});

		const renderedSearchTickets = _.map(this.state.search_selected, (ticket) => {
			return (
				<Grid item
					xs={12}
					sm={6}
					md={4}
					lg={3}
					key={ticket.uid}>
					<Ticket ticket={ticket} noEmail noDelete noVerified classPaper={classNames(classes.newTicket)} onClick={()=>{this.props.addTicket(team, ticket)}}/>
				</Grid>
			)
		})

		return (
			<>
				<Paper className={classNames(classes.paper)}>
					<Grid container
						spacing={2}
						alignItems="flex-start"
						justify="space-between">
						<Grid item>
							<Typography variant="h4">{team.name || " - "}</Typography>
						</Grid>
						<Grid item>
							<Typography variant="h5">{team.registration || " - "}</Typography>
						</Grid>
					</Grid>
					<Button onClick={this.onEditButtonClicked}>Edit</Button>
					<Button onClick={this.onAddTicketButtonClicked}>Add Ticket</Button>
					<Grid container
						spacing={2}>
						{this.state.ticket_selector &&
							<Grid item>
								<Card className={classNames(classes.paper)}>
									<TextField
										label="Search Name"
										value={this.state.search || ""}
										onChange={this.onSearchUpdate}
										margin="normal"
										autoFocus={true}
									/>
								</Card>
							</Grid>
						}
						{renderedSearchTickets}
						{renderedTickets}
					</Grid>
				</Paper>
				<Modal open={this.state.editor_open}
					onClose={this.handleTeamClosed}>
					<TeamEditor team={team}
						onSave={this.handleTeamClosed} />
				</Modal>
			</>
		);
	}
}

Team.propTypes = {
	classes: PropTypes.object.isRequired,
	booking: PropTypes.object.isRequired,
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

Team.propTypes = {
};

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
		tickets: _.flatten(state.bookings.map((booking) => booking.tickets)),
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
		addTicket: (team, ticket) => dispatch(teamsActions.addTicket(team, ticket)),
	};
};

export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(Team));
