import axios from "axios";
import _ from "underscore";

import {bookingsActions} from "actions/bookings.actions";


const update = (ticket) => {
	return async (dispatch) => {
		await axios.patch(`/tickets/${ticket.uid}`, {
			user: ticket.user,
		});

		dispatch(bookingsActions.pull());
	};
};

const destroy = (ticket) => {
	return async (dispatch) => {
		await axios.delete(`/tickets/${ticket.uid}`);

		dispatch(bookingsActions.pull());
	};
};

const sendEmails = (emailid, uids) => {
	return async (dispatch) => {
		const {data} = await axios.post(`/email/${emailid}`, {
			ticketUIDs: uids,
		});
	};
};

const newTicket = (user) => {
	return async (dispatch) => {
		const {data} = await axios.post(`/tickets`, {
			user,
		});
		dispatch(bookingsActions.pull());		
	};
};

const ticketsActions = {
	update,
	destroy,
	sendEmails,
	newTicket
};

export {ticketsActions};
