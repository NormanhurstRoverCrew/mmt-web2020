import React, { Component } from "react";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import { withStyles } from "@material-ui/core/styles";
import Title from "components/common/Title";
import ActivityTable from "./ActivityTable";
import TeamActivityTable from "./TeamActivityTable";

export class Activitys extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<>
				<Title>Activities</Title>
				<TeamActivityTable/>
				<ActivityTable />
			</>
		);
	}
}

Activitys.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
});

Activitys.propTypes = {
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


export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(Activitys));
