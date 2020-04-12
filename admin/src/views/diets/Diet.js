import React, {Component} from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {Typography, Grid, Modal, Card, CardContent, CardActions, FormControlLabel, Checkbox, Button} from "@material-ui/core";
import TicketEditor from "components/TicketEditor";

import {ticketsActions} from "actions/tickets.actions";

export class Diet extends Component {
	constructor(props) {
		super(props);
		this.handelTicketClicked = this.handelTicketClicked.bind(this);
		this.handelTicketClose = this.handelTicketClose.bind(this);
		this.emailVerifiedChanged = this.emailVerifiedChanged.bind(this);
		this.handleDelete = this.handleDelete.bind(this);

		this.state = {
			editor_open: false,
		};
	}

	handelTicketClicked(e) {
		this.setState({editor_open: true});
	}

	handelTicketClose(e) {
		this.setState({editor_open: false});
	}

	emailVerifiedChanged(e) {
		e.preventDefault();
		e.stopPropagation();
		this.props.updateTicket({
			uid: this.props.ticket.uid,
			user: {
				email_verified: e.target.checked,
			},
		});
	}

	handleDelete(e) {
		e.preventDefault();
		e.stopPropagation();

		this.props.deleteTicket(this.props.ticket);
	}

	render() {
		const {classes, ticket} = this.props;
		const {user} = ticket;

		return (
			<>
				<Card className={classNames(classes.paper)}
					onClick={this.handelTicketClicked}>
					<CardContent>
						<Typography variant="h6"
							component="h1"
							align="left">{(user && user.name) || " - "}</Typography>
						<Grid container>
							<Detail title="Crew">{(user && user.crew)}</Detail>
							<Detail title="Email">{(user && user.email) || " - "}</Detail>
							<Detail title="Mobile">{(user && user.mobile) || " - "}</Detail>
						</Grid>
					</CardContent>
					<CardActions>
						<FormControlLabel control={
							<Checkbox
								checked={user.email_verified}
								onClick={this.emailVerifiedChanged} />
						}
						label="Verified" />
						<Button onClick={this.handleDelete}>Delete</Button>
					</CardActions>
				</Card>
				<Modal open={this.state.editor_open}
					onClose={this.handelTicketClose}>
					<TicketEditor ticket={ticket}
						onSave={this.handelTicketClose} />
				</Modal>
			</>
		);
	}
}

Diet.propTypes = {
	ticket: PropTypes.object.isRequired,
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
	detailText: {
		paddingRight: theme.spacing(3),
	},
});

const Detail = withStyles(styles)(
	(props) => (
		<Grid item
			xs={6}
			sm={12}>
			<Typography variant="body1"
				noWrap
				className={classNames(props.classes.detailText)}>{props.title}: <Typography
					component="small"
					noWrap>{props.children}</Typography></Typography>
		</Grid>
	)
);

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
		updateTicket: (ticket) => dispatch(ticketsActions.update(ticket)),
		deleteTicket: (ticket) => dispatch(ticketsActions.destroy(ticket)),
	};
};

export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(Diet));
