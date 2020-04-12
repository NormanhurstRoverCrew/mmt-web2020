import React from "react";
import { withStyles } from "@material-ui/core/styles";
import {
	Grid,
	Typography,
	Card,
	CardContent,
	CardActions,
	Button,
} from "@material-ui/core";
import PropTypes from "prop-types";

export const ParadeItemElement = (props) => {
	const { classes } = props;

	let actions;
	if (props.actionText && props.actionClick) {
		actions = (
			<CardActions
				id="action-button"
			>
				<Button onClick={
					(e) => {
						// Prevent the click event from triggering the Card click event
						e.stopPropagation();
						props.actionClick(e);
					}
				}
					size="small">
					{props.actionText}
				</Button>
			</CardActions>
		);
	}

	return (
		<div>
			<Typography className={classes.title}
				id="title"
				color="textSecondary"
				gutterBottom>
				{props.title}: {props.children}
			</Typography>
			{actions}
		</div>
	);
};

ParadeItemElement.propTypes = {
	classes: PropTypes.object.isRequired,
	xs: PropTypes.number,
	title: PropTypes.string.isRequired,
	children: PropTypes.oneOfType([
		PropTypes.arrayOf(PropTypes.node),
		PropTypes.node,
	]).isRequired,
	actionText: PropTypes.string,
	actionClick: PropTypes.func,
	onClick: PropTypes.func,
};

const styles = (theme) => ({
	title: {

	},
	card: {

	},
});

export default withStyles(styles)(ParadeItemElement);
