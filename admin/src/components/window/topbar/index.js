import React, {useState, useEffect, useContext} from 'react';
import {withStyles} from '@material-ui/core/styles';
import classNames from 'classnames';

import Typography from '@material-ui/core/Typography';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import IconButton from '@material-ui/core/IconButton';
import Badge from '@material-ui/core/Badge';

import MenuIcon from '@material-ui/icons/Menu';
import UpdateIcon from '@material-ui/icons/Update';
import NotificationsIcon from '@material-ui/icons/Notifications';
import AccountCircle from '@material-ui/icons/AccountCircle';
import {windowActions} from 'actions/window.actions';
import {liveUpdateActions} from 'actions/liveUpdate.actions';
import {Menu, MenuItem, FormControlLabel, Switch} from '@material-ui/core';
import {BookingContext} from 'context/BookingContext';
import {VehicleContext} from 'context/VehicleContext';

const drawerWidth = 240;

export const Topbar = ({classes, sideBarOpen, toggleSideBar}) => {
	const {reloadData: reloadBookingData} = useContext(BookingContext);
	const {reloadData: reloadVehicleData} = useContext(VehicleContext);
	const [anchorEl, updateAnchorEl] = useState(null);

	const handleClick = e => {
		updateAnchorEl(e.currentTarget);
	};

	const handleClose = () => {
		updateAnchorEl(null);
	};

	/*<FormControlLabel
							control={
								<Switch
									checked={this.props.liveUpdate}
									onChange={this.handleToggleLiveChange}
									// value="checkedA"
								/>
							}
							label="Live"
						/>*/

	const handleLogout = () => {
		localStorage.clear();
		location.reload();
	};

	return (
		<div className={classes.root}>
			<AppBar
				className={classNames(
					classes.appBar,
					sideBarOpen && classes.appBarShift,
				)}
				position="absolute">
				<Toolbar
					disableGutters={!sideBarOpen}
					className={classNames(classes.toolbar)}>
					<IconButton
						id="openSidebar"
						color="inherit"
						aria-label="Open drawer"
						onClick={toggleSideBar}
						className={classNames(
							classes.menuButton,
							sideBarOpen && classes.menuButtonHidden,
						)}>
						<MenuIcon />
					</IconButton>
					<Typography
						component="h1"
						variant="h6"
						color="inherit"
						className={classNames(classes.title)}>
						MMT2021 Admin
					</Typography>
					<IconButton color="inherit" onClick={() => {
							reloadBookingData();
							reloadVehicleData();
					}}>
						<UpdateIcon />
					</IconButton>
					<IconButton
						color="inherit"
						aria-owns={anchorEl ? 'simple-menu' : undefined}
						aria-haspopup="true"
						onClick={handleClick}>
						<AccountCircle />
					</IconButton>
					<Menu
						id="simple-menu"
						anchorEl={anchorEl}
						open={Boolean(anchorEl)}
						onClose={handleClose}>
						<MenuItem onClick={handleClose}>Profile</MenuItem>
						<MenuItem
							onClick={e => {
								handleLogout();
								handleClose();
							}}>
							Logout
						</MenuItem>
					</Menu>
				</Toolbar>
			</AppBar>
		</div>
	);
};

export const styles = theme => ({
	root: {},
	toolbar: {
		paddingRight: 24, // keep right padding when drawer closed
	},
	toolbarIcon: {
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'flex-end',
		padding: '0 8px',
		...theme.mixins.toolbar,
	},
	appBar: {
		zIndex: theme.zIndex.drawer + 1,
		transition: theme.transitions.create(['width', 'margin'], {
			easing: theme.transitions.easing.sharp,
			duration: theme.transitions.duration.leavingScreen,
		}),
	},
	appBarShift: {
		marginLeft: drawerWidth,
		width: `calc(100% - ${drawerWidth}px)`,
		transition: theme.transitions.create(['width', 'margin'], {
			easing: theme.transitions.easing.sharp,
			duration: theme.transitions.duration.enteringScreen,
		}),
	},
	menuButton: {
		marginLeft: 12,
		marginRight: 36,
	},
	menuButtonHidden: {
		display: 'none',
	},
	title: {
		flexGrow: 1,
	},
});

const mapStateToProps = state => ({
	sideBarOpen: state.window.sidebar.open,
	liveUpdate: state.liveUpdate,
});

const mapDispatchToProps = dispatch => ({
	toggleSideBar: () => dispatch(windowActions.toggleSideBar()),
	setLiveUpdate: live => dispatch(liveUpdateActions.set(live)),
});

export default withStyles(styles)(Topbar);
