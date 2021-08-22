import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow, mount } from 'enzyme';

import sinon from 'sinon'

import React from "react";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { MemoryRouter } from 'react-router'

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { styled as TicketBasicInput } from 'components/tickets/TicketBasicInput';

let component;
let instance;

describe("<TicketBasicInput/>", () => {
	let ticket;
	let fakeUpdateUser;
	beforeEach(async () => {
		ticket = {
			user: {
				name: "",
				mobile: "",
				email: "",
				crew: "",
			}
		}

		let store = mockStore({ticket})

		fakeUpdateUser = sinon.fake();

		component = mount(
			<TicketBasicInput
				ticket={ticket}
				remove={false}
				updateUser={fakeUpdateUser}
			/>
		);
		await component.update();
		instance = component.instance()

		// Get rid of the MemoryRouter wrap
		// component = component.children().first();        
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		component.unmount();
	})

	it("should update user.name", () => {
		console.log(component.debug())
		let val = "Abraham Lincoln";
		component.find("InputName").find("TextField").find("Input").simulate('change', { target: { value: val } });

		console.log(fakeUpdateUser.args[0])
		assert(fakeUpdateUser.calledOnce);
	})

});