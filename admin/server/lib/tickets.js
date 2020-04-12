import axios from 'axios';

import _ from 'underscore';

class Tickets {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async update(req, res) {
		if (_.contains(req.user.permissions, 'users:edit')) {
			const { uid } = req.params;
			const { data } = await axios.patch(`http://backend:3000/api/tickets/${uid}`, req.body);
			const bookings = data;
			res.send(bookings);
		}
	}

	static async delete(req, res) {
		if (_.contains(req.user.permissions, 'delete:all')) {
			const { uid } = req.params;
			const { data } = await axios.delete(`http://backend:3000/api/tickets/${uid}`);
			res.send(data);
		}
	}

	static async newTicket(req, res) {
		if (_.contains(req.user.permissions, 'users:edit')) {
			const { user } = req.body;
	
			const { data: booking_data } = await axios.post('http://backend:3000/api/bookings', {
				quick_add: true,
				user,
			});

			if (booking_data.errors) {
				res.send(booking_data);
				return;
			}
			
			const { data: pay_data } = await axios.post(`http://backend:3000/api/bookings/${booking_data.uid}/payments`, {
				method: "cash",
				amount: 40.00,
				send_receipt: false,
				quick_add: true,
			});
			
			res.send(booking_data);
		}
	}
}

export default Tickets;
