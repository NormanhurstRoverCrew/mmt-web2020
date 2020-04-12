import {Button, Grid, Hidden} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import Link from "components/ButtonLink";
import React, {Component} from "react";

class TopBar extends Component {
	constructor(props) {
		super(props);
	}
	
	render() {
		const {classes} = this.props;

		return (
			<Grid container
				className={classNames(classes.topBar)}
				component="ul"
				spacing={3}
				alignItems="baseline">
				<Grid item
					className={classNames(classes.topBarItem, classes.topBarItemText)}
					component="li">
					<Button href="/#home"
						className={classNames(classes.topBarItemLight)}>HOME</Button>
				</Grid>
				<Grid item
					className={classNames(classes.topBarItem, classes.topBarItemText)}
					component="li">
					<Button href="/#about"
						className={classNames(classes.topBarItemLight)}>ABOUT</Button>
				</Grid>
				<Grid item
					className={classNames(classes.topBarItem, classes.topBarItemText)}
					component="li">
					<Button href="/contact"
						className={classNames(classes.topBarItemLight)}>CONTACT</Button>
				</Grid>
				<Hidden xsDown>
				<Grid item
					className={classNames(classes.topBarItem)}
					component="li">
					<Link to="/register"
						color="primary"
						size="large"
						classes={{
							root: classes.buttonRoot,
							textPrimary: classes.buttonTextPrimary,
						}}>
						Register
					</Link>
				</Grid>
				</Hidden>
			</Grid>
		);
	}
}

const padLeft = "8vw";
const padVert = "10vw";

const styles = (theme) => ({
	root: {
		display: "flex",
		flexDirection: "row",
		height: "-webkit-fill-available",
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
	},
	background: {
		backgroundImage: "linear-gradient(to left, rgba(0, 0, 0, 0.12), rgba(0, 0, 0, 0.68)), url('https://i.imgur.com/JZxrcDe.jpg')",
		backgroundRepeat: "no-repeat",
		backgroundSize: "cover",
		backgroundPosition: "bottom",
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
		fontWeight: "bold",
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
	topBar: {
		paddingTop: "2em",
		paddingRight: "3em",
		position: "absolute",
		top: 0,
		right: "40px", //fix for screen scroll?
		width: "fit-content",
	},
	topBarItem: {
		listStyle: "none",
	},
	topBarItemText: {
		// textShadow: "-1px 0 black, 0 1px black, 1px 0 black, 0 -1px black",
	},
	contactSection: {
		background: "initial",
		height: "100vh",
	},
	contactBlock: {
		paddingTop: "15vw",
	},
	topBarItemLight: {
		color: "black",
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


export default withStyles(styles)(TopBar);
