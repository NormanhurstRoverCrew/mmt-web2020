import {MuiThemeProvider, createMuiTheme} from '@material-ui/core/styles';
import {MuiPickersUtilsProvider} from '@material-ui/pickers';
import React, {useState} from 'react';
import {withStyles} from '@material-ui/core/styles';
import classNames from 'classnames';
import CssBaseline from '@material-ui/core/CssBaseline';
import MomentUtils from '@date-io/moment';
import _ from 'underscore';

import Topbar from 'components/window/topbar/';
import Sidebar from 'components/window/sidebar/';
import Content from 'components/window/Content';
import {bookingsActions, teamsActions, activitiesActions} from 'actions/';

const theme = createMuiTheme({
	palette: {
		type: 'dark', // Switching the dark mode on is a single property value change.
	},
	typography: {useNextVariants: true},
});

export const Root = ({classes}) => {
	const [sideBarOpen, updateSideBarOpen] = useState(false);
	const toggleSideBar = () => updateSideBarOpen(!sideBarOpen);

	return (
		<div className={classNames(classes.root)}>
			<MuiThemeProvider theme={theme}>
				<MuiPickersUtilsProvider utils={MomentUtils}>
					<CssBaseline />
					<Topbar sideBarOpen={sideBarOpen} toggleSideBar={toggleSideBar} />
					<Sidebar sideBarOpen={sideBarOpen} toggleSideBar={toggleSideBar} />
					<Content />
				</MuiPickersUtilsProvider>
			</MuiThemeProvider>
		</div>
	);
};

const styles = theme => ({
	root: {
		position: 'absolute',
		top: 0,
		left: 0,
		display: 'flex',
		height: '100vh',
		width: '100vw',
	},
});

export default withStyles(styles)(Root);
