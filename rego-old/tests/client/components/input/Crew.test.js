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

import CREWS from "rego/crews";
import Crew from 'components/input/Crew';

let component;
let instance;

describe("<Crew/>", () => {
	let someValue;
	let fake;
	beforeEach(async () => {
		someValue = CREWS[Math.floor(Math.random() * CREWS.length)];
		fake = sinon.fake();
		component = mount(
			<Crew value={someValue}
				onChange={fake}
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

	it("should set the select to the value", () => {
		expect(component.find("select").props().value).to.equal(someValue)
	})

	it("should change the value when a new crew is selected", async () => {
		expect(component.find("select").props().value).to.equal(someValue)

		// generate a new crew
		someValue = CREWS[Math.floor(Math.random() * CREWS.length)];

		component.find("select").simulate('change', { target: { value: someValue } });

		assert(fake.calledOnce)
	})

	it("should provide an error message if the crew input is invalid", () => {
		let invalidCrew = "SomeInvalidCrew";
		component.find("select").simulate('change', { target: { value: invalidCrew } });

		expect(component.html()).contains(invalidCrew + " is not a valid NSW Crew")
	})
});