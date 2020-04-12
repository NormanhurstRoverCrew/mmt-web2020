const set = (live) => {
	return {
		type: "LIVEUPDATE:SET",
		live,
	};
};

const liveUpdateActions = {
	set,
};

export {liveUpdateActions};
