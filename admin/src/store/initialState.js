const initialState = {
	auth: {
		isAuthenticated: false,
		expires_at: 0,
	},
	window: {
		sidebar: {
			open: false,
		},
	},
	bookings: [],
	teams: [],
	activities: [],
	liveUpdate: false,
};

export default initialState;
