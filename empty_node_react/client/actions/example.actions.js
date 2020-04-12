// This is an example of a redux/thunk action dispatch

import API from "../services/API/";

const get = () => {
	return (dispatch, getState) => {
		new API({access_token: getState().auth.access_token}).get("/api/bookings")
			.then((resp) => {
				dispatch(success());
				dispatch(update(resp.bookings));
			})
			.catch((err) => {
				dispatch(error());
			});

		dispatch(initGet());
	};
};

const initGet = () => {
	return {
		type: "BOOKINGS:INIT",
	};
};

const update = (bookings) => {
	return {
		type: "BOOKINGS:UPDATE",
		bookings,
	};
};

const error = () => {
	return {
		type: "BOOKINGS:FAILED",
	};
};

const success = () => {
	return {
		type: "BOOKINGS:SUCCESS",
	};
};

export const bookingsActions = {
	get,
};
