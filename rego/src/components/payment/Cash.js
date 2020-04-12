import React, {Component} from "react";
import {Typography, Paper} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";

export class Cash extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {tickets, classes} = this.props;
		return (
			<div>
				<Typography variant="h5"
					className={classNames(classes.bold, classes.padTop, classes.pad)}>
					Cash on the day
				</Typography>
				<Typography>
					You must bring exact hard cash for entry to Valhalla. Please bring <b>{new Intl.NumberFormat("en-AU", {style: "currency", currency: "AUD"}).format(tickets * 40.00)}</b> to the regestrations tent.
				</Typography>
			</div>
		);
	}
}

const styles = (theme) => ({
	bold: {
		fontWeight: "bold",
	},
	padTop: {
		paddingTop: "1em",
	},
	pad: {
		paddingBottom: "1vw",
	},
});

export default withStyles(styles)(Cash);
