import { Grid, Link as OutLink, Typography, Hidden } from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
import React, { Component } from "react";

import instagram from "img/instagram.svg";
import facebook from "img/facebook.svg";
import facebookw from "img/facebook-white.svg";
import email from "img/email.svg";
import emailw from "img/email-white.svg";

class Theme extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const { classes } = this.props;
		return (
			<>
				<Grid item>
					<OutLink href="https://www.facebook.com/events/2296000984006174/"
						target="_blank">
						<img src="/client/img/facebook.svg"
							className={classNames(classes.socialBadge)} />
					</OutLink>
				</Grid>
				<Grid item>
					<OutLink href="https://www.instagram.com/normorovers/"
						target="_blank">
						<img src="/client/img/instagram.svg"
							className={classNames(classes.socialBadge)} />
					</OutLink>
				</Grid>
				<Grid item>
					<OutLink href="mailto:bookings@***REMOVED***"
						target="_blank">
						<img src="/client/img/email.svg"
							className={classNames(classes.socialBadge)} />
					</OutLink>
				</Grid>
			</>
		);
	}
}

const styles = (theme) => ({
	socialBadge: {
		width: "32px",
		height: "32px",
		objectFit: "contain",
	},
});

export default withStyles(styles)(Theme);
