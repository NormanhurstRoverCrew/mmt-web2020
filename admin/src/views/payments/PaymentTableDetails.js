import React from 'react';
import {connect} from 'react-redux';
import {withStyles} from '@material-ui/core/styles';
import classNames from 'classnames';

import _ from 'underscore';
import {Typography, Grid, Paper} from '@material-ui/core';

const fmtCurrency = new Intl.NumberFormat('en-AU', {
	style: 'currency',
	currency: 'AUD',
});

export const PaymentTableDetails = ({classes, booking}) => {
	return (
		<Grid container spacing={4} className={classNames(classes.root)}>
			{_.map(booking.payment.transactions, (tx, i) => {
				return (
					<Grid item key={i} xs={4}>
						<Paper className={classNames(classes.paper)}>
							<Typography>Id: {tx.id}</Typography>
							<Typography>Method: {tx.method}</Typography>
							<Typography>
								Amount: {fmtCurrency.format(tx.value)}
							</Typography>
						</Paper>
					</Grid>
				);
			})}
		</Grid>
	);
};

const styles = theme => ({
	root: {
		padding: theme.spacing(3),
	},
	paper: {
		padding: theme.spacing(2),
	},
});

export default withStyles(styles)(PaymentTableDetails);
