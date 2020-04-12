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

import Name from 'components/input/Name';

let component;
let instance;

describe("<Name/>", () => {
	let someValue;
	let fake;
	beforeEach(async () => {
		fake = sinon.fake();

		someValue = "Abe Lincoln"

		component = mount(
			<Name value={someValue}
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

	it("should set initial value", () => {
		expect(component.find("Input").props().value).to.equal(someValue)
	})

	it("should change the value when a key is pressed", async () => {
		expect(component.find("Input").props().value).to.equal(someValue)

		someValue = "Abraham Lincoln";

		component.find("input").simulate('change', { target: { value: someValue } });

		expect(fake.callCount).to.equal(1)
	})

	it("should provide an error message if the mobile is too short", () => {
		let invalidName = "Abe";
		component.find("input").simulate('change', { target: { value: invalidName } });
		expect(component.html()).contains("Please type your Full Name");
	})
	
	it ("should not display an error if the Name is Valid", () => {
		let invalidMobile = "Abraham Lincoln";
		component.find("input").simulate('change', { target: { value: invalidMobile } });

		expect(component.html()).does.not.contains("Please type your Full Name");
	})

	it("should allow names with apostrophies", () => {
		let name = "Someone Mc'Donald";
		component.find("input").simulate('change', { target: { value: name } });

		expect(component.html()).does.not.contains("Please type your Full Name");
		assert(fake.calledOnce)
		expect(fake.args[0][1]).to.equal(name)
	})

	it("should allow names with dashes", () => {
		let name = "Someone Maree-Obrien";
		component.find("input").simulate('change', { target: { value: name } });

		expect(component.html()).does.not.contains("Please type your Full Name");
		assert(fake.calledOnce)
		expect(fake.args[0][1]).to.equal(name)
	})

	it("should allow 2 or more names", () => {
		let name = "Someone One Two Three Four";
		component.find("input").simulate('change', { target: { value: name } });

		expect(component.html()).does.not.contains("Please type your Full Name");
		assert(fake.calledOnce)
	})
});