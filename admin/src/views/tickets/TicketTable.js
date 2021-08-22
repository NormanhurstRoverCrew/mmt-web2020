import React, {useState, useContext} from 'react';
import {connect} from 'react-redux';
import _ from 'underscore';
import moment from 'moment';

import MaterialTable from 'material-table';
import {Grid, Modal} from '@material-ui/core';
import TicketTableDetails from 'views/tickets/TicketTableDetails';
import EmailSelector from 'components/EmailSelector';

import {BookingContext} from 'context/BookingContext';

export const TicketTable = ({}) => {
	const {tickets, deleteTickets} = useContext(BookingContext);

	const [emailSelectDialog, updateEmailSelectDialog] = useState(false);
	const [deleteConfirmationDialog, updateDeleteConfirmationDialog] = useState(false);
	const [ticketsForDeletion, updateTicketsForDeletion] = useState([]);

	const handleSendEmail = (e, rows) => {
		updateEmailSelectDialog(true);
		// tickets: rows,
	};

	const handelEmailSelectorClose = e => {
		updateEmailSelectDialog(false);
		// this.setState({emailSelectDialog: false, tickets: []});
	};

		const handleDeleteTickets = (e, rows) => {
				// Tickets that need to be deleted
				updateTicketsForDeletion(rows);
				updateDeleteConfirmationDialog(true);
		}

	const handleDeleteConfirmClosed = e => {
		updateDeleteConfirmationDialog(false);
	};

		const handleDelete = e => {
				handleDeleteConfirmClosed(e);

				deleteTickets(ticketsForDeletion);
		};

	return (
		<>
			<Grid item xs={12}>
				<MaterialTable
					columns={[
						{title: 'ID', field: 'id', type: 'numeric', defaultSort: 'desc'},
						{
							title: 'Verified',
							render: rowData => (
								<div>{rowData.user.emailVerified ? 'Yes' : 'No'}</div>
							),
						},
						{title: 'Name', field: 'user.name'},
						{title: 'Crew', field: 'user.crew'},
						{title: 'EMail', field: 'user.email'},
						{title: 'Mobile', field: 'user.mobile'},
						{title: 'Created', field: 'created_at', type: 'datetime'},
					]}
					data={tickets}
					title="Tickets"
					detailPanel={[
						{
							tooltip: 'Show Details',
							render: rowData => {
								return <TicketTableDetails ticket={rowData} />;
							},
						},
					]}
					onRowClick={(event, rowData, togglePanel) => togglePanel(0)}
					options={{
						selection: true,
						pageSize: 100,
						pageSizeOptions: [20, 50, 100, 200, 250],
					}}
					actions={[
						{
							icon: 'done_all',
							tooltip: 'Send Email',
							onClick: handleSendEmail,
						},
						{
							icon: 'delete',
							tooltip: 'Delete ticket/s',
							onClick: handleDeleteTickets,
						},
					]}
				/>
			</Grid>
			<Modal open={emailSelectDialog} onClose={handelEmailSelectorClose}>
				<EmailSelector tickets={tickets} />
			</Modal>
			<Modal open={deleteConfirmationDialog} onClose={handleDeleteConfirmClosed}>
				<>
					<h2>Are you sure you want to delete {ticketsForDeletion.length} tickets?</h2>
					<button onClick={handleDelete}>Delete</button>
				</>
			</Modal>
		</>
	);
};

export default TicketTable;
