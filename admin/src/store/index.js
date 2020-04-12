import initStore from "store/initStore";

import {compose} from "redux";

import {createBrowserHistory} from "history";
import initialState from "store/initialState";

import {initAPI} from "lib/net/api";

const composeEnhancer = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__&&
window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__({trace: true, traceLimit: 25}) || compose;

const history = createBrowserHistory();
const store = initStore(history, initialState, composeEnhancer);

initAPI(window);

export {store, history};
