import auth0 from "auth0-js";

import axios from "axios";

const createAuth0 = (window) => {
	const arr = window.location.href.split("/");

	return new auth0.WebAuth({
		domain: "normorovers.au.auth0.com",
		clientID: "OQs8FmpqhKurwcf9XR076O0iYB5ihdWI",
		redirectUri: arr[0] + "//" + arr[2] + "/callback",
		audience: arr[0] + "//" + arr[2] + "/",
		responseType: "token id_token",
		scope: "openid profile full",
	});
};

const login = (window) => {
	return (dispatch) => {
		auth0Actions.createAuth0(window).authorize();
	};
};

const logout = (window, localStorage) => {
	return (dispatch) => {
		// Clear access token and ID token from local storage
		localStorage.removeItem("access_token");
		localStorage.removeItem("id_token");
		localStorage.removeItem("expires_at");
		localStorage.removeItem("name");
		localStorage.removeItem("avatar");
		localStorage.removeItem("scopes");
		localStorage.removeItem("email");
		// navigate to the home route

		dispatch({type: "AUTH:LOGOUT"});
		dispatch(auth0Actions.login(window));
		// this.history.push("/")
		// history.replace('/');
	};
};

// get the previous auth deets from localStorage
const loadState = (window, localStorage) => {
	return (dispatch) => {
		const auth = {
			access_token: localStorage.getItem("access_token"),
			id_token: localStorage.getItem("id_token"),
			expires_at: parseInt(localStorage.getItem("expires_at")),
			name: localStorage.getItem("name"),
			avatar: localStorage.getItem("avatar"),
			scopes: JSON.parse(localStorage.getItem("scopes")),
			email: localStorage.getItem("email"),
		};

		if (auth.expires_at > new Date().getTime()) {
			const bearer = "Bearer " + auth.access_token;
			axios.defaults.headers.common["Authorization"] = bearer;
			dispatch({
				type: "AUTH:LOGIN:LOCAL",
				auth,
			});
		} else {
			dispatch(auth0Actions.login(window));
		}
	};
};


// get the previous auth deets from localStorage
const handleAuthentication = (window, history, localStorage) => {
	return (dispatch, getState) => {
		auth0Actions.createAuth0(window).parseHash((err, authResult) => {
			if (authResult && authResult.accessToken && authResult.idToken) {
				dispatch(auth0Actions.setSession(authResult, localStorage));
				// history.replace("/");
				// history.push("/");
				dispatch(push("/"));
			} else if (err) {
				// this.history.push("/")
				// history.replace('/');
				dispatch(logout(window, localStorage));
			}
		});
	};
};

const setSession = (authResult, localStorage) => {
	return (dispatch) => {
		// Set the time that the access token will expire at
		let expiresAt = (authResult.expiresIn * 1000) + new Date().getTime();
		expiresAt = JSON.stringify(expiresAt);

		// If there is a value on the `scope` param from the authResult,
		// use it to set scopes in the session for the user. Otherwise
		// use the scopes as requested. If no scopes were requested,
		// set it to nothing
		const scopes = authResult.scope || "";

		localStorage.setItem("access_token", authResult.accessToken);
		localStorage.setItem("id_token", authResult.idToken);
		localStorage.setItem("expires_at", expiresAt);
		localStorage.setItem("name", authResult.idTokenPayload.nickname);
		localStorage.setItem("avatar", authResult.idTokenPayload.picture);
		// store scopes
		localStorage.setItem("scopes", JSON.stringify(scopes));
		localStorage.setItem("email", authResult.idTokenPayload.email);

		const bearer = "Bearer " + authResult.accessToken;
		axios.defaults.headers.common["Authorization"] = bearer;

		dispatch({
			type: "AUTH:LOGIN",
			auth: authResult,
		});

		// navigate to the home route
		// this.history.push("/")
		// history.replace('/');
	};
};

const auth0Actions = {
	login,
	logout,
	handleAuthentication,
	setSession,
	loadState,
	createAuth0,
};

export {auth0Actions};
