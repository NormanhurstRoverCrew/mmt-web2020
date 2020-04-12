import React, {useContext} from "react";
import _ from "underscore";
import moment from "moment";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";
import BookingTableDetails from "./BookingTableDetails";
import {BookingContext} from "context/BookingContext";

export const BookingTable = ({classes}) => {
		const {bookings} = useContext(BookingContext);

		return (
			<Grid item
				xs={12}>
				<MaterialTable
					columns={[
						{title: "ID", field: "id", type: "numeric", defaultSort: "desc"},
						{title: "Name", field: "user.name"},
						{title: "Quantity/Tickets", field: "tickets.length", type: "numeric"},
						{title: "Created", field: "created_at", type: "datetime"},
					]}
					data={bookings}
					title="Bookings"
					detailPanel={[
						{
							tooltip: "Show Details",
							render: (rowData) => {
								return (<BookingTableDetails booking={rowData}/>);
							},
						},
					]}
					onRowClick={(event, rowData, togglePanel) => togglePanel(0)}
					options={{
						// selection: true
						pageSize: 100,
						pageSizeOptions: [50, 100, 200],
					}}
				/>
			</Grid>
		);
};

export default BookingTable;
