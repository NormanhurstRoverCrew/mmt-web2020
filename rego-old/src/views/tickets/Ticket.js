import React, {Component} from "react";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";

import Theme from "components/theme/Theme";
import {Grid, Paper, Typography} from "@material-ui/core";
import TicketFullInput from "components/tickets/TicketFullInput";
import Norse from "components/theme/Norse";


export class Ticket extends Component {
	constructor(props) {
		super(props);

		this.updateUser = this.updateUser.bind(this);
	}

	componentDidMount() {
		this.props.getTicket();
	}

	updateUser(data) {
		this.props.updateUser(data);
	}

	render() {
		const {classes, ticket, ticket: {errors}} = this.props;

		return (
			<Theme shiftLeft>
				<Grid container
					direction="row-reverse">
					<Grid item
						lg={6}
						xs={10}>
						<Paper className={classNames(classes.fullHeight, classes.root)}>
							<Norse className={classNames(classes.bold, classes.padBottomTitle)}
								variant="h3"
								black="true">Thanks for verifying your email</Norse>
							<Typography className={classNames(classes.bold, classes.padBottom)}>You will receive your ticket via email in the days leading up to MMT.</Typography>
							<TicketFullInput ticket={ticket}
								updateUser={this.updateUser}
								saveTicket={this.props.saveTicket}
								errors={errors}
							/>
						</Paper>
					</Grid>
				</Grid>
			</Theme>
		);
	}
}

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
		overflow: "auto",
	},
	fullHeight: {
		height: "100vh",
		borderRadius: "0",
		bottom: 0,
		top: 0,
		paddingBottom: "2em",
	},
	bold: {
		fontWeight: "bold",
	},
	padBottomTitle: {
		paddingBottom: "0.5em",
	},
	padBottom: {
		paddingBottom: "1em",
	},
});

export default withStyles(styles)(Ticket);
