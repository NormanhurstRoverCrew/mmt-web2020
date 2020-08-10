import React from 'react';
import {withStyles} from '@material-ui/core/styles';
import classNames from 'classnames';

import {Drawer, IconButton, Divider} from '@material-ui/core';

import ChevronLeftIcon from '@material-ui/icons/ChevronLeft';
import HomeIcon from '@material-ui/icons/Home';
import BookingIcon from '@material-ui/icons/LibraryBooks';
import TicketIcon from '@material-ui/icons/Receipt';
import FoodIcon from '@material-ui/icons/Fastfood';

import Link from 'components/window/sidebar/Link';
import {windowActions} from '../../../actions/window.actions';

const drawerWidth = 240;

export const Sidebar = ({classes, sideBarOpen, toggleSideBar}) => {

		return (
			<Drawer
				variant="permanent"
				classes={{
					paper: classNames(
						classes.drawerPaper,
						!sideBarOpen && classes.drawerPaperClose,
					),
				}}
				open={sideBarOpen}>
				<div className={classes.drawerHeader}>
					<IconButton onClick={toggleSideBar} id="closeSidebar">
						<ChevronLeftIcon />
					</IconButton>
				</div>

				<Divider />

				<div>
					<Link icon={<HomeIcon />} to="/">
						Dashboard
					</Link>

					<Link icon={<BookingIcon />} to="/bookings">
						Bookings
					</Link>
					<Link icon={<TicketIcon />} to="/tickets">
						Tickets
					</Link>
					<Link icon={<TicketIcon />} to="/payments">
						Payments
					</Link>
					<Link icon={<FoodIcon />} to="/diets">
						Diets
					</Link>
					<Link icon={<TicketIcon />} to="/vehicles">
						Vehicles
					</Link>
					<Link icon={<TicketIcon />} to="/activities">
						Activities
					</Link>
				</div>
			</Drawer>
		);
};

const styles = theme => ({
	root: {},
	drawerPaper: {
		position: 'relative',
		whiteSpace: 'nowrap',
		width: drawerWidth,
		transition: theme.transitions.create('width', {
			easing: theme.transitions.easing.sharp,
			duration: theme.transitions.duration.enteringScreen,
		}),
	},
	drawerPaperClose: {
		overflowX: 'hidden',
		transition: theme.transitions.create('width', {
			easing: theme.transitions.easing.sharp,
			duration: theme.transitions.duration.leavingScreen,
		}),
		width: theme.spacing(7),
		[theme.breakpoints.up('sm')]: {
			width: theme.spacing(9),
		},
	},
	drawerHeader: {
		display: 'flex',
		alignItems: 'center',
		padding: '0 8px',
		...theme.mixins.toolbar,
		justifyContent: 'flex-end',
	},
});

export const mapStateToProps = state => ({
	isSideBarOpen: state.window.sidebar.open,
});

export const mapDispatchToProps = dispatch => ({
	toggleSideBar: () => dispatch(windowActions.toggleSideBar()),
});

export default withStyles(styles)(Sidebar);
