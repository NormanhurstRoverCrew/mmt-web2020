import React, { useState, createContext } from "react";

export const BookingContext = createContext();

const BookingContextProvider = ({ children }) => {
	const [bookingId, _updateBookingId] = useState(localStorage.getItem("MMT2020_USER_ID", ""));

	const updateBookingId = (id) => {
		localStorage.setItem("MMT2020_USER_ID", id);
		_updateBookingId(id);
	};

	return (
		<BookingContext.Provider
			value={{
				bookingId,
				updateBookingId,
			}}
		>
			{children}
		</BookingContext.Provider>
	);
};

export default BookingContextProvider;
