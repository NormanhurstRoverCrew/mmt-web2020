// import React from "react";
import Contact from "views/Contact";
import Register from "views/register/Register";
import Checkin from "views/checkin/Checkin";
import ConfirmEmail from "./register/ConfirmEmail";
import Purchase from "./register/Purchase";
import Verify from "./register/Verify";
import Checkout from "./register/Checkout";
import TicketIngress from "./tickets/Ingress";
import Ticket from "./tickets/Ticket";
import Review from "./tickets/Review";

export default [
	{
		path: "/purchase",
		view: Purchase,
	},
	{
		path: "/contact",
		view: Contact,
	},
	{
		path: "/register",
		view: Register,
	},
	{
		path: "/confirm_email",
		view: ConfirmEmail,
	},
	{
		path: "/verify",
		view: Verify,
	},
	{
		path: "/checkout",
		view: Checkout,
	},
	{
		path: "/ticket/edit",
		view: Ticket,
	},
	{
		path: "/ticket/:uid",
		view: TicketIngress,
	},
	{
		path: "/ticket",
		view: Review,
	},
	{
		path: "/checkin",
		view: Checkin,
	},
];
