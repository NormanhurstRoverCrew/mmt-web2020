import React from "react";
import { connect } from "react-redux";

import MaterialTable from "material-table";
import { Grid, Paper, Typography } from "@material-ui/core";
import Team from "./Team";

export class TeamTableDetails extends React.Component {
	constructor(props) {
		super(props)
	}

	render() {
		const { team } = this.props;

		return (
			<Team team={team} />
		)
	}
}

export default TeamTableDetails;
