import React from "react";
import {connect} from "react-redux";
import _ from "underscore";

import MaterialTable from "material-table";
import {Grid, Modal} from "@material-ui/core";
import TicketTableDetails from "views/tickets/TicketTableDetails";
import EmailSelector from "components/EmailSelector";

export class DietTable extends React.Component {
	constructor(props) {
		super(props);

		this.handleSendEmail = this.handleSendEmail.bind(this);
		this.handelEmailSelectorClose = this.handelEmailSelectorClose.bind(this);

		this.state = {
			emailSelectDialog: false,
			tickets: [],
		};
	}

	handleSendEmail(e, rows) {
		this.setState({
			emailSelectDialog: true,
			tickets: rows,
		});
	}

	handelEmailSelectorClose(e) {
		this.setState({emailSelectDialog: false, tickets: []});
	}

	render() {
		return (
			<>
			<Grid item
				xs={12}>
				<MaterialTable
					columns={[
						{title: "ID", field: "id", type: "numeric"},
						{title: "Name", field: "user.name"},
						{title: "Diet", field: "user.diet"},
					]}
					data={this.props.tickets}
					title="Tickets Diet"
					detailPanel={[
						{
							tooltip: "Show Details",
							render: (rowData) => {
								return (<TicketTableDetails ticket={rowData} />);
							},
						},
					]}
					onRowClick={(event, rowData, togglePanel) => togglePanel(0)}
					options={{
						selection: true,
						pageSize: 20,
						pageSizeOptions: [20, 50, 100, 200, 250],
					}}
					actions={[
						{
							icon: "done_all",
							tooltip: "Send Email",
							onClick: this.handleSendEmail,
						},
					]}
				/>
			</Grid>
			<Modal open={this.state.emailSelectDialog}
				onClose={this.handelEmailSelectorClose}>
				<EmailSelector tickets={this.state.tickets}/>
			</Modal>
				</>
		);
	}
}

// shortend mapStateToProps to stop long line error at export
const mapSToP = (state) => {
	return {
		// take all the tickets from each booking...
		// this will return n(bookings) arrays of tickets.
		// flatten them into a linear array...
		tickets: _(_.flatten(state.bookings.map((booking) => booking.tickets)))
			.filter((ticket) => {
				if (ticket.user.diet) {
					return ticket.user.diet.length > 0;
				}
				return false;
			}),
	};
};


// export this so we can test the implementation of these methods.
export const mapDispatchToProps = (dispatch) => {
	return {
	};
};

export default connect(mapSToP, mapDispatchToProps)(DietTable);
