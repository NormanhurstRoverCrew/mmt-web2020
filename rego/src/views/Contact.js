import {Grid, Link as OutLink, Paper, Typography} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import Theme from "components/theme/Theme";
import React, {Component} from "react";
import TopBar from "views/TopBar";

class Contact extends Component {
	constructor(props) {
		super(props);
	}
	render() {
		const {classes} = this.props;
		return (
			<Theme>
				<TopBar />
				<Paper
					className={classNames(classes.contactSection, classes.contactBlock)}>
					<Grid container
						spacing={5}
						justify="center">
						<ContactItem
							name="Alyssa-Maree O'Brien"
							img="https://i.imgur.com/fIWfDEL.jpg"
							comment="MMT Co-ordinator"
							messenger="https://www.facebook.com/messages/t/alyssamaree.obrien" />
						<ContactItem
							name="Sophie Carter"
							img="https://i.imgur.com/YGM9xnb.jpg"
							comment="Catering, Dietary"
							email="dietry@***REMOVED***"
							messenger="https://www.facebook.com/messages/t/Parrafan2" />
						<ContactItem
							name="Grant Perry"
							img="https://i.imgur.com/5CCumg6.jpg"
							comment="Admin, Web Dev"
							email="bookings@***REMOVED***"
							messenger="https://www.facebook.com/messages/t/grantipoos" />
					</Grid>
				</Paper>
			</Theme>
		);
	}
}

class ContactItemC extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes, name, img, comment, messenger, email} = this.props;
		return (
			<Grid item
				lg={3}
				xs={12}
				className={classNames(classes.contactItem)}>
				<Grid container
					direction="column"
					alignItems="center">
					<Grid item>
						<img
							className={classNames(classes.contactItemImg)}
							src={img} />
					</Grid>
					<Grid item>
						<Grid container
							alignItems="center">
							{messenger && <Grid item>
								<OutLink href={messenger}
									target="_blank">
									<img src="/client/img/facebook-white.svg"
										className={classNames(classes.socialBadge)} />
								</OutLink>
							</Grid>}
							{email && <Grid item>
								<OutLink href={"mailto:" + email}
									target="_blank">
									<img src="/client/img/email-white.svg"
										className={classNames(classes.socialBadge)} />
								</OutLink>
							</Grid>}
						</Grid>
					</Grid>
					<Grid item>
						<Typography
							variant="h4"
							align="center">{name}</Typography>
					</Grid>
					<Grid item>
						<Typography className={classNames(classes.contactItemText)}>{comment}</Typography>
					</Grid>
				</Grid>
			</Grid>
		);
	}
}

const styles = (theme) => ({
	socialBadge: {
		width: "32px",
		height: "32px",
		objectFit: "contain",

	},
	contactSection: {
		background: "initial",
		height: "100vh",
	},
	contactBlock: {
		padding: "15vw 5vw",
	},
	topBarItemLight: {
		color: "white",
	},
	contactItemImg: {
		width: "12em",
		height: "12em",
		border: "solid white 2px",
		borderRadius: "50%",
	},
	contactItemText: {
		color: "white",
	},
	contactItem: {
	},
});

const ContactItem = withStyles(styles)(ContactItemC);

export default withStyles(styles)(Contact);
