import React, {Component, useState, useContext} from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import {withStyles} from '@material-ui/core/styles';
import {Paper, Typography, Grid, Button, TextField} from '@material-ui/core';
import {DatePicker} from '@material-ui/pickers';

import {ticketsActions} from 'actions/tickets.actions.js';

import CREWS from 'components/../../crews.js';
import {BookingContext} from "context/BookingContext";

export const TicketEditor = ({classes, ticket, onSave}) => {
		const {updateTicket} = useContext(BookingContext);
	const [user, updateUser] = useState(ticket.user);

	const onInputUpdate = e => {
		const name = e.target.value;
		// this.props.onChange(e, e.target.value);
		updateUser({
			...user,
			[e.target.id]: name,
		});
	};

	const handleSaveButton = e => {
		updateTicket({
				...ticket,
				user: {
						...user,
				}
		});
		onSave(e, ticket);
	};

	return (
		<Paper className={classNames(classes.paper)}>
			<Typography variant="h3">{user.name}</Typography>
			<TextField
				id="name"
				label="Name"
				value={user.name || ''}
				onChange={onInputUpdate}
				margin="normal"
				fullWidth
				autoFocus={true}
			/>
			<TextField
				id="email"
				label="Email"
				value={user.email || ''}
				onChange={onInputUpdate}
				margin="normal"
				fullWidth
			/>
			<TextField
				id="mobile"
				label="Mobile"
				value={user.mobile || ''}
				onChange={onInputUpdate}
				margin="normal"
				fullWidth
			/>
			<TextField
				id="crew"
				select
				label="Crew"
				value={user.crew || ''}
				onChange={onInputUpdate}
				SelectProps={{
					native: true,
					MenuProps: {
						className: classes.menu,
					},
				}}
				fullWidth
				helperText="Select a Crew"
				margin="normal">
				<option key="" value="" />
				{CREWS.map(option => (
					<option key={option} value={option}>
						{option}
					</option>
				))}
			</TextField>
			<TextField
				id="diet"
				label="Diet"
				multiline
				value={user.diet || ''}
				onChange={onInputUpdate}
				margin="normal"
				fullWidth
				helperText="Can be multiline"
			/>
			<Button variant="contained" color="primary" onClick={handleSaveButton}>
				Save
			</Button>
		</Paper>
	);
};

TicketEditor.propTypes = {
	ticket: PropTypes.object.isRequired,
	classes: PropTypes.object.isRequired,
};

const styles = theme => ({
	root: {
		paddingTop: theme.spacing(1),
	},
	paper: {
		padding: theme.spacing(2),
		margin: '0 auto',
		width: '40vw',
	},
});

export default withStyles(styles)(TicketEditor);
