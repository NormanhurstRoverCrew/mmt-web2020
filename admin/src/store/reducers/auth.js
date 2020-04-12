import _ from "underscore";

export const getExpiry = (seconds) => {
	return (seconds * 1000) + new Date().getTime();
};

export default (state = {}, action) => {
	switch (action.type) {
	case "AUTH:LOGIN":
		return {
			access_token: action.auth.accessToken,
			id_token: action.auth.idToken,
			expires_at: getExpiry(action.auth.expiresIn),
			name: action.auth.idTokenPayload.nickname,
			avatar: action.auth.idTokenPayload.picture,
			scopes: JSON.stringify(action.auth.scope || ""),
			email: action.auth.idTokenPayload.email,
			isAuthenticated: new Date().getTime() < getExpiry(action.auth.expiresIn),
		};

	case "AUTH:LOGIN:LOCAL":
		return {
			access_token: action.auth.access_token,
			id_token: action.auth.id_token,
			expires_at: action.auth.expires_at,
			name: action.auth.name,
			avatar: action.auth.avatar,
			scopes: action.auth.scopes,
			email: action.auth.email,
			isAuthenticated: new Date().getTime() < action.auth.expires_at,
		};

	case "AUTH:LOGOUT":
		return {
			isAuthenticated: false,
			expires_at: 0,
		};

	default:
		return state;
	}
};
