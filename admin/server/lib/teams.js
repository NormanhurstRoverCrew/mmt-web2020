import axios from 'axios';

import _ from 'underscore';

class Teams {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async index(req, res) {
		const { data } = await axios('http://backend:3000/api/teams?with_tickets=true');
		const teams = data;
		res.send(teams);
	}

	static async update(req, res) {
		const { id } = req.params;
		const { data } = await axios.patch(`http://backend:3000/api/teams/${id}`, req.body);
		const teams = data;
		res.send(teams);
	}

	static async addTicket(req, res) {
		const { id } = req.params;
		const { data } = await axios.patch(`http://backend:3000/api/teams/${id}/ticket`, {uid: req.body.uid});
		const teams = data;
		res.send(teams);
	}
}

export default Teams;
