import React, {useContext} from "react";
import {connect} from "react-redux";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";
import VehicleTableDetails from "./VehicleTableDetails";
import {VehicleContext} from "context/VehicleContext";

export const VehicleTable = () => {
		const {vehicles} = useContext(VehicleContext);

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
							title: "Name",
							field: "name",
							render: rowData => <div>{rowData.name || "[NO NAME]"}</div>,
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
