import axios from 'axios';

import _ from 'underscore';

class Payments {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async index(req, res) {
		if (_.contains(req.user.permissions, 'payments:view')) {
			const { data } = await axios('http://backend:3000/api/payments');
			res.send(data);
		}
	}
}

export default Payments;
