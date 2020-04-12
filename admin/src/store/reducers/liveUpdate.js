export default (state = true, action) => {
	switch (action.type) {
	case "LIVEUPDATE:SET":
		return action.live;

	default:
		return state;
	}
};
