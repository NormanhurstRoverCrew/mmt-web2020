import {applyMiddleware, compose, createStore} from "redux";
import thunk from "redux-thunk";
import {createBrowserHistory} from "history";
import {routerMiddleware} from "connected-react-router";

import rootReducer from "store/reducers";
import initialState from "store/initialState";

const history = createBrowserHistory();

const composeEnhancer = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
const store = createStore(
	rootReducer(history), // root reducer with router state
	initialState,
	composeEnhancer(
		applyMiddleware(
			routerMiddleware(history), // for dispatching history actions
			thunk
		)
	)
);

export {store, history};
