import chai, { assert, expect } from 'chai';
import axios from 'axios';
import moxios from 'moxios';
import fakes from 'tests/client/fakes';
import configureStore from 'redux-mock-store'
import thunk from "redux-thunk";
import sinon from 'sinon';
var should = chai.should();
import _ from 'underscore';

import {push} from "connected-react-router";

import { bookingActions } from 'actions/';

const mockStore = configureStore([thunk]);

let store;

describe("bookings actions", () => {
	beforeEach(async () => {
		moxios.install()

		store = mockStore({
			bookings: []
		});
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		moxios.uninstall(axios);
	})

	describe("updateUser()", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should dispatch the user object", () => {
            let user = {name:"abcd efg"}
			store.dispatch(bookingActions.updateUser(user))

            expect(store.getActions()[0].type).to.equal("BOOKING:USER:UPDATE")
            expect(store.getActions()[0].user).to.deep.equal(user)
		})
	})
    
    describe("setBooking()", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should dispatch the booking object", () => {
            let booking = {tickets: [], user: {}};
			store.dispatch(bookingActions.setBooking(booking))

            expect(store.getActions()[0].type).to.equal("BOOKING:SET")
            expect(store.getActions()[0].booking).to.deep.equal(booking)
		})
    })
    
    describe("setBookingUID()", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should dispatch the booking object", () => {
            let uid = "someUIDString"
			store.dispatch(bookingActions.setBookingUID(uid))

            expect(store.getActions()[0].type).to.equal("BOOKING:UID:SET")
            expect(store.getActions()[0].uid).to.equal(uid)
		})
    })
    
    describe("addTicket()", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should dispatch the booking object", () => {
			store.dispatch(bookingActions.addTicket())

            expect(store.getActions()[0].type).to.equal("BOOKING:TICKET:ADD")
		})
    })
    
    describe("updateTicket()", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should dispatch the booking object", () => {
            let index = Math.floor(Math.random()*100);
            let user = {name:"abcd efg"}
			store.dispatch(bookingActions.updateTicket(index, user))

            expect(store.getActions()[0].type).to.equal("BOOKING:TICKET:USER:UPDATE")
            expect(store.getActions()[0].user).to.deep.equal(user)
            expect(store.getActions()[0].index).to.equal(index)
		})
    })


    describe("removeTicket(index)", () => {
		beforeEach(() => {
            store = mockStore({})
		})

		afterEach(() => {
		})

		it("should not remove the ticket if index == 0", () => {
            let index = 0;
			store.dispatch(bookingActions.removeTicket(index))

            expect(store.getActions().length).to.equal(0)
        })
        
        it("should dispatch a remove ticket", () => {
            let index = Math.floor(Math.random()*100);
            index = (index > 0) ? index : 1; //make sure this is above 0
			store.dispatch(bookingActions.removeTicket(index))

            expect(store.getActions()[0].type).to.equal("BOOKING:TICKET:REMOVE")
            expect(store.getActions()[0].index).to.equal(index)
		})
    })
   
    
    describe("checkout() //TODO", () => {
		beforeEach(() => {
            store = mockStore({})
            moxios.install()
		})

		afterEach(() => {
            moxios.uninstall(axios)
		})

		it("should dispatch the booking object", () => {
			store.dispatch(bookingActions.checkout())

            expect(store.getActions()[0]).to.deep.equal(push("/checkout"))
		})
    })

    describe("createUser()", () => {
		beforeEach(() => {
            store = mockStore({
                steppers: {
                    home: 42
                },
                booking: {
                    user: {

                    }
                }
            })
		})

		afterEach(() => {
		})

		it("should setBooking once the repsonse comes back", () => {
            moxios.stubRequest("/bookings", {
                status: 200,
                response: {something: "mate"}
            })
            
            bookingActions.createUser()(store.dispatch, store.getState)

            moxios.wait(() => {
                console.log(store.getActions())


				let resp = {uid: "Something", user: {}}
				let request = moxios.requests.mostRecent();

				request.respondWith({
					status: 200,
					response: resp,
				}).then(() => {
                    expect(store.getActions()[0]).to.deep.equal(bookingActions.setBooking(resp))
                })
                // .catch((err) => {
				// 	assert(false, "error of some sort " + err.message)
				// })
			})
		})
    })
});