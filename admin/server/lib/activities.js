import axios from 'axios';

import _ from 'underscore';

class Activities {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async index(req, res) {
		const { data } = await axios('http://backend:3000/api/point_logs');
		const activities = data;
		res.send(activities);
	}
}

export default Activities;
