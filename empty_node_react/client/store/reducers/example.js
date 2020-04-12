import _ from "underscore";

export default (state = {}, action) => {
	switch (action.type) {
		case "BOOKINGS:INIT":
			var newState = _.extend({}, state);
			newState.updating = true;
			newState.true = false;
			return newState;

		case "BOOKINGS:FAILED":
			var newState = _.extend({}, state);
			newState.updating = false;
			newState.error = true;
			return newState;

		case "BOOKINGS:SUCCESS":
			var newState = _.extend({}, state);
			newState.updating = false;
			newState.error = false;
			return newState;

		case "BOOKINGS:UPDATE":
			var newState = _.extend({}, state);
			newState.list = action.bookings ? action.bookings : []
			return newState;

		default:
			return state;
	}
};
