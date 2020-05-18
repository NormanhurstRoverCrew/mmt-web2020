import React, { useState, createContext, useEffect } from "react";
import _ from "underscore";
import { gql } from "apollo-boost";
import { useQuery, useLazyQuery } from "@apollo/react-hooks";

export const BookingContext = createContext();

const TICKET_PRICE = 40.0;

const GET_BOOKING = gql`
	query GetBooking($id: String!) {
		bookingFromUser(id: $id) {
			id
			no
			tickets {
				id
				user {
					id
					name
					email
					mobile
					crew
				}
			}
		}
	}
`;

const GET_TICKET_PRICE = gql`
	{
		ticketPrice
	}
`;

const defaultTicket = {
	user: {
		name: "",
		email: "",
		crew: "",
		mobile: "",
	},
	errors: {
		name: "",
		email: "",
		crew: "",
		mobile: "",
	},
};

const rando = () =>
	Math.random().toString(36).substring(2, 15) +
	Math.random().toString(36).substring(2, 15);

const sanitizeTickets = (tickets) =>
	_.map(tickets, (ticket) => _.extend({}, defaultTicket, ticket));

const BookingContextProvider = ({ children }) => {
	const [bookingId, updateBookingId] = useState("");
	const [tickets, _updateTickets] = useState([]);
	const [checkoutMethod, updateCheckoutMethod] = useState("");
	const [bookingNo, updateBookingNo] = useState(99999999999999);
	const [ticketPrice, updateTicketPrice] = useState(TICKET_PRICE);
	const { data: getPriceData } = useQuery(GET_TICKET_PRICE);

	const updateUserId = (id) => {
		if (id) {
			localStorage.setItem("MMT2020_USER_ID", id);
		}
	};

	const [pullBooking, { data: bookingData }] = useLazyQuery(GET_BOOKING);
	useEffect(() => {
		const id = localStorage.getItem("MMT2020_USER_ID", "");
		if (tickets.length == 0 && id && id.length > 0) {
			pullBooking({
				variables: {
					id,
				},
			});
		}
	}, [tickets, bookingId]);
	useEffect(() => {
		if (bookingData) {
			updateBookingId(bookingData.bookingFromUser.id);
			updateTickets(bookingData.bookingFromUser.tickets);
			console.log(bookingData);
			updateBookingNo(bookingData.bookingFromUser.no);
		}
	}, [bookingData]);

	const price = () => {
		return tickets.length * ticketPrice;
	};

	const updateTickets = (tickets) => {
		_updateTickets(sanitizeTickets(tickets));
	};

	const addTicket = () => {
		var newTicket = _.extend({}, defaultTicket);
		newTicket.newTicket = true;
		newTicket.user = _.extend({}, defaultTicket.user);
		newTicket.errors = _.extend({}, defaultTicket.errors);
		newTicket.id = `${rando()}`;

		var t = _.union([], tickets, [newTicket]);
		updateTickets(t);
	};

	const removeTicket = (uid) => {
		var t = _.filter(tickets, (ticket) => ticket.id !== uid);
		updateTickets(t);
	};

	useEffect(() => {
		if (getPriceData) {
			updateTicketPrice(getPriceData.ticketPrice);
		}
	}, [getPriceData]);

	const obj = {
		get userId() {
			return localStorage.getItem("MMT2020_USER_ID", "");
		},
		updateUserId,
		bookingNo,
		bookingId,
		updateBookingId,
		tickets,
		updateTickets,
		addTicket,
		removeTicket,
		checkoutMethod,
		updateCheckoutMethod,
		get price() {
			return price();
		},
	};

	return (
		<BookingContext.Provider value={obj}>{children}</BookingContext.Provider>
	);
};

export default BookingContextProvider;
