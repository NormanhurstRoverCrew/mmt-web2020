import React, {Component} from 'react';
import PropTypes from 'prop-types';
import Title from 'components/common/Title';

import BookingsByTime from 'components/data/BookingsByTime';
// import TicketsByTime from "../components/data/TicketsByTime";
import DashboardParade from './DashboardParade';
import DashboardParadePoints from './DashboardParadePoints';

export const Dashboard = () => {
	return (
		<>
			<Title>Dashboard</Title>
			<DashboardParade />
			<DashboardParadePoints />
			<BookingsByTime />
			{/* <TicketsByTime /> */}
		</>
	);
};

Dashboard.propTypes = {};

export default Dashboard;
