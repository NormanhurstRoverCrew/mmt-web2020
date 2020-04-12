import React from "react";
import PropTypes from "prop-types";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import {Grid} from "@material-ui/core";
import Router from "views/";

export const Content = ({classes}) => {
		return (
			<main className={classNames(classes.content)}>
				<div className={classNames(classes.appBarSpacer)} />
				<Grid container
					spacing={2}>
					<Router />
				</Grid>
			</main>
		);
};

Content.propTypes = {
	classes: PropTypes.object,
};

const styles = (theme) => ({
	root: {

	},
	appBarSpacer: theme.mixins.toolbar,
	content: {
		flexGrow: 1,
		padding: theme.spacing(3),
		height: "100vh",
		overflow: "auto",
	},
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

export default withStyles(styles)(Content);
