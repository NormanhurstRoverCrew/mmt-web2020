import React, {useState} from "react";
import {connect} from "react-redux";
import PropTypes from "prop-types";
import classNames from "classnames";
import {withStyles} from "@material-ui/core/styles";
import {Typography, Grid, Modal, Paper, TextField, Button} from "@material-ui/core";

import CREWS from "components/../../crews.js";

import {ticketsActions} from "actions/tickets.actions";

export const AddTicket = ({classes, className}) => {
		const [editor_open, updateOpen] = useState(false);

		const [name, updateName] = useState("");
		const [mobile, updateMobile] = useState("");
		const [crew, updateCrew] = useState("");

	const handelTicketOpen = (e) => {
			updateOpen(true);
	};

	const handelTicketClose = (e) => {
			updateOpen(false);
	};

    const handleSaveButton = (e) => {
		this.props.doCreateTicket({name,mobile,crew});
			updateOpen(false);
			updateName("");
			updateMobile("");
			updateCrew("");
	};

		return (
			<div className={className}>
                <Button onClick={handelTicketOpen}>Add Ticket</Button>
				<Modal open={editor_open}
					onClose={handelTicketClose}>
					<Paper className={classNames(classes.paper)}>
                        <Typography variant="h3">Create New Ticket</Typography>
                        <TextField
                            id="name"
                            label="* Name"
                            value={name || ""}
                            onChange={(e) => updateName(e.target.value)}
                            margin="normal"
                            fullWidth
                            autoFocus={true}
                        />
                        <TextField
                            id="mobile"
                            label="* Mobile"
                            value={mobile || ""}
                            onChange={(e) => updateMobile(e.target.value)}
                            margin="normal"
                            fullWidth
                        />
                        <TextField
                            id="crew"
                            select
                            label="Crew"
                            value={crew || ""}
                            onChange={(e) => updateCrew(e.target.value)}
                            SelectProps={{
                                native: true,
                                MenuProps: {
                                    className: classes.menu,
                                },
                            }}
                            fullWidth
                            helperText="Select a Crew"
                            margin="normal"
                        >
                            <option key=""
                                value="" />
                            {CREWS.map((option) => (
                                <option key={option}
                                    value={option}>
                                    {option}
                                </option>
                            ))}
                        </TextField>
                        <Typography>
                            Only click 'Create' if the user has paid $40!
                        </Typography>
                        <Button variant="contained"
                            color="primary"
                            onClick={handleSaveButton}>Create</Button>
                    </Paper>
				</Modal>
			</div>
		);
};

AddTicket.propTypes = {
	classes: PropTypes.object.isRequired,
};

const styles = (theme) => ({
	paper: {
		padding: theme.spacing(2),
		textAlign: "justify",
		color: theme.palette.text.secondary,
	},
	detailText: {
		paddingRight: theme.spacing(3),
	},
});

const Detail = withStyles(styles)(
	(props) => (
		<Grid item
			xs={12}>
			<Typography variant="body1"
				noWrap
				className={classNames(props.classes.detailText)}>{props.title}: <Typography
					component="small"
					noWrap>{props.children}</Typography></Typography>
		</Grid>
	)
);

export default withStyles(styles)(AddTicket);
