import React, {Component} from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import TicketTable from "views/diets/DietTable";

export class Diets extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<>
			<TicketTable />
			</>
		);
	}
}

Diets.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

Diets.propTypes = {
};

export default withStyles(styles)(Diets);
