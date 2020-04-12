import { Grid, Link as OutLink, Typography, Hidden } from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
import React, { Component } from "react";

import instagram from "img/instagram.svg";
import facebook from "img/facebook.svg";
import facebookw from "img/facebook-white.svg";
import email from "img/email.svg";
import emailw from "img/email-white.svg";
import SocialBadges from "components/theme/SocialBadges";

class Theme extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const { classes, shiftLeft, noBG } = this.props;
		return (
			<div className={classNames(classes.root)}>
				<Hidden xsDown>
					<Grid container
						direction="column"
						justify="space-between"
						className={classNames(classes.sidebar)}>
						<Grid item>
							<Typography
								xs={1}
								className={classNames(classes.sideTitle)}>
								MMT 2019
						</Typography>
						</Grid>
						<Grid item
							container
							direction="column"
							alignItems="center"
							className={classNames(classes.socialMedias)}>
							<SocialBadges/>
						</Grid>
						<Grid item>
							<Typography className={classNames(classes.sideTitle)}>

							</Typography>
						</Grid>
					</Grid>
				</Hidden>
				<div className={classNames(classes.background, classes.main, noBG && classes.noBg, shiftLeft && classes.backgroundShiftLeft)}>
					{this.props.children}
				</div>
			</div >
		);
	}
}

const padLeft = "8vw";
const padVert = "10vw";

const styles = (theme) => ({
	socialBadge: {
		width: "32px",
		height: "32px",
		objectFit: "contain",
	},
	root: {
		display: "flex",
		flexDirection: "horizontal",
		height: "-webkit-fill-available",
	},
	main: {
		width: "-webkit-fill-available",
		height: "-webkit-fill-available",
		overflowY: "scroll",
	},
	titlePadTopLarge: {
		paddingTop: theme.spacing(3),
	},
	titlePadBottom: {
		paddingBottom: theme.spacing(5),
	},
	titleMMT: {
		paddingLeft: "0.2em",
	},
	sidebar: {
		width: "7em",
		[theme.breakpoints.down('xs')]: {
			width: "4em",
		},
	},
	background: {
		backgroundImage: "linear-gradient(to left, rgba(0, 0, 0, 0.12), rgba(0, 0, 0, 0.68)), url('https://i.imgur.com/JZxrcDe.jpg')",
		backgroundRepeat: "no-repeat",
		backgroundSize: "cover",
		backgroundPosition: "bottom",
	},
	noBg: {
		background: "none",
		backgroundColor: "white",
	},
	backgroundShiftLeft: {
		backgroundPosition: "right",
	},
	titleSection: {
		height: "100vh",
		backgroundColor: "initial",
	},
	aboutBlock: {
		paddingLeft: padLeft,
		paddingRight: padLeft,
		paddingTop: padVert,
		paddingBottom: padVert,
	},
	sideTitle: {
		transform: "rotate(-90deg)",
		padding: "6vw 0",
		fontSize: "1.2em",
		fontWeight: "600",
	},
	button: {
		color: "white",
	},
	socialMedias: {
		height: "fit-content",
	},
	titleBlock: {
		paddingTop: "15vw",
		paddingLeft: padLeft,
	},
	aboutIcons: {
		borderRadius: "100%",
		border: "solid #979797 1px",
		backgroundColor: "#d8d8d8",
		width: "3em",
		height: "3em",
	},
	aboutItemTitle: {
		color: "rgb(217, 131, 10)",
	},
	aboutHeaders: {
		color: "black",
		fontWeight: "bold",
	},
	buttonTextPrimary: {
		color: "white",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	topBarItem: {
		listStyle: "none",
	},
	contactSection: {
		background: "initial",
		height: "100vh",
	},
	contactBlock: {
		paddingTop: "15vw",
	},
	topBarItemLight: {
		color: "white",
	},
	contactItemImg: {
		width: "12em",
		height: "12em",
		border: "solid black 2px",
		borderRadius: "50%",
	},
	contactItemText: {
		color: "white",
	},
	contactItem: {
		width: "18em",
	},
});

export default withStyles(styles)(Theme);
