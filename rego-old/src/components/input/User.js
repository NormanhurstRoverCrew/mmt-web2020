import React, {Component} from "react";
import PropTypes from "prop-types";
import {Button, Grid, Typography} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";

import Name from "components/input/Name";
import Email from "components/input/Email";
import Crew from "components/input/Crew";
import Mobile from "components/input/Mobile";

import minusicon from "img/minus.svg";

const User = ({user, updateUser, errors, classes}) => (
	<Grid container
		item
		xs={12}
		spacing={2}
		direction="column">
		<Grid item>
			<Name
				onChange={(e) => {
					updateUser({name: e.target.value});
				}}
				value={user && user.name}
			/>
			<Error>{errors.name || ""}</Error>
		</Grid>

		<Grid item>
			<Email
				onChange={(e) => {
					updateUser({email: e.target.value});
				}}
				value={user.email}
			/>
			<Error>{errors.email || ""}</Error>
		</Grid>

		<Grid item>
			<Mobile
				onChange={(e) => {
					updateUser({mobile: e.target.value});
				}}
				value={user.mobile}
			/>
			<Error>{errors.mobile || ""}</Error>
		</Grid>

		<Grid item>
			<Crew
				onChange={(e) => {
					updateUser({crew: e.target.value});
				}}
				value={user.crew}
			/>
			<Error>{errors.crew || ""}</Error>
		</Grid>
	</Grid>
);

/**
 * Basic User input. Includes Name, Mobile, Email and Crew
 */
export class TicketBasicInput extends Component {
	constructor(props) {
		super(props);

		this.updateUser = this.updateUser.bind(this);
		this.delete = this.delete.bind(this);
	}

	/**
	 * Update a field in the user object.
	 * The ID of the field calling this method, determins the field in the user object that will be updated.
	 * @param {event} e The input field event
	 */
	updateUser(e) {
		this.props.updateUser(this.props.index || undefined, {
			[e.target.id]: e.target.value,
		});
	}

	delete(e) {
		this.props.removeTicket(this.props.index || undefined, {
			[e.target.id]: e.target.value,
		});
	}

	render() {
		const {classes, ticket, remove, index, title} = this.props;
		const errors = this.props.errors || {};
		const {user} = ticket;
		return (
			<Grid container
				spacing={2}
				className={classNames(classes.root)}>
				<Grid container
					item
					xs={remove ? 10 : 12}
					spacing={2}
					direction="column">
					{
						title && <Grid item>
							<Typography variant="h5"
								className={classNames(classes.bold, classes.padTop)}>
								{title}
							</Typography>
						</Grid>
					}
					<Grid item>
						<Name
							onChange={this.updateUser}
							value={user && user.name}
						/>
						<Error>{errors.name || ""}</Error>
					</Grid>

					<Grid item>
						<Email
							onChange={this.updateUser}
							value={user.email}
						/>
						<Error>{errors.email || ""}</Error>
					</Grid>

					<Grid item>
						<Mobile
							onChange={this.updateUser}
							value={user.mobile}
						/>
						<Error>{errors.mobile || ""}</Error>
					</Grid>

					<Grid item>
						<Crew
							onChange={this.updateUser}
							value={user.crew}
						/>
						<Error>{errors.crew || ""}</Error>
					</Grid>
				</Grid>

				{remove && index != 0 && <Grid item
					xs={1}>
					<Button
						onClick={this.delete}
						className={classNames(classes.removeButton)}
					>
						<img src="/client/img/minus.svg"
							className={classes.minusIcon} />
						Remove
					</Button>
				</Grid>}
			</Grid>

		);
	}
}

const Error = (props) => {
	return (
		<Typography style={{color: "red"}}>{props.children}</Typography>
	);
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

export default withStyles(styles)(User);
