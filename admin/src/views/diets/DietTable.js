import React, {useContext} from 'react';
import _ from 'underscore';

import MaterialTable from 'material-table';
import {Grid, Modal} from '@material-ui/core';
import TicketTableDetails from 'views/tickets/TicketTableDetails';
import EmailSelector from 'components/EmailSelector';

import {BookingContext} from 'context/BookingContext';

export const DietTable = ({}) => {
		const {tickets} = useContext(BookingContext);
	return (
		<>
			<Grid item xs={12}>
				<MaterialTable
					columns={[
						{title: 'ID', field: 'id', type: 'numeric'},
						{title: 'Name', field: 'user.name'},
						{title: 'Diet', field: 'user.diet'},
					]}
					data={tickets}
					title="Tickets Diet"
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
						pageSize: 20,
						pageSizeOptions: [20, 50, 100, 200, 250],
					}}
				/>
			</Grid>
		</>
	);
};

export default DietTable;
