import React, {useState, createContext, useEffect} from 'react';
import _ from 'underscore';
import {gql} from 'apollo-boost';
import {useQuery, useLazyQuery} from '@apollo/react-hooks';
import {useHistory, useLocation} from 'react-router-dom';

import auth0 from 'auth0-js';

export const AuthContext = createContext();

const createAuth0 = () => {
	const arr = window.location.href.split('/');

	return new auth0.WebAuth({
		domain: 'normorovers.au.auth0.com',
		clientID: 'OQs8FmpqhKurwcf9XR076O0iYB5ihdWI',
		redirectUri: window.location.origin + '/callback',
		audience: window.location.origin + '/',
		responseType: 'token id_token',
		scope: 'openid profile full',
	});
};

const getExpiry = seconds => {
	return seconds * 1000 + new Date().getTime();
};

const isAuthed = exp => new Date().getTime() < getExpiry(exp);

// Hook
const useLocalStorage = (key, initialValue) => {
	// State to store our value
	// Pass initial state function to useState so logic is only executed once
	const [storedValue, setStoredValue] = useState(() => {
		try {
			// Get from local storage by key
			const item = window.localStorage.getItem(key);
			// Parse stored json or if none return initialValue
			return item ? JSON.parse(item) : initialValue;
		} catch (error) {
			// If error also return initialValue
			console.log(error);
			return initialValue;
		}
	});

	// Return a wrapped version of useState's setter function that ...
	// ... persists the new value to localStorage.
	const setValue = value => {
		try {
			// Allow value to be a function so we have same API as useState
			const valueToStore =
				value instanceof Function ? value(storedValue) : value;
			// Save state
			setStoredValue(valueToStore);
			// Save to local storage
			window.localStorage.setItem(key, JSON.stringify(valueToStore));
		} catch (error) {
			// A more advanced implementation would handle the error case
			console.log(error);
		}
	};

	return [storedValue, setValue];
};

const AuthContextProvider = ({children}) => {
	const history = useHistory();
	const l = useLocation();

	const [a0, updateA0] = useState(createAuth0());

	const [auth, updateAuth] = useState({
		access_token: localStorage.getItem('access_token'),
		id_token: localStorage.getItem('id_token'),
		expires_at: parseInt(localStorage.getItem('expires_at')),
		name: localStorage.getItem('name'),
		avatar: localStorage.getItem('avatar'),
		scopes: JSON.parse(localStorage.getItem('scopes')),
		email: localStorage.getItem('email'),
	});
	const [access_token, updateAccessToken] = useLocalStorage(
		'access_token',
		null,
	);
	const [expires_at, updateExpiresAt] = useLocalStorage('expires_at', null);

	const [authenticated, updateAuthenticated] = useState(false);
	const [notCallback, updateNotCallback] = useState(false);

	const login = () => {
		a0.authorize();
	};

	const logout = () => {
		localStorage.clear();
		// updateAuth({
		// 	expires_at: 0,
		// });
		updateAuthenticated(false);
		login();
	};


	const setSession = authResult => {
		// Set the time that the access token will expire at
		let expiresAt = authResult.expiresIn * 1000 + new Date().getTime();
		expiresAt = JSON.stringify(expiresAt);

		// If there is a value on the `scope` param from the authResult,
		// use it to set scopes in the session for the user. Otherwise
		// use the scopes as requested. If no scopes were requested,
		// set it to nothing
		const scopes = authResult.scope || '';

		updateAccessToken(authResult.accessToken);
		updateExpiresAt(expiresAt);

		localStorage.setItem('id_token', authResult.idToken);
		localStorage.setItem('name', authResult.idTokenPayload.nickname);
		localStorage.setItem('avatar', authResult.idTokenPayload.picture);
		localStorage.setItem('scopes', JSON.stringify(scopes));
		localStorage.setItem('email', authResult.idTokenPayload.email);

		updateAuth(authResult);
		history.push('/');
	};

	const handleAuthCallback = () => {
		a0.parseHash((err, authResult) => {
			if (err) {
				return console.log('parseHash', err);
			}

			setSession(authResult);
		});
	};

	useEffect(() => {
		if (location.pathname === '/callback') {
			if (/access_token|id_token|error/.test(l.hash)) {
				handleAuthCallback();
			}
		} else {
			updateNotCallback(true);
		}
	}, [a0]);

	const [logoutTimer, updateLogoutTimer] = useState(null);

	useEffect(() => {
		if (logoutTimer) {
			clearTimeout(logoutTimer);
		}
		if (!authenticated) {
			updateLogoutTimer(
				setTimeout(() => {
					logout();
				}, 3000),
			);
		}
	}, [authenticated]);

	useEffect(() => {
		const bearer = 'Bearer ' + access_token;
		axios.defaults.headers.common['Authorization'] = bearer;
	}, [access_token]);

	useEffect(() => {
		if (expires_at) {
				updateAuthenticated(isAuthed(parseInt(expires_at)));
			const timeTillExpire = Math.max(0, expires_at - new Date().getTime());
			setTimeout(() => {
				logout();
			}, timeTillExpire);
		}
	}, [expires_at]);

	const obj = {
		authenticated,
	};

	return <AuthContext.Provider value={obj}>{children}</AuthContext.Provider>;
};

export default AuthContextProvider;
