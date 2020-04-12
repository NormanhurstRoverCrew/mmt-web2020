import axios from 'axios';

import _ from 'underscore';

class Email {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async send(req, res) {
		const { id } = req.params;
		const { ticketUIDs } = req.body;

		await _.each(ticketUIDs, async (uid) => {
			const { data } = await axios.post(`http://backend:3000/api/tickets/${uid}/email/${id}`);
		});

		res.send({});
	}
}

export default Email;
