import axios from 'axios';

import _ from 'underscore';

class Bookings {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async index(req, res) {
		const { data } = await axios('http://backend:3000/api/bookings');
		const bookings = data;
		res.send(bookings);
	}

	static async delete(req, res) {
		if (_.contains(req.user.permissions, 'delete:all')) {
			const { uid } = req.params;
			const { data } = await axios.delete(`http://backend:3000/api/bookings/${uid}`);
			res.send(data);
		}
	}

	static async addPayment(req, res) {
		if (_.contains(req.user.permissions, 'payments:add')) {
			const { uid } = req.params;
			const { data } = await axios.post(`http://backend:3000/api/bookings/${uid}/payments`, req.body);
			res.send(data);
		}
	}
}

export default Bookings;
