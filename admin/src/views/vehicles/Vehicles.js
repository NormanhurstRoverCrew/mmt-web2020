import React, { Component } from "react";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import { withStyles } from "@material-ui/core/styles";
import Title from "components/common/Title";
import VehicleTable from "./VehicleTable";

export const Vehicles = () => (
			<>
				<Title>Vehicles</Title>
				<VehicleTable />
			</>
);

Vehicles.propTypes = {};

export default Vehicles;
