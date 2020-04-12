import axios from "axios";
// import {store} from "/client/index";

const initAPI = (window) => {
	const urlSplit = window.location.href.split("/");
	axios.defaults.transformResponse.push((data) => {
		if (data.error) {
			if (data.error.includes("Unauthorized:")) {
				console.log("You are Unauthorized...");
				localStorage.removeItem("access_token");
				localStorage.removeItem("expires_at");
				location.reload();
				return {
					unauthorized: true,
				};
			}
		}
		return data;
	});
	axios.defaults.baseURL = urlSplit[0] + "//" + urlSplit[2] + "/api";
};

export {initAPI};
