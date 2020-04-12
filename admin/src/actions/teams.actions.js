import axios from "axios";
import _ from "underscore";

const pull = () => {
	return async (dispatch) => {
		const {data} = await axios.get("/teams");
		const teams = _.map(data, (team) => _.extend({}, team, {
			created_at: new Date(team.created_at).getTime(),
			updated_at: new Date(team.updated_at).getTime(),
		}));

		dispatch(update(teams));
	};
};


const update = (teams) => {
	return {
		type: "TEAMS:UPDATE",
		teams,
	};
};

const pushUpdateTeam = (team) => {
	return {
		type: "TEAM:UPDATE",
		team,
	};
};

const updateTeam = (team) => {
	return async (dispatch) => {
		dispatch(pushUpdateTeam(team));

		const {data} = await axios.patch(`/teams/${team.id}`, team);

	}
}

const addTicket = (team, ticket) => {
	return async (dispatch) => {
		const {data} = await axios.post(`/teams/${team.id}/ticket`, {uid: ticket.uid});
		dispatch(pull())
	}
}

const teamsActions = {
	pull,
	update,
	updateTeam,
	addTicket,
};

export {teamsActions};
