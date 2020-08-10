import React from "react";
import { connect } from "react-redux";

import MaterialTable from "material-table";
import { Grid, Paper, Typography } from "@material-ui/core";
import Vehicle from "./Vehicle";

export const VehicleTableDetails = ({vehicle}) => (
		<Vehicle vehicle={vehicle}/>
);

export default VehicleTableDetails;
