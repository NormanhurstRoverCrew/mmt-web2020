import React, {Component, useContext} from 'react';
import PropTypes from 'prop-types';
import {connect} from 'react-redux';

import {auth0Actions} from 'actions/';

import {AuthContext} from 'context/AuthContext';

export const Callback = () => {
	return <div>Loading... Logging into Auth0</div>;
};

Callback.propTypes = {};

// shortend mapStateToProps to stop long line error at export
const mapSToP = state => {
	return {
		isAuthenticated: state.auth.isAuthenticated,
		expires_at: state.auth.expires_at,
	};
};

// export this so we can test the implementation of these methods.
export const mapDispatchToProps = dispatch => {
	return {
		handleAuthentication: (window, history, localStorage) =>
			dispatch(
				auth0Actions.handleAuthentication(window, history, localStorage),
			),
	};
};

export default Callback;
