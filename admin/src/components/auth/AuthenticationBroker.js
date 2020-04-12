import React, {Component, useContext, useEffect} from 'react';
import {connect} from 'react-redux';
import {auth0Actions} from 'actions/';
import PropTypes from 'prop-types';
import {AuthContext} from 'context/AuthContext';

export const AuthenticationBroker = ({children}) => {
	const {authenticated} = useContext(AuthContext);

	if (authenticated) {
		const childrenWithProps = React.Children.map(children, child =>
			React.cloneElement(child, {}),
		);
		return (
			<div id="authentication-broker" className="authenticed">
				{childrenWithProps}
			</div>
		);
	}
	return <div id="authentication-broker" className="un-authenticed"></div>;
};

AuthenticationBroker.propTypes = {
	children: PropTypes.object.isRequired,
};

// shortend mapStateToProps to stop long line error at export
export const mapSToP = state => {
	return {
		isAuthenticated: state.auth.isAuthenticated,
		expires_at: state.auth.expires_at,
	};
};

export const mapDispatchToProps = dispatch => {
	return {
		loadAuth: (window, localStorage) =>
			dispatch(auth0Actions.loadState(window, localStorage)),
		login: window => dispatch(auth0Actions.login(window)),
		logout: (window, localStorage) =>
			dispatch(auth0Actions.logout(window, localStorage)),
	};
};

export default AuthenticationBroker;
