import _ from "underscore";
import React from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";

export class ActivityTable extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {teams} = this.props;
		if (teams.length <= 0) return (<></>);
		return (
			<Grid item
				xs={12}>
				<MaterialTable
					columns={[
						{
							title: "TeamID",
							field: "team_id",
						},
						{
							title: "Team Name",
							field: "name",
							render: (rowData) => <div>{
								_.findWhere(teams, {id: rowData.team_id}).name || "[NO NAME]"
							}</div>,
						},
						{
							title: "Time",
							render: (rowData) => <div>{
								new Date(rowData.logged_at).toLocaleString("en-AU")
							}</div>,
						},
						{
							title: "Base ID",
							field: "base_id",
							render: (rowData) => <div>{rowData.base || "[NO REGO]"}</div>,
						},
						{
							title: "Message",
							render: (rowData) => <div>{message(rowData)}</div>,
						},
						{
							title: "Value",
							render: (rowData) => <div>{value(rowData)}</div>,
						},
					]}
					data={this.props.activities}
					title="Activities"
					options={{
						selection: true,
						pageSize: 200,
						pageSizeOptions: [100, 200, 500],
					}}
				/>
			</Grid>
		);
	}
}

const message = (activity) => {
	if (activity.arrived) return "ARRIVE";
	if (activity.departed) return "DEPART";
	if (activity.points) return "POINTS";
	if (activity.trivia) return "TRIVIA";
	if (activity.clues) return "CLUES OPENED";
	if (activity.comment) return "COMMENT";
	return JSON.stringify(activity);
};

const value = (activity) => {
	if (activity.points) return `${activity.points}`;
	if (activity.trivia) return `${activity.trivia}`;
	if (activity.comment) return `${activity.comment}`;
	if (activity.clues) return "-10";
	return "";
};

ActivityTable.propTypes = {
	activities: PropTypes.array,
	teams: PropTypes.array,
};

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
		activities: state.activities,
		teams: state.teams,
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
	};
};

export default connect(mapSToP, mapDispatchToProps)(ActivityTable);
