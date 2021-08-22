import React, { Component } from "react";
import PropTypes from "prop-types";
import { Button, Grid, Typography } from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";

import Name from "components/input/Name";
import Email from "components/input/Email";
import Crew from "components/input/Crew";
import Mobile from "components/input/Mobile";
import Diet from "components/input/Diet";

/**
 * Basic User input. Includes Name, Mobile, Email and Crew
 */
export class TicketFullInput extends Component {
	constructor(props) {
		super(props);

		this.updateUser = this.updateUser.bind(this);
		this.updateTicket = this.updateTicket.bind(this);
		this.getError = this.getError.bind(this);
	}

	/**
	 * Update a field in the user object.
	 * The ID of the field calling this method, determins the field in the user object that will be updated.
	 * @param {event} e The input field event
	 */
	updateUser(e, v) {
		this.props.updateUser({
			[e.target.id]: v,
		});
	}

	updateTicket() {
		this.props.saveTicket();
	}

	getError(error) {
		if (this.props.errors) {
			return this.props.errors[error] || "";
		} else {
			return "";
		}
	}

	render() {
		const { classes, ticket, remove } = this.props;
		const { user } = ticket;

		return user ? (
			<Grid container spacing={2}>
				<Grid
					container
					item
					xs={remove ? 10 : 12}
					spacing={2}
					direction="column"
				>
					<Typography className={classNames(classes.pad)}>
						Do you have any dietry requirements?
					</Typography>
					<Grid item>
						<Diet
							onChange={this.updateUser}
							value={user.diet}
							className={classNames(classes.padBottom)}
							error={this.getError("diet")}
						/>
					</Grid>
					<Grid item>
						<Button
							onClick={this.updateTicket}
							color="primary"
							variant="contained"
							size="large"
							className={classNames(classes.padBottom)}
							classes={{
								root: classes.buttonRoot,
							}}
						>
							Submit
						</Button>
					</Grid>
				</Grid>
			</Grid>
		) : null;
	}
}

TicketFullInput.propTypes = {
	ticket: PropTypes.object.isRequired,
	updateUser: PropTypes.func.isRequired,
	errors: PropTypes.object,
};

const styles = (theme) => ({
	removeButton: {
		color: theme.palette.primary.main,
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "0.5vw",
	},
	padBottom: {
		paddingBottom: "1em",
	},
	pad: {
		padding: theme.spacing(1),
	},
	buttonRoot: {
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
		borderRadius: "0",
	},
});

export default withStyles(styles)(TicketFullInput);
