import React from "react";
import {connect} from "react-redux";

import MaterialTable from "material-table";
import {Grid} from "@material-ui/core";
import TeamTableDetails from "./TeamTableDetails";

export const TeamTable = ({teams}) => (
			<Grid item
				xs={12}>
				<MaterialTable
					columns={[
						{title: "ID", field: "id", type: "numeric"},
						{
							title: "Name",
							field: "name",
							render: rowData => <div>{rowData.name || "[NO NAME]"}</div>,
						},
						{
							title: "Registration",
							field: "registration",
							render: rowData => <div>{rowData.registration || "[NO REGO]"}</div>,
						},
					]}
					data={teams}
					title="Teams"
					detailPanel={[
						{
							tooltip: "Show Details",
							render: (rowData) => {
								return (<TeamTableDetails team={rowData}/>);
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

export default TeamTable;
