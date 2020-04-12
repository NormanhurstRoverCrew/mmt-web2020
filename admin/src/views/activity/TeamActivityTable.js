import _ from "underscore";
import React from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";

export class TeamActivityTable extends React.Component {
	constructor(props) {
		super(props);

		this.columnsForBase = this.columnsForBase.bind(this);
	}

	render() {
		let columns = [];
		columns.push({title: "ID", field: "id", type: "numeric"});
		columns.push({
			title: "Name",
			field: "name",
			render: (rowData) => <div>{rowData.name || "[NO NAME]"}</div>,
		});
		columns.push({
			title: "Registration",
			field: "registration",
			render: (rowData) => <div>{
				rowData.registration || "[NO REGO]"
			}</div>,
		});

		columns = _.union(
			columns,
			this.columnsForBase(1, "rgba(255,0,0,0.2)"),
			this.columnsForBase(10, "rgba(255,255,0,0.2)"),
			this.columnsForBase(2, "rgba(0,255,0,0.2)"),
			this.columnsForBase(3, "rgba(0,0,255,0.2)"),
		);

		return (
			<Grid item
				xs={12}>
				<MaterialTable
					columns={columns}
					data={this.props.teams}
					title="Teams Activity"
					options={{
						// selection: true
						pageSize: 100,
						pageSizeOptions: [20, 100, 300, 1000],
						headerStyle: {
							paddingLeft: 0,
							paddingRight: 0,
						},
					}}
				/>
			</Grid>
		);
	}

	columnsForBase(baseId, backgroundColor) {
		const {activities} = this.props;

		return [
			{
				title: `Base ${baseId}: Arrived`,
				render: (rowData) => {
					const base = _.where(activities, {
						team_id: rowData.id,
						base: baseId,
					});
					const arrived = _.find(base, (b) => b.arrived != null);
					if (arrived) {
						return <div>{
							new Date(arrived.logged_at).toLocaleTimeString("en-AU")
						}</div>;
					}
				},
				cellStyle: (rowData) => {
					return {backgroundColor};
				},
			},
			{
				title: "Points",
				render: (rowData) => {
					const base = _.where(activities, {
						team_id: rowData.id,
						base: baseId,
					});
					const points = _.where(base, (b) => b.points != null);
					let sum = 0.0;
					_.each(points, (point) => sum += point.points);
					return <div>{sum}</div>;
				},
				cellStyle: (rowData) => {
					return {backgroundColor};
				},
			},
			{
				title: "Trivia",
				render: (rowData) => {
					const base = _.where(activities, {
						team_id: rowData.id,
						base: baseId,
					});
					const trivias = _.where(base, (b) => b.trivia != null);
					let sum = 0.0;
					_.each(trivias, (trivia) => sum += trivia.trivia);
					return <div>{sum}</div>;
				},
				cellStyle: (rowData) => {
					return {backgroundColor};
				},
			},
			{
				title: "Base Total",
				render: (rowData) => {
					const base = _.where(activities, {
						team_id: rowData.id,
						base: baseId,
					});
					const points = _.where(base, (b) => b.points != null);
					const trivias = _.where(base, (b) => b.trivia != null);
					const emergency = _.where(base, (b) => b.clues != null);
					console.log(points, trivias, emergency, _.last(emergency))
					let sum;
					if (_.last(emergency)) {
						sum = (_.last(emergency).clues == true) ? 0 : 10.0;
					} else {
						sum = 10;
					}
					_.each(points, (point) => sum += point.points);
					_.each(trivias, (trivia) => sum += trivia.trivia);
					return <div>{sum}</div>;
				},
				cellStyle: (rowData) => {
					return {backgroundColor};
				},
			},
			{
				title: `Base ${baseId}: Departed`,
				render: (rowData) => {
					const base = _.where(activities, {team_id: rowData.id, base: baseId});
					const departed = _.find(base, (b) => b.departed != null);
					if (departed) {
						return <div>{
							new Date(departed.logged_at).toLocaleTimeString("en-AU")
						}</div>;
					}
				},
				cellStyle: (rowData) => {
					return {backgroundColor};
				},
			},
		];
	}
}

TeamActivityTable.propTypes = {
	activities: PropTypes.array,
	teams: PropTypes.array,
};

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
		teams: state.teams,
		activities: state.activities,
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
	};
};

export default connect(mapSToP, mapDispatchToProps)(TeamActivityTable);
