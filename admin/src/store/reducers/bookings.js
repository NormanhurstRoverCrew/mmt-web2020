import _ from "underscore";

export default (state = [], action) => {
	const newState = [];
	switch (action.type) {
	case "BOOKINGS:ADD":
		return [
			...state,
			action.booking,
		];

	case "BOOKINGS:UPDATE":
		return _.map(action.bookings, (booking) => {
			const old = _.find(state, (oldBooking) => {
				return oldBooking.uid == booking.uid;
			});

			if (old) {
				return _.extend({}, old, booking);
			} else {
				return _.extend({}, booking);
			}
		});


	default:
		return state;
	}
};
