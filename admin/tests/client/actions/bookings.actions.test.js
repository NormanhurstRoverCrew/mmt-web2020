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

import { bookingsActions } from 'actions/';

const mockStore = configureStore([thunk]);

let store;

describe("bookings actions", () => {
	beforeEach(async () => {
		moxios.install()

		initAPI(fakes.browser.window)

		store = mockStore({
			bookings: []
		});
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		moxios.uninstall(axios);
	})

	describe("pull()", () => {
		beforeEach(() => {
			store = mockStore({})
		})

		afterEach(() => {
		})

		it("should GET backend/bookings", () => {
			store.dispatch(bookingsActions.pull())

			moxios.wait(() => {
				let fakeBookings = [
					{},
					{}
				]
				let request = moxios.requests.mostRecent();

				request.respondWith({
					status: 200,
					response: fakeBookings,
				}).then(() => {
					expect(store.getActions()[0].bookings).to.eql(fakeBookings)
				}).catch((err) => {
					assert(false, "error of some sort " + err.message)
				})
			})
		})
	})

	describe("update()", () => {

		let fakeBookings = [
			{},
			{}
		]
		beforeEach(() => {
			store = mockStore({})
			store.dispatch(bookingsActions.update(fakeBookings))
		})

		afterEach(() => {
		})

		it("should dispatch BOOKINGS:UPDATE", () => {
			expect(store.getActions()[0].type).to.eql("BOOKINGS:UPDATE")
		})

		it("should pass bookings as a property of the action", () => {
			expect(store.getActions()[0].bookings).to.eql(fakeBookings)
		})
	})
});