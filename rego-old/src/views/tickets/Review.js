import React, {Component} from "react";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";

import Theme from "components/theme/Theme";
import {Grid, Paper, Typography, Button, Link} from "@material-ui/core";
import Norse from "components/theme/Norse";

export class Review extends Component {
	constructor(props) {
		super(props);
	}

	componentDidMount() {
		this.props.getTicket();
	}

	render() {
		const {classes} = this.props;

		return (
			<Theme shiftLeft>
				<Grid container
					direction="row-reverse">
					<Grid item
						lg={6}
						xs={10}>
						<Paper className={classNames(classes.fullHeight, classes.root)}>
							<Norse className={classNames(classes.bold, classes.padBottom)}
								variant="h3"
								black="true">You are on the way to VALHALLA</Norse>
							<Typography
								className={classNames(classes.bold, classes.padBottom)}
							>
								You have finished your verification. If you havent already, please click attending on the <Link href="https://www.facebook.com/events/2296000984006174/">Facebook Event</Link>, and make sure to invite your crew and friends.
							</Typography>
							<Typography
								className={classNames(classes.padBottom)}
							>
								You may close this page.
							</Typography>
							<Button
								onClick={this.props.gotoHome}
								color="primary"
								variant="contained"
								size="large"
								className={classNames(classes.padTop)}
								classes={{
									root: classes.buttonRoot,
								}}
							>
								Home
							</Button>
						</Paper>
					</Grid>
				</Grid>
			</Theme>
		);
	}
}

const styles = (theme) => ({
	root: {
		padding: "6em 4em",
		overflow: "auto",
	},
	fullHeight: {
		height: "100vh",
		borderRadius: "0",
		bottom: 0,
		top: 0,
		paddingBottom: "2em",
	},
	bold: {
		fontWeight: "bold",
	},
	buttonRoot: {
		padding: theme.spacing(1.75) + "px " + theme.spacing(3.5) + "px",
		borderRadius: "0",
	},
	padTop: {
		marginTop: "1.2em",
	},
	padBottom: {
		paddingBottom: "1em",
	},
});

export default withStyles(styles)(Review);
