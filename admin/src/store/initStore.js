import {applyMiddleware, compose, createStore} from "redux";
import thunk from "redux-thunk";
import {routerMiddleware} from "connected-react-router";

import rootReducer from "store/reducers";

export default (history, initialState, composeEnhancer = compose) => {
	return createStore(
		rootReducer(history), // root reducer with router state
		initialState,
		composeEnhancer(
			applyMiddleware(
				routerMiddleware(history), // for dispatching history actions
				thunk
			)
		)
	);
};
