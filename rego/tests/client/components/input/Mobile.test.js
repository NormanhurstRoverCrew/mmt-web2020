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

import Mobile from 'components/input/Mobile';

let component;
let instance;

describe("<Mobile/>", () => {
	let someValue;
	let fake;
	beforeEach(async () => {
		fake = sinon.fake();

		someValue = ""

		component = mount(
			<Mobile value={someValue}
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

		someValue = "0400 000 00";

		component.find("input").simulate('change', { target: { value: someValue } });

		assert(fake.calledOnce)
	})

	it("should provide an error message if the mobile is too short", () => {
		let invalidMobile = "04";
		component.find("input").simulate('change', { target: { value: invalidMobile } });
		expect(component.html()).contains("Mobile number is too short");

		invalidMobile = "040";
		component.find("input").simulate('change', { target: { value: invalidMobile } });
		expect(component.html()).contains("Mobile number is too short");
		
		invalidMobile = "0400 000";
		component.find("input").simulate('change', { target: { value: invalidMobile } });
		expect(component.html()).contains("Mobile number is too short");

		invalidMobile = "0400 000 00";
		component.find("input").simulate('change', { target: { value: invalidMobile } });
		expect(component.html()).contains("Mobile number is too short");
	})
	
	it("should provide an error message if the mobile is too long", () => {
		let invalidMobile = "0400 000 000 000";
		component.find("input").simulate('change', { target: { value: invalidMobile } });
		
		expect(component.html()).contains("Mobile number is too long");
	})
	
	it ("should not display an error if the mobile is valid", () => {
		let invalidMobile = "0400 000 000";
		component.find("input").simulate('change', { target: { value: invalidMobile } });

		expect(component.html()).does.not.contains("Mobile number is too short");
		expect(component.html()).does.not.contains("Mobile number is too long");
	})
});