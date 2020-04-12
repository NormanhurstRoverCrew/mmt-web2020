import React from "react";
import {withStyles} from "@material-ui/core/styles";
import {
	Grid,
	Typography,
	Card,
	CardContent,
	CardActions,
	Button,
} from "@material-ui/core";
import PropTypes from "prop-types";

export const ParadeItem = (props) => {
	const {classes} = props;

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
		<Grid item
			xs={props.xs || 6}
			md={props.md || 4}
			lg={props.lg || 2}
			xl={props.xl || 1}>
			<Card
				className={classes.card}
				onClick={props.onClick}
			>
				<CardContent>
					<Typography className={classes.title}
						id="title"
						color="textSecondary"
						gutterBottom>
						{props.title}
					</Typography>
					<Typography
						id="data"
						variant="h5"
						component="h2"
					>
						{props.children}
					</Typography>
				</CardContent>
				{actions}
			</Card>
		</Grid>
	);
};

ParadeItem.propTypes = {
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

export default withStyles(styles)(ParadeItem);
