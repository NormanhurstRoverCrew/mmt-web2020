import _ from "underscore";

export default (state = {}, action) => {
	switch (action.type) {
	case "WINDOW:SIDEBAR:TOGGLE":
		var newState = _.clone(state);
		newState.sidebar.open = !state.sidebar.open;
		return newState;

	default:
		return state;
	}
};
