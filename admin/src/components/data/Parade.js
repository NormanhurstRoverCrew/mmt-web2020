import React from "react";
import PropTypes from "prop-types";
import {Grid} from "@material-ui/core";

export const Parade = (props) => {
	return (
		<Grid item
			xs={props.xs || 12}
			container
			spacing={props.spacing || 1}
		>
			{props.children}
		</Grid>
	);
};

Parade.propTypes = {
	children: PropTypes.oneOfType([
		PropTypes.arrayOf(PropTypes.node),
		PropTypes.node,
	]).isRequired,
	xs: PropTypes.number,
	spacing: PropTypes.number,
};

export default (Parade);
