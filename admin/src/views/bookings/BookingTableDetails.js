import React, {useContext} from 'react';

import MaterialTable from 'material-table';
import {Grid, Paper, Typography} from '@material-ui/core';
import Booking from './Booking';
import {BookingContext} from 'context/BookingContext';

export const BookingTableDetails = ({booking}) => {
	return <Booking booking={booking} />;
};

export default BookingTableDetails;
