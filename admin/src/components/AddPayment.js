import React, {useState, useContext} from 'react';
import _ from 'underscore';
import PropTypes from 'prop-types';
import {connect} from 'react-redux';
import classNames from 'classnames';
import {withStyles} from '@material-ui/core/styles';
import {
	Paper,
	Typography,
	Grid,
	Button,
	TextField,
	InputAdornment,
	Input,
	Select,
	MenuItem,
} from '@material-ui/core';

import {BookingContext} from 'context/BookingContext';

export const AddPayment = ({classes, booking, close}) => {
	const {user, payment: p} = booking;

	const {addTransaction} = useContext(BookingContext);

	const [payment, updatePayment] = useState({
		value: p.remaining,
		method: 'EFT',
	});

	const onInputUpdate = e => {
		const val = e.target.value;
		const id = e.target.id || e.target.name;
		if (id == 'value') {
			updatePayment(_.extend({}, payment, {value: parseFloat(val)}));
		} else {
			updatePayment(_.extend({}, payment, {[id]: val}));
		}
	};

	const handleCreateButton = e => {
		addTransaction(booking.id, payment);
		close();
	};

	return (
		<Paper className={classNames(classes.paper)}>
			<Typography variant="h3">Add Payment</Typography>
			<Typography variant="h4">{user.name}</Typography>
			<Typography variant="h5">Booking ID: {booking.idn}</Typography>
			<Input
				id="value"
				label="Value"
				type="number"
				value={payment.value || ''}
				onChange={onInputUpdate}
				fullWidth
				autoFocus={true}
				startAdornment={<InputAdornment position="start">$</InputAdornment>}
				className={classNames(classes.padItems)}
			/>
			<Select
				value={payment.method}
				onChange={onInputUpdate}
				inputProps={{
					id: 'method',
					name: 'method',
				}}
				fullWidth
				className={classNames(classes.padItems)}>
				<MenuItem value="">
					<em>None</em>
				</MenuItem>
				<MenuItem value="STRIPE">Stripe</MenuItem>
				<MenuItem value="PAYPAL">Paypal</MenuItem>
				<MenuItem value="EFT">Electronic Funds Transfer</MenuItem>
				<MenuItem value="CASH">Cash</MenuItem>
			</Select>
			<Typography className={classNames(classes.padItems)}>
				Note: Creating a payment will automatically send a receipt.
			</Typography>
			<Typography className={classNames(classes.padItems)}>
				Note: If this booking has multiple tickets, an invitation will be sent
				to each team member to fill out their details
			</Typography>
			<Button onClick={handleCreateButton}>Create</Button>
		</Paper>
	);
};

AddPayment.propTypes = {
	booking: PropTypes.object.isRequired,
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
	padItems: {
		paddingTop: theme.spacing(2),
	},
});

export default withStyles(styles)(AddPayment);
