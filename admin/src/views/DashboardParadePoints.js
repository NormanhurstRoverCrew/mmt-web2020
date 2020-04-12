import React, {Component} from "react";
import PropTypes from "prop-types";
import classNames from "classnames";
import {connect} from "react-redux";
import {withStyles} from "@material-ui/core/styles";
import Parade from "components/data/Parade";
import ParadeItem from "components/data/ParadeItem";
import ParadeItemElement from "components/data/ParadeItemElement";
import _ from "underscore";

export const DashboardParadePoints = ({classes}) => {
		return <></>;

		return (
			<Parade>
				<ParadeItem
					title="Base 1"
					xl={2}
				>
					<ParadeItemElement
						title="In"
					>
						{_.size(arrived[1])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Out"
					>
						{_.size(departed[1])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Current"
					>
						{_.size(arrived[1]) - _.size(departed[1])}
					</ParadeItemElement>
				</ParadeItem>
				<ParadeItem
					title="Base 2"
					xl={2}
				>
					<ParadeItemElement
						title="In"
					>
						{_.size(arrived[2])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Out"
					>
						{_.size(departed[2])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Current"
					>
						{_.size(arrived[2]) - _.size(departed[2])}
					</ParadeItemElement>
				</ParadeItem>
				<ParadeItem
					title="Base 3"
					xl={2}
				>
					<ParadeItemElement
						title="In"
					>
						{_.size(arrived[3])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Out"
					>
						{_.size(departed[3])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Current"
					>
						{_.size(arrived[3]) - _.size(departed[3])}
					</ParadeItemElement>
				</ParadeItem>
				<ParadeItem
					title="Loki Base"
					xl={2}
				>
					<ParadeItemElement
						title="In"
					>
						{_.size(arrived[11])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Out"
					>
						{_.size(departed[11])}
					</ParadeItemElement>
					<ParadeItemElement
						title="Current"
					>
						{_.size(arrived[11]) - _.size(departed[4])}
					</ParadeItemElement>
				</ParadeItem>
				<ParadeItem
					title="Endpoint"
					xl={2}
				>
					<ParadeItemElement
						title="Arrived"
					>
						{_.size(arrived[4])}
					</ParadeItemElement>
				</ParadeItem>
			</Parade>
		);
};

DashboardParadePoints.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
		height: "300px",
	},
});

DashboardParadePoints.propTypes = {
};

const N_BASES = 11;

const mapSToP = (state) => {
	console.log(state);
	const arrived = {};
	const departed = {};
	for (var n = 1; n <= N_BASES; n++) {
		arrived[n] = {};
		departed[n] = {};
	}

	_.each(state.activities, (a) => {
		for (var n = 1; n <= N_BASES; n++) {
			if (a.arrived && a.base == n) {
				arrived[n][a.team_id] = true;
			}
			if (a.departed && a.base == n) {
				departed[n][a.team_id] = true;
			}
		}
	});

	return {
		arrived,
		departed,
	};
};

export default withStyles(styles)(DashboardParadePoints);
