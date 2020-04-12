import React, {Component} from "react";
import PropTypes from "prop-types";
import {connect} from "react-redux";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {Paper, Typography, Grid, Button, TextField} from "@material-ui/core";
import {InlineDatePicker} from "@material-ui/pickers";

import _ from "underscore";

import {ticketsActions} from "actions/tickets.actions.js";

export class EmailSelector extends Component {
	constructor(props) {
		super(props);

		this.onSelectorUpdate = this.onSelectorUpdate.bind(this);
		this.handleSendEmailsButton = this.handleSendEmailsButton.bind(this);

		this.state = {
			emailID: 0,
		};
	}

	onSelectorUpdate(e) {
		this.setState({emailID: e.target.value});
	}

	handleSendEmailsButton(e) {
		this.props.sendEmails(this.state.emailID, _.pluck(this.props.tickets, "uid"));
	}

	render() {
		const {classes, tickets} = this.props;

		const emails = [
			{
				id: 1,
				title: "Booking Verification Email",
				comment: "Contains a unique link that is used to verify if a user has control over an email",
			},
			{
				id: 2,
				title: "Ticket/Invite Email",
				comment: "Sends out an email containing the tickets unique QR code and an E1",
			},
		];

		return (
			<Paper className={classNames(classes.paper)}>
				<Typography
					variant="h4">
					Send Email Template
				</Typography>
				<TextField
					id="emails"
					select
					label="Email to send"
					value={this.state.emailID}
					onChange={this.onSelectorUpdate}
					SelectProps={{
						native: true,
						MenuProps: {
							className: classes.menu,
						},
					}}
					fullWidth
					helperText="Select an Email"
					margin="normal"
				>
					<option key=""
						value="" />
					{
						_.map(emails, (email) => (
							<option key={email.id}
								value={email.id}>
								{email.title}
							</option>
						))
					}
				</TextField>
				<Button className={classNames(classes.sendEmailsButton)}
					onClick={this.handleSendEmailsButton}
				>
					Are you fucking sure you want to send this email to {tickets.length} people???!!!
				</Button>
			</Paper>
		);
	}
}

EmailSelector.propTypes = {
	tickets: PropTypes.array.isRequired,
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	root: {
		paddingTop: theme.spacing(1),
	},
	paper: {
		padding: theme.spacing(2),
		margin: "0 auto",
		width: "40vw",
	},
	sendEmailsButton: {
		color: "white",
		backgroundColor: "red",
	},
});


const mapSToP = (state) => {
	return {
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
		sendEmails: (emailid, uids) => dispatch(ticketsActions.sendEmails(emailid, uids)),
	};
};

export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(EmailSelector));
