import React, {Component} from "react";
import {Typography} from "@material-ui/core";
import classNames from "classnames";
import {withStyles} from "@material-ui/core";
import PropTypes from "prop-types";

class Norse extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes, children, className, black} = this.props;
		return (
			<Typography
				{...this.props}
				className={classNames(classes.font, className, black && classes.black)}
				classes={null}>
				{children}
			</Typography>
		);
	}
}

Norse.propTypes = {
	black: PropTypes.any,
	classes: PropTypes.object,
	className: PropTypes.string,
};


const styles = (theme) => ({
	font: {
		fontFamily: [
			"Norse", "Roboto", "Helvetica", "Arial", "sans-serif",
		].join(","),
		color: "#fff",
		[theme.breakpoints.down('xs')]: {
			fontSize: theme.typography.fontSize * 3,
		},
	},
	black: {
		color: theme.palette.text.primary,
	},
});

export default withStyles(styles)(Norse);
