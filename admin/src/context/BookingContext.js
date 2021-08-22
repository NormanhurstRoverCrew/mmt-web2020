import React, {useState, createContext, useEffect} from 'react';
import _ from 'underscore';
import {gql} from 'apollo-boost';
import {useQuery, useMutation} from '@apollo/react-hooks';
import { onError } from "apollo-link-error";

export const BookingContext = createContext();

const GET_BOOKING = gql`
	query {
		bookings {
			id
			no
			user {
				id
				name
				email
				crew
			}
			tickets {
				id
				user {
					id
					name
					email
					mobile
					crew
					diet
					emailVerified
				}
			}
			payment {
				ticketPrice
				transactions {
					id
					value
					method
				}
			}
		}
	}
`;

const UPDATE_TICKETS = gql`
	mutation UpdateTickets($tickets: [TicketUpdate!]!) {
		updateTickets(tickets: $tickets) {
			id
		}
	}
`;

const DELETE_TICKETS = gql`
	mutation DeleteTickets($tids: [ObjectId!]!) {
		deleteTickets(ticketIds: $tids)
	}
`;

const BOOKING_DELETE = gql`
	mutation DeleteBooking($bid: ObjectId!) {
		deleteBooking(bookingId: $bid)
	}
`;

const BOOKING_ADD_TRANSACTION = gql`
	mutation AddTransaction(
		$booking_id: String!
		$transaction: TransactionInput!
	) {
		addTransaction(bookingId: $booking_id, transaction: $transaction)
	}
`;

const BookingContextProvider = ({children}) => {
	const [bookings, updateBookings] = useState([]);
	const [tickets, updateTickets] = useState([]);

	const {data: bookingsQueryData, refetch: refetchBookings} = useQuery(
		GET_BOOKING,
	);

	useEffect(() => {
		if (bookingsQueryData) {
			const b = _.chain(bookingsQueryData.bookings)
				.map(b => {
					const n = b.tickets ? b.tickets.length : 0;
					const end = n * b.payment.ticketPrice;
					const total = _.reduce(
						b.payment.transactions,
						(sum, p) => sum + p.value,
						0.0,
					);
					const remaining = end - total;
					b.payment = _.extend({}, b.payment, {
						remaining,
						total,
					});
					return b;
				})
				.value();
			updateBookings(b);
		}
	}, [bookingsQueryData]);

	const [_updateTickets, {data: dataUpdateTickets}] = useMutation(UPDATE_TICKETS);
	const updateTicket = ticket => {
		var ticket = _.pick(ticket, 'id', 'user');
		ticket = _.extend(ticket, {
			user: _.pick(
				ticket.user,
				'name',
				'email',
				'mobile',
				'crew',
				'diet',
				'emailVerified',
			),
		});

		_updateTickets({
			variables: {
				tickets: [ticket],
			},
		});
	};

	useEffect(() => {
		if (dataUpdateTickets) {
			refetchBookings();
		}
	}, [dataUpdateTickets]);

	const [_deleteTickets, {data: dataDeleteTickets}] = useMutation(
		DELETE_TICKETS,
	);
	const deleteTickets = tickets => {
		const t = _.map(tickets, ticket => ticket.id);
		_deleteTickets({variables: {tids: t}});
	};

	useEffect(() => {
		refetchBookings();
	}, [dataDeleteTickets]);

	const reloadData = () => {
		refetchBookings();
	};

	useEffect(() => {
		updateTickets(
			_.chain(bookings)
				.map(booking => booking.tickets)
				.flatten()
				.value(),
		);
	}, [bookings]);

	const [_addTransaction, {data: dataAddTransaction}] = useMutation(
		BOOKING_ADD_TRANSACTION,
	);
	const addTransaction = (bid, payment) => {
		_addTransaction({variables: {booking_id: bid, transaction: payment}});
	};
	useEffect(() => {
		refetchBookings();
	}, [dataAddTransaction]);

	const [_deleteBooking, {data: dataDeleteBooking}] = useMutation(
		BOOKING_DELETE,
	);
	const deleteBooking = booking => {
		_deleteBooking({variables: {bid: booking.id}});
	};
	useEffect(() => {
		refetchBookings();
	}, [dataDeleteBooking]);

	const obj = {
		bookings,
		tickets,
		updateTicket,
		deleteTickets,
		reloadData,
		addTransaction,
		deleteBooking,
	};

	return (
		<BookingContext.Provider value={obj}>{children}</BookingContext.Provider>
	);
};

export default BookingContextProvider;
