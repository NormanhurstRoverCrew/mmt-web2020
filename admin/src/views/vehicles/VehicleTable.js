import _ from "underscore";
import React, {useContext} from "react";
import {connect} from "react-redux";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";
import VehicleTableDetails from "./VehicleTableDetails";
import {VehicleContext} from "context/VehicleContext";
import {BookingContext} from "context/BookingContext";

export const VehicleTable = () => {
		const {vehicles} = useContext(VehicleContext);
		const {tickets} = useContext(BookingContext);

		return (
			<Grid item
				xs={12}>
				<MaterialTable
					columns={[
						{title: "ID", field: "id", type: "numeric"},
						{
							title: "Registration",
							field: "registration",
							render: rowData => <div>{rowData.rego || "[NO REGO]"}</div>,
						},
						{
							title: "Team Name",
							field: "name",
							render: rowData => <div>{rowData.name || "[NO NAME]"}</div>,
						},
						{
							title: "Driver Name",
							render: rowData => { 
									const ticket = _.find(tickets, (t) => t.id == rowData.driver.id );
									const driverName = (ticket && ticket.user && ticket.user.name) || "[NO NAME]";

									return <div>{driverName}</div>},
						},
					]}
					data={vehicles}
					title="Vehicles"
					detailPanel={[
						{
							tooltip: "Show Details",
							render: (rowData) => {
								return (<VehicleTableDetails vehicle={rowData}/>);
							},
						},
					]}
					onRowClick={(event, rowData, togglePanel) => togglePanel(0)}
					options={{
						// selection: true
						pageSize: 50,
						pageSizeOptions: [20, 50, 100, 200],
					}}
				/>
			</Grid>
		);
};

export default VehicleTable;
