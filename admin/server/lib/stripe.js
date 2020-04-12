import axios from 'axios';

import _ from 'underscore';

class Stripe {
	// constructor() {
	// this.index = this.index.bind(this);
	// }

	static async index(req, res) {
		const { data } = await axios('http://backend:3000/api/stripe/payment_intents');

		const payments = _.map(data.data,
			(d) => {
				const a = _.pick(d,
					[
						'id',
						'amount',
						'created',
						'status',
						'charges',
					]);
				a.charges = _.map(a.charges.data);
				a.amount /= 100.0;
				return a;
			});

		res.send(payments);
	}

	static async charges(req, res) {
		const { data } = await axios('http://backend:3000/api/stripe/charges');

		res.send(data);
	}

	static async bookingCharges(req, res) {
		const { data } = await axios(`http://backend:3000/api/stripe/bookings/${req.params.uid}/charges`);

		const charge = data;
		charge.amount /= 100;
		charge.amount_refunded /= 100.0;
		res.send(charge);
	}
}

export default Stripe;
