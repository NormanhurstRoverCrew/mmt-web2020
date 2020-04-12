import React, {Component} from 'react';
import PropTypes from 'prop-types';
import {Route, Switch} from 'react-router-dom';
import {withRouter} from 'react-router-dom';

import Root from 'components/window/';

import Callback from './auth/Callback';

import AuthenticationBroker from './auth/AuthenticationBroker';
import AuthContextProvider from 'context/AuthContext';
import BookingContextProvider from 'context/BookingContext';

export const App = props => {
	return (
		<Switch>
			<AuthContextProvider>
				<Route
					path="/callback"
					render={props => {
						return <Callback {...props} {...props} />;
					}}
				/>
				<Route
					path="/"
					render={props => (
						<AuthenticationBroker {...props}>
							<BookingContextProvider>
								<Root {...props} />
							</BookingContextProvider>
						</AuthenticationBroker>
					)}
				/>
			</AuthContextProvider>
		</Switch>
	);
};

App.propTypes = {};

export default withRouter(App);
