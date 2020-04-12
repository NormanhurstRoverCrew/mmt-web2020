import React from "react";
import PropTypes from "prop-types";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import Router from "views/";


export class Content extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes} = this.props;
		this.classes = classes;

		return (
			<main className={classNames(classes.content)}>
				<Router />
			</main>
		);
	}
}

Content.propTypes = {
	classes: PropTypes.object,
};

const styles = (theme) => ({
	root: {

	},
	content: {
		flexGrow: 1,
		// padding: theme.spacing(3),
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
