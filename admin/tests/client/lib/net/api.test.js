import chai, { assert, expect } from 'chai';
import axios from 'axios';
import moxios from 'moxios';
import fakes from 'tests/client/fakes';
import configureStore from 'redux-mock-store'
import thunk from "redux-thunk";
import { initAPI } from 'lib/net/api';
import sinon from 'sinon';
var should = chai.should();
import _ from 'underscore';
import { push } from "connected-react-router";

import { auth0Actions } from 'actions/';

const mockStore = configureStore([thunk]);

let store;

describe("lib/net/api", () => {
	describe("initAPI()", () => {
		let window;
		beforeEach(async () => {
			window = {
				location: {
					href: 'https://localhost:3000/'
				}
			}

			initAPI(window)
		})

		afterEach(function () {

		})
		it("should set the default baseURL to the current href", () => {
			expect(axios.defaults.baseURL).to.equal(window.location.href + 'api')
		})
	})
});