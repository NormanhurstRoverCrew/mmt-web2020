import React, {Component} from "react";
import {Typography, Grid} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import _ from "underscore";


export class TicketBasics extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes, ticket, title} = this.props;
		return (
			<Grid container
				direction="column"
				className={classNames(classes.root)}>
				{
					title && <Grid item>
						<Typography variant="h5"
							className={classNames(classes.bold, classes.padTop, classes.padBottom)}>
							{title}
						</Typography>
					</Grid>
				}
				<Grid item
					className={classNames(classes.padBottom)}>
					<Typography className={classNames(classes.label)}>Name</Typography>
					<Typography className={classNames(classes.data)}>{ticket.user.name}</Typography>
				</Grid>
				<Grid item
					className={classNames(classes.padBottom)}>
					<Typography className={classNames(classes.label)}>Email</Typography>
					<Typography className={classNames(classes.data)}>{ticket.user.email}</Typography>
				</Grid>
				<Grid item
					className={classNames(classes.padBottom)}>
					<Typography className={classNames(classes.label)}>Mobile</Typography>
					<Typography className={classNames(classes.data)}>{ticket.user.mobile}</Typography>
				</Grid>
				<Grid item
					className={classNames(classes.padBottom)}>
					<Typography className={classNames(classes.label)}>Crew</Typography>
					<Typography className={classNames(classes.data)}>{ticket.user.crew}</Typography>
				</Grid>
			</Grid>

		);
	}
}

const styles = (theme) => ({
	root: {
		paddingBottom: "1em",
	},
	label: {
		fontSize: "0.7em",
		marginBottom: "-0.6em",
		color: theme.palette.text.secondary,
	},
	data: {
		fontSize: "1.0em",
	},
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1em",
	},
	padBottom: {
		paddingBottom: "1em",
	},
});

export default (withStyles(styles)(TicketBasics));
