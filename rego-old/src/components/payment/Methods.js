import React, {Component} from "react";
import {connect} from "react-redux";
import {Grid} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";

import Paypal from "components/payment/Paypal";
import Eft from "components/payment/Eft";
import Cash from "components/payment/Cash";

export class Methods extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<Grid container
				spacing={3}>
				<Grid item
					xs={4}>
					<Paypal/>
				</Grid>
				<Grid item
					xs={4}>
					<Eft/>
				</Grid>
				<Grid item
					xs={4}>
					<Cash/>
				</Grid>
			</Grid>
		);
	}
}

const styles = (theme) => ({

});

const mapStateToProps = (state) => {
	return {

	};
};

const mapDispatchToProps = (dispatch) => {
	return {

	};
};

export default connect(mapStateToProps, mapDispatchToProps)(withStyles(styles)(Methods));
