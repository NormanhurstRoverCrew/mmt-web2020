import React, {Component} from "react";
import PropTypes from "prop-types";
import {connect} from "react-redux";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {Paper, Typography, Grid, Button, TextField} from "@material-ui/core";
import {DatePicker} from "@material-ui/pickers";

import {teamsActions} from "actions/";

export class TeamEditor extends Component {
	constructor(props) {
		super(props);

		this.onInputUpdate = this.onInputUpdate.bind(this);
		this.handleSaveButton = this.handleSaveButton.bind(this);

		this.state = {
			team: props.team,
		};
	}

	onInputUpdate(e) {
		const name = e.target.value;
		// this.props.onChange(e, e.target.value);
		this.setState({
			team: {
				...this.state.team,
				[e.target.id]: name,
			},
		});
	}

	handleSaveButton(e) {
		this.props.updateTeam(this.state.team);
		this.props.onSave(e, this.state.team);
	}

	render() {
		const {classes} = this.props;
		const {team} = this.state;

		return (
			<Paper className={classNames(classes.paper)}>
				<Typography variant="h3">{team.name}</Typography>
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
					value={team.registration || ""}
					onChange={this.onInputUpdate}
					margin="normal"
					fullWidth
				/>
				<Button variant="contained"
					color="primary"
					onClick={this.handleSaveButton}>Save</Button>
			</Paper>
		);
	}
}

TeamEditor.propTypes = {
	team: PropTypes.object.isRequired,
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


const mapSToP = (state) => {
	return {
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
		updateTeam: (team) => dispatch(teamsActions.updateTeam(team)),
	};
};

export default connect(mapSToP, mapDispatchToProps)(withStyles(styles)(TeamEditor));
