// This is an example of a redux/thunk action dispatch

import {auth} from "actions/.."; // the same as /client

const login = (auth, local = false) => {
	return (dispatch) => {
		dispatch({
			type: local ? "AUTH:LOGIN" : "AUTH:LOGIN:LOCAL",
			auth,
		});
	};
};

const logout = () => {
	return (dispatch) => {
		dispatch({type: "AUTH:LOGOUT"});
		auth.login();
	};
};

// get the previous auth deets from localStorage
const loadState = () => {
	return (dispatch, getState) => {
		// dispatch(setStatus({ready: false}));
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
			dispatch(login(auth), true);
		} else {
			dispatch(logout());
		}
	};
};

const setStatus = (status = {}) => ({
	type: "AUTH:STATUS:SET",
	status,
});

export const authActions = {
	login,
	logout,
	loadState,
	setStatus,
};
