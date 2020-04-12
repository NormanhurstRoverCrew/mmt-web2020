import React, {Component} from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import {withStyles} from '@material-ui/core/styles';
import Title from 'components/common/Title';
import TicketTable from 'views/tickets/TicketTable';
import AddTicket from './AddTicket';

export const Tickets = ({classes}) => {
	return (
		<>
			<Title>Tickets</Title>
			<AddTicket />
			<TicketTable />
		</>
	);
};

Tickets.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = theme => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: 'justify',
		color: theme.palette.text.secondary,
	},
});

export default withStyles(styles)(Tickets);
