import axios from "axios";
import _ from "underscore";

const pull = () => {
	return async (dispatch) => {
		const {data} = await axios.get("/activities");
		const activities = _.map(data, (activity) => _.extend({}, activity, {
			created_at: new Date(activity.created_at).getTime(),
			updated_at: new Date(activity.updated_at).getTime(),
			logged_at: new Date(activity.logged_at).getTime(),
		}));

		dispatch(update(activities));
	};
};


const update = (activities) => {
	return {
		type: "ACTIVITIES:UPDATE",
		activities,
	};
};

const activitiesActions = {
	pull,
	update,
};

export {activitiesActions};
