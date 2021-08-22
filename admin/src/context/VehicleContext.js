import React, {useState, createContext, useEffect} from 'react';
import _ from 'underscore';
import {gql} from 'apollo-boost';
import {useQuery, useMutation} from '@apollo/react-hooks';
import { onError } from "apollo-link-error";

export const VehicleContext = createContext();

const GET_VEHICLE = gql`
	query {
		vehicles {
			id
			rego
			name
			driver {
				id
			}
			tickets {
				id
			}
			requests {
				id
			}
		}
	}
`;

const APPROVE_TICKET = gql`
	mutation ApproveTicket($vehicle: ObjectId!, $ticket: ObjectId!) {
		addTicketToVehicle(vehicle: $vehicle, ticket: $ticket) {
			id
		}
	}
`;

const DENY_TICKET = gql`
	mutation DenyTicket($vehicle: ObjectId!, $ticket: ObjectId!) {
		removeTicketFromVehicle(vehicle: $vehicle, ticket: $ticket) {
			id
		}
	}
`;

const VehicleContextProvider = ({children}) => {
	const [vehicles, updateVehicles] = useState([]);

	const {data: vehiclesQueryData, refetch: refetchVehicles} = useQuery(
		GET_VEHICLE,
	);

	const [_approveTicket, {data: dataApproveTicket}] = useMutation(APPROVE_TICKET);
	const [_denyTicket, {data: dataDenyTicket}] = useMutation(DENY_TICKET);

	useEffect(() => {
		if (vehiclesQueryData) {
			updateVehicles(vehiclesQueryData.vehicles);
		}
	}, [vehiclesQueryData]);

	const approveTicket = (vehicle, ticket) => {
			_approveTicket({variables: {ticket, vehicle}});
	};

	const denyTicket = (vehicle, ticket) => {
			_denyTicket({variables: {ticket, vehicle}});
	};

	const reloadData = () => {
		refetchVehicles();
	};

	useEffect(() => {
			reloadData();
	}, [dataApproveTicket, dataDenyTicket]);

	const obj = {
		vehicles,
		reloadData,
		approveTicket,
		denyTicket
	};

	return (
		<VehicleContext.Provider value={obj}>{children}</VehicleContext.Provider>
	);
};

export default VehicleContextProvider;
