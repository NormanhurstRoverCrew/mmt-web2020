import {Grid, Typography} from "@material-ui/core";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import Norse from "components/theme/Norse";
import React, {Component} from "react";

import beermug from "img/beer-mug.svg";
import mapsmark from "img/maps-mark.svg";
import pancar from "img/style-one-pin-car.svg";


class About extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		const {classes, className} = this.props;
		return (
			<div className={className}
				id="about">
				<Grid container
					spacing={5}>
					<Grid item
						container
						direction="row">
						<Grid item
							xs={12}
							sm={4}>
							<Norse variant="h3"
								className={classNames(classes.aboutHeaders)}>
								Drink Mead and
							</Norse>
							<Norse variant="h3"
								className={classNames(classes.aboutHeaders)}>
								Be Merry
							</Norse>
						</Grid>
						<Grid item
							xs={12}
							sm={8}>
							<Typography>
								Normanhurst Rover Crew’s annual Magical Mystery Tour is back again, this year presenting to you Magical Mystery Tour to Valhalla. Join us in your teams to drink mead, fight battles and hail the gods. All Rovers, Olaves and Ex-Rovers are welcome (just need to be sponsored by a crew).
							</Typography>
							<Typography>By popular demand, our secret end point is back at a pub again this year! </Typography>
						</Grid>
					</Grid>
					<Grid item
						container>
						<Grid item
							xs={12}
							sm={4}>
							<Norse variant="h3"
								className={classNames(classes.aboutHeaders)}>
								What to Expect
							</Norse>
						</Grid>
						<Grid item
							container
							xs={12}
							sm={8}
							spacing={5}>
							<AboutItem title="Meeting Point"
								icon="/client/img/style-one-pin-car.svg">
								Meet your team at Thornleigh Macca’s overflow carpark at 8am on Saturday morning
							</AboutItem>
							<AboutItem title="Cryptic clues and bases"
								icon="/client/img/maps-mark.svg">
								Cryptic clues will lead you on a journey stopping at activity bases where you will complete team challenges
							</AboutItem>
							<AboutItem title="The end point"
								icon="/client/img/beer-mug.svg">
								You will arrive at a mystery location approximately 2 hours outside of Sydney where good times await you
							</AboutItem>
						</Grid>
					</Grid>
				</Grid>
			</div>
		);
	}
}


const padLeft = "8vw";
const padVert = "10vw";

const styles = (theme) => ({
	aboutBlock: {
		paddingLeft: padLeft,
		paddingRight: padLeft,
		paddingTop: padVert,
		paddingBottom: padVert,
	},
	aboutIcons: {
		// borderRadius: "100%",
		// border: "solid #979797 1px",
		backgroundColor: "transparent",
		padding: "0.5em",
		width: "3em",
		height: "3em",
	},
	aboutItemTitle: {
		color: "rgb(217, 131, 10)",
	},
	aboutHeaders: {
		color: "black",
		fontWeight: "bold",
	},
});

const AboutItem = withStyles(styles)((props) => {
	const {classes, title, children, icon} = props;
	return (
		<Grid item
			container
			direction='row'
			alignItems="center">
			<Grid item
				sm={1}
				xs={2}>
				<div>
					<img className={classNames(classes.aboutIcons)}
						src={icon}/>
				</div>
			</Grid>
			<Grid item
				container
				direction='column'
				sm={9}
				xs={10}>
				<Grid item>
					<Typography variant="h5"
						className={classNames(classes.aboutItemTitle)}>
						{title}
					</Typography>
				</Grid>
				<Grid item>
					<Typography>
						{children}
					</Typography>
				</Grid>
			</Grid>
		</Grid>
	);
});

export default withStyles(styles)(About);
