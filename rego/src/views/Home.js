import { Grid, Paper, Hidden, Typography } from "@material-ui/core";
import { withStyles } from "@material-ui/core/styles";
import classNames from "classnames";
import Link from "components/ButtonLink";
import Norse from "components/theme/Norse";
import Theme from "components/theme/Theme";
import React, { Component } from "react";
import About from "views/About";
import TopBar from "views/TopBar";
import SocialBadges from "components/theme/SocialBadges";

const TitleBlock = ({classes, className}) => (
	<div className={className}
		id="home">
		<Norse variant="h4"
			component="h1">
				Magical Mystery Tour 2019
		</Norse>
		<Norse variant="h1"
			component="h2"
			className={classNames(classes.titlePadTopLarge)}>
				Your Journey to
		</Norse>
		<Norse variant="h1"
			component="h2"
			className={classNames(classes.titlePadBottom)}>
				ABC
		</Norse>
		<Link to="/register"
			color="primary"
			size="large"
			classes={{
				root: classes.buttonRoot,
				textPrimary: classes.buttonTextPrimary,
			}}>
				Register Now
		</Link>
	</div>
);

const Home = ({classes}) => (
	<Theme>
		<TopBar classes={classes} />
		<Paper
			className={classNames(classes.titleSection)}>
			<TitleBlock classes={classes}
				className={classNames(classes.titleBlock)} />
		</Paper>
		{/* <Paper>
					<Norse variant="h3"
						component="h2"
						style={{color:"black"}}>
						The CDS
					</Norse>
					<iframe width="100%" height="55" src="https://www.iradeo.com/station/embed/158028" frameborder="0" scrolling="no" allow="autoplay"></iframe>
					<iframe width="100%" height="55" src="https://www.iradeo.com/station/embed/158030" frameborder="0" scrolling="no" allow="autoplay"></iframe>
					<iframe width="100%" height="55" src="https://www.iradeo.com/station/embed/158032" frameborder="0" scrolling="no" allow="autoplay"></iframe>
					<iframe width="100%" height="55" src="https://www.iradeo.com/station/embed/158033" frameborder="0" scrolling="no" allow="autoplay"></iframe>
				</Paper> */}
		<Paper
			className={classNames(classes.aboutSection)}>
			<About
				className={classNames(classes.aboutBlock)} />
		</Paper>
		<Hidden smUp>
			<Grid item
				container
				direction="row"
				alignItems="center"
				justify="center"
				className={classNames(classes.socialMedias)}>
				<SocialBadges />
			</Grid>
		</Hidden>
	</Theme>
);

const padLeft = "8vw";
const padVert = "10vw";

const styles = (theme) => ({
	titlePadTopLarge: {
		paddingTop: theme.spacing(3),
	},
	root: {
		display: "flex",
		flexDirection: "row",
		height: "-webkit-fill-available",
	},
	titlePadBottom: {
		paddingBottom: theme.spacing(5),
	},
	buttonTextPrimary: {
		color: "white",
	},
	buttonRoot: {
		backgroundColor: theme.palette.primary.main,
		borderRadius: "0",
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
	},
	titleSection: {
		height: "100vh",
		[theme.breakpoints.down('xs')]: {
			height: "unset",
			paddingBottom: theme.spacing(10),
		},
		backgroundColor: "initial",
	},
	aboutSection: {
		minHeight: "100vh",
	},
	aboutBlock: {
		paddingLeft: padLeft,
		paddingRight: padLeft,
		paddingTop: padVert,
		paddingBottom: padVert,
	},
	titleBlock: {
		paddingTop: "15vw",
		[theme.breakpoints.down('xs')]: {
			paddingTop: "30vw",
		},
		paddingLeft: padLeft,
		paddingRight: padLeft,
	},
	socialMedias: {
		padding: theme.spacing(6),
		backgroundColor: "#ffffff88",
	},
});

export default withStyles(styles)(Home);
