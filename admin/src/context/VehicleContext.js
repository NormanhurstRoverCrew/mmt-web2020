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

const VehicleContextProvider = ({children}) => {
	const [vehicles, updateVehicles] = useState([]);

	const {data: vehiclesQueryData, refetch: refetchVehicles} = useQuery(
		GET_VEHICLE,
	);

	useEffect(() => {
		if (vehiclesQueryData) {
			updateVehicles(vehiclesQueryData.vehicles);
		}
	}, [vehiclesQueryData]);

	const obj = {
		vehicles,
	};

	return (
		<VehicleContext.Provider value={obj}>{children}</VehicleContext.Provider>
	);
};

export default VehicleContextProvider;
