import React, {useState, useContext} from 'react';
import _ from 'underscore';
import moment from 'moment';

import MaterialTable from 'material-table';
import {Grid, Modal} from '@material-ui/core';
import PaymentTableDetails from 'views/payments/PaymentTableDetails';

import {bookingsActions} from 'actions/bookings.actions';
import AddPayment from '../../components/AddPayment';

import {BookingContext} from 'context/BookingContext';

export const BookingPaymentTable = ({}) => {
	const {bookings} = useContext(BookingContext);
	const [modal, updateModal] = useState(false);
	const [booking_uid, updateUid] = useState(false);

	const handleModalClose = () => updateModal(false);

	const addPayment = booking_uid => {
		updateUid(booking_uid);
		updateModal(true);
		// setTimeout(1000, () => { this.getCharges(); });
	};

	return (
		<>
			<Grid item xs={12}>
				<MaterialTable
					columns={[
						{
							title: 'Booking ID',
							field: 'idn',
							type: 'numeric',
							defaultSort: 'desc',
						},
						{title: 'Name', field: 'user.name'},
						{title: 'Crew', field: 'user.crew'},
						{
							title: 'Proposed Payment Method',
							field: 'payment.proposed',
							lookup: {eft: 'Electronic Funds Transfer', cash: 'Cash', '': '-'},
						},
						{title: 'Total Paid', field: 'payment.total', type: 'currency'},
						{
							title: 'Total Remaining',
							field: 'payment_remaining',
							type: 'currency',
							render: rowData => {
								return (
									<div
										style={{
											color: rowData.payment.remaining <= 0 ? 'green' : 'red',
										}}>
										{rowData.payment.remaining.toLocaleString('en-AU', {
											style: 'currency',
											currency: 'AUD',
										})}
									</div>
								);
							},
						},
						{title: 'Created', field: 'created_at', type: 'datetime'},
					]}
					data={bookings}
					title="Booking Payments"
					detailPanel={[
						{
							tooltip: 'Show Details',
							render: rowData => {
								return <PaymentTableDetails booking={rowData} />;
							},
						},
					]}
					onRowClick={(event, rowData, togglePanel) => togglePanel(0)}
					options={{
						pageSize: 100,
						pageSizeOptions: [20, 50, 100, 200, 250],
					}}
					actions={[
						{
							icon: 'add',
							tooltip: 'Add Payment',
							onClick: (event, rowData) => addPayment(rowData.id),
						},
					]}
				/>
			</Grid>
			<Modal open={modal} onClose={handleModalClose}>
				<AddPayment
					booking={_.find(bookings, b => b.id == booking_uid)}
					// defaultAmount={/*_.find(payments, p => p.uid == booking_uid)*/}
					close={() => handleModalClose()}
				/>
			</Modal>
		</>
	);
};

export default BookingPaymentTable;
