import React from "react";
import { connect } from "react-redux";

import MaterialTable from "material-table";
import { Grid, Paper, Typography } from "@material-ui/core";
import Activity from "./Activity";

export class ActivityTableDetails extends React.Component {
	constructor(props) {
		super(props)
	}

	render() {
		const { activity } = this.props;

		return (
			<Activity activity={activity} />
		)
	}
}

export default ActivityTableDetails;
