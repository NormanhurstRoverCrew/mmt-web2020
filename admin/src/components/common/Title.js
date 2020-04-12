import React from "react";
import {Grid, Typography} from "@material-ui/core";

const Title = (props) => {
	return (
		<>
			<Grid item xs={12}>
				<Typography variant="h5"
					component="h1">
					{props.children}
				</Typography>
			</Grid>
		</>
	);
};

export default Title;
