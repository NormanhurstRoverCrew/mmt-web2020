import {combineReducers} from "redux";
import {connectRouter} from "connected-react-router";

import auth from "store/reducers/auth";
import window from "store/reducers/window";
import bookings from "store/reducers/bookings";
import teams from "store/reducers/teams";
import activities from "store/reducers/activities";
import liveUpdate from "store/reducers/liveUpdate";

export default (history) => combineReducers({
	router: connectRouter(history),
	auth,
	window,
	bookings,
	teams,
	activities,
	liveUpdate,
});
