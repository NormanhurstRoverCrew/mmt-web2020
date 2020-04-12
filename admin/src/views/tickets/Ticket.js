import React, {Component, useState, useContext} from 'react';
import _ from 'underscore';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import {withStyles} from '@material-ui/core/styles';
import {
	Typography,
	Grid,
	Modal,
	Card,
	CardContent,
	CardActions,
	FormControlLabel,
	Checkbox,
	Button,
} from '@material-ui/core';
import TicketEditor from 'components/TicketEditor';

import {BookingContext} from 'context/BookingContext';

export const Ticket = ({
	classes,
	onClick,
	ticket,
	noEmail,
	noDelete,
	noVerified,
	className,
	classPaper,
}) => {
	const {updateTicket, deleteTickets} = useContext(BookingContext);

	const {user} = ticket;
	const [editor_open, setEditorOpen] = useState(false);

	const handelTicketClicked = e => {
		if (onClick) {
			onClick(e);
		} else {
			setEditorOpen(true);
		}
	};

	const handelTicketClose = e => {
		setEditorOpen(false);
	};

	const emailVerifiedChanged = e => {
		e.preventDefault();
		e.stopPropagation();
		const t = _.extend({}, ticket, {
			user: _.extend(ticket.user, {emailVerified: e.target.checked}),
		});
		// console.log(t);
		updateTicket(t);
	};

	const handleDelete = e => {
		e.preventDefault();
		e.stopPropagation();

		deleteTickets([ticket]);
	};

	return (
		<div className={className}>
			<Card
				className={classNames(classes.paper, classPaper)}
				onClick={handelTicketClicked}>
				<CardContent>
					<Typography variant="h6" component="h1" align="left">
						{(user && user.name) || ' - '}
					</Typography>
					<Grid container>
						<Detail title="Crew">{user && user.crew}</Detail>
						{!noEmail && (
							<Detail title="Email">{(user && user.email) || ' - '}</Detail>
						)}
						<Detail title="Mobile">{(user && user.mobile) || ' - '}</Detail>
					</Grid>
				</CardContent>
				<CardActions>
					{!noVerified && (
						<FormControlLabel
							control={
								<Checkbox
									checked={user.emailVerified}
									onClick={emailVerifiedChanged}
								/>
							}
							label="Verified"
						/>
					)}
					{!noDelete && <Button onClick={handleDelete}>Delete</Button>}
				</CardActions>
			</Card>
			<Modal open={editor_open} onClose={handelTicketClose}>
				<TicketEditor ticket={ticket} onSave={handelTicketClose} />
			</Modal>
		</div>
	);
};

Ticket.propTypes = {
	ticket: PropTypes.object.isRequired,
	classes: PropTypes.object.isRequired,
};

const styles = theme => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: 'justify',
		color: theme.palette.text.secondary,
	},
	detailText: {
		paddingRight: theme.spacing(3),
	},
});

const Detail = withStyles(styles)(props => (
	<Grid item xs={12}>
		<Typography
			variant="body1"
			noWrap
			className={classNames(props.classes.detailText)}>
			{props.title}:{' '}
			<Typography component="small" noWrap>
				{props.children}
			</Typography>
		</Typography>
	</Grid>
));

export default withStyles(styles)(Ticket);
