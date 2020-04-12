const toggleSideBar = (window) => {
	return {
		type: "WINDOW:SIDEBAR:TOGGLE",
	};
};

const windowActions = {
	toggleSideBar,
};

export {windowActions};
