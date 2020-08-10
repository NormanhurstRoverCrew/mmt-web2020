import React, {Component} from "react";
import PropTypes from "prop-types";
import {connect} from "react-redux";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {Paper, Typography, Grid, Button, TextField} from "@material-ui/core";
import {DatePicker} from "@material-ui/pickers";

export const VehicleEditor = ({classes, vehicle}) => {
		return (
			<Paper className={classNames(classes.paper)}>
				<Typography variant="h3">{vehicle.name}</Typography>
				<TextField
					id="name"
					label="Name"
					value={team.name || ""}
					onChange={this.onInputUpdate}
					margin="normal"
					fullWidth
					autoFocus={true}
				/>
				<TextField
					id="registration"
					label="Rego"
					value={vehicle.rego || ""}
					onChange={this.onInputUpdate}
					margin="normal"
					fullWidth
				/>
				<Button variant="contained"
					color="primary"
					onClick={this.handleSaveButton}>Save</Button>
			</Paper>
		);
};

VehicleEditor.propTypes = {
	vehicle: PropTypes.object.isRequired,
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	root: {
		paddingTop: theme.spacing(1),
	},
	paper: {
		padding: theme.spacing(2),
		margin: "0 auto",
		width: "40vw",
	},
});

export default withStyles(styles)(VehicleEditor);
