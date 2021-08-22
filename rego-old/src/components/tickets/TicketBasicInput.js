import React, { Component, useEffect } from "react";
import PropTypes from "prop-types";
import { Button, Grid, Typography } from "@material-ui/core";
import classNames from "classnames";
import { withStyles } from "@material-ui/core/styles";

import Name from "components/input/Name";
import Email from "components/input/Email";
import Crew from "components/input/Crew";
import Mobile from "components/input/Mobile";

import minusicon from "img/minus.svg";

/**
 * Basic User input. Includes Name, Mobile, Email and Crew
 */
const TicketBasicInput = ({
	tid,
	title,
	classes,
	ticket,
	removeTicket,
	updateUser: upstreamUpdateUser,
}) => {
	const { user } = ticket;

	const updateUser = (e) => {
		upstreamUpdateUser(e, tid);
	};

		if (!user) return <></>;
	return (
		<Grid container spacing={2} className={classNames(classes.root)}>
			<Grid
				container
				item
				xs={removeTicket ? 10 : 12}
				spacing={2}
				direction="column"
			>
				{title && (
					<Grid item>
						<Typography
							variant="h5"
							className={classNames(classes.bold, classes.padTop)}
						>
							{title}
						</Typography>
					</Grid>
				)}
				<Grid item>
					<Name onChange={updateUser} value={user && user.name} />
					<Error>{ticket.errors.name || ""}</Error>
				</Grid>

				<Grid item>
					<Email onChange={updateUser} value={user.email} />
					<Error>{ticket.errors.email || ""}</Error>
				</Grid>

				<Grid item>
					<Mobile onChange={updateUser} value={user.mobile} />
					<Error>{ticket.errors.mobile || ""}</Error>
				</Grid>

				<Grid item>
					<Crew onChange={updateUser} value={user.crew} />
					<Error>{ticket.errors.crew || ""}</Error>
				</Grid>
			</Grid>

			{removeTicket && (
				<Grid item xs={1}>
					<Button
						onClick={() => removeTicket(ticket.id)}
						className={classNames(classes.removeButton)}
					>
						<img src="/client/img/minus.svg" className={classes.minusIcon} />
						Remove
					</Button>
				</Grid>
			)}
		</Grid>
	);
};

const Error = (props) => {
	return <Typography style={{ color: "red" }}>{props.children}</Typography>;
};

TicketBasicInput.propTypes = {
	ticket: PropTypes.object.isRequired,
	updateUser: PropTypes.func.isRequired,
	remove: PropTypes.bool,
};

const styles = (theme) => ({
	root: {
		paddingTop: "1em",
	},
	removeButton: {
		color: theme.palette.primary.main,
		marginTop: "1.3em",
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1vw",
	},
	minusIcon: {
		width: "16px",
		height: "16px",
		objectFit: "contain",
		marginRight: "0.2em",
	},
});

export default withStyles(styles)(TicketBasicInput);
