import React, { Component } from "react";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import { withStyles } from "@material-ui/core/styles";
import Title from "components/common/Title";
import TeamTable from "./TeamTable";

export const Teams = () => (
			<>
				<Title>Teams</Title>
				<TeamTable />
			</>
);

Teams.propTypes = {};

export default Teams;
