// import React from "react";
import Bookings from "views/bookings/";
import Tickets from "views/tickets/";
import Payments from "views/payments/";
import Diets from "views/diets/";
import Vehicles from "views/vehicles/";
import Activities from "./activity/Activities";

export default [
	{
		path: "/bookings",
		view: Bookings,
	},
	{
		path: "/vehicles",
		view: Vehicles,
	},
	{
		path: "/tickets",
		view: Tickets,
	},
	{
		path: "/payments",
		view: Payments,
	},
	{
		path: "/diets",
		view: Diets,
	},
	{
		path: "/activities",
		view: Activities,
	},
];
