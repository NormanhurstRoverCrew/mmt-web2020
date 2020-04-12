import _ from "underscore";

export default (state = [], action) => {
	switch (action.type) {

		case "ACTIVITIES:UPDATE":
		return _.map(action.activities, (activity) => {
			const old = _.find(state, (oldactivity) => {
				return oldactivity.uid == activity.uid;
			});

			if (old) {
				return _.extend({}, old, activity);
			} else {
				return _.extend({}, activity);
			}
		});

	case "ACTIVITY:UPDATE":
		return _.map(state, (activity) => {
			if (activity.uid == action.activity.uid) {
				return _.extend({}, activity, action.activity);
			} else {
				return _.extend({}, activity);
			}
		});

	default:
		return state;
	}
};
