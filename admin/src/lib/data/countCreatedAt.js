import _ from "underscore";

const countCreatedAt = (input, name) => {
	const withDate = _.map(input, (booking) => {
		return _.extend({}, booking, {created_at: new Date(booking.created_at)});
	});

	let start = new Date(8000000000000);
	let end = new Date(0);

	_.each(withDate, (booking) => {
		if (booking.created_at < start) {
			start = booking.created_at;
		}
		if (booking.created_at > end) {
			end = booking.created_at;
		}
	});

	start = Math.floor(start.getTime() / 86400000);
	end = Math.ceil(end.getTime() / 86400000);

	// because initially the props are empty cause redux hasnt been updated. we get some weird values.
	// catch this condition
	if (end < start) return [];

	let counted = _.range(start, end + 1, 1);

	counted = _.map(counted, (day) => {
		return {name: new Date(day * 86400000).toLocaleDateString(), [name]: 0};
	});

	const bookingsWithDateString = _.map(withDate, (booking) => {
		return _.extend({}, booking, {created_at: new Date(booking.created_at).toLocaleDateString()});
	});

	let countedBookingsByDate = _.countBy(bookingsWithDateString, "created_at");

	countedBookingsByDate = _.map(countedBookingsByDate, (value, key) => {
		return {name: key, [name]: value};
	});

	counted = _.map(counted, (date) => {
		const count = _.filter(countedBookingsByDate, (b) => b.name == date.name);
		_.each(count, (c) => {
			date[name] += c[name];
		});
		return date;
	});

	return counted;
};


export default countCreatedAt;
