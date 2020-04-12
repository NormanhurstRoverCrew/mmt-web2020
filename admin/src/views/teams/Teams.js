import React, { Component } from "react";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import { withStyles } from "@material-ui/core/styles";
import Title from "components/common/Title";
import TeamTable from "./TeamTable";

export class Teams extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const { classes } = this.props;

		return (
			<>
				<Title>Teams</Title>
				<TeamTable />
			</>
		);
	}
}

Teams.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

Teams.propTypes = {
};

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
	};
};


export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(Teams));
