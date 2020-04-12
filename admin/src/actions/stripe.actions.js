import axios from "axios";
import _ from "underscore";

const getAllCharges = async () => {
	const {data} = await axios.get("/stripe/charges");
	return data;
};

const getBookingCharges = async (bookingUID) => {
	const {data} = await axios.get(`/bookings/${bookingUID}/charges`);
	return data;
};

const stripeActions = {
	getAllCharges,
	getBookingCharges,
};

export {stripeActions};
