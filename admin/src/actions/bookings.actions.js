import axios from "axios";
import _ from "underscore";

const pull = (window) => {
	return async (dispatch) => {
		const {data} = await axios.get("/bookings");
		const bookings = _.map(data, (booking) => _.extend({}, booking, {
			created_at: new Date(booking.created_at).getTime(),
			updated_at: new Date(booking.updated_at).getTime(),
		}));

		dispatch(bookingsActions.update(bookings));
	};
};


const update = (bookings) => {
	return {
		type: "BOOKINGS:UPDATE",
		bookings,
	};
};

const deleteBooking = (booking) => {
	return async (dispatch) => {
		await axios.delete(`/bookings/${booking.uid}`);

		dispatch(pull());
	};
};

const getChargesByBookings = async () => {
	const {data} = await axios.get("/bookings/charges");
	return data;
};

const getPayments = async () => {
	const {data} = await axios.get("/payments");
	return data;
};

const createPayment = (bookingUid, method, amount) => {
	return async (dispatch) => {
		const {data} = await axios.post(`/bookings/${bookingUid}/payments`, {
			method,
			amount,
		});
	};
};

const bookingsActions = {
	pull,
	update,
	deleteBooking,
	getChargesByBookings,
	getPayments,
	createPayment,
};

export {bookingsActions};
