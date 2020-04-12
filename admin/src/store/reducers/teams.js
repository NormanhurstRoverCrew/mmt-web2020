import _ from "underscore";

export default (state = [], action) => {
	switch (action.type) {
	case "TEAMS:UPDATE":
		return _.map(action.teams, (team) => {
			const old = _.find(state, (oldTeam) => {
				return oldTeam.uid == team.uid;
			});

			if (old) {
				return _.extend({}, old, team);
			} else {
				return _.extend({}, team);
			}
		});

	case "TEAM:UPDATE":
		return _.map(state, (team) => {
			if (team.uid == action.team.uid) {
				return _.extend({}, team, action.team);
			} else {
				return _.extend({}, team);
			}
		});

	default:
		return state;
	}
};
