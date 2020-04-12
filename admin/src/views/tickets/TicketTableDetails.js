import React from "react";
import {connect} from "react-redux";

import Ticket from "views/tickets/Ticket";

export const TicketTableDetails = ({ticket}) => {
		return (<Ticket ticket={ticket}/>);
};

export default TicketTableDetails;
