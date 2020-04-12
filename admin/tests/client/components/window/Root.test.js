import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow } from 'enzyme';

import React from "react";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { initAPI } from 'lib/net/api';

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { Root } from "components/window/";

let component;
let instance;

describe("component window/<Root />", () => {
    beforeEach(async () => {
        moxios.install()
        initAPI(fakes.browser.window);

        const store = mockStore({});

        component = shallow(<Root />);
				
        await component.update();
        instance = component.instance()
    })

    afterEach(function () {
        // import and pass your custom axios instance to this method
        moxios.uninstall()
        component.unmount();
		})
		
		it("should do some things")

    
});