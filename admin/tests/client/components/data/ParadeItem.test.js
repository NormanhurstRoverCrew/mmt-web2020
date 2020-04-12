import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow, mount } from 'enzyme';
import sinon from 'sinon';

import React from "react";
import { Provider } from "react-redux";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { initAPI } from 'lib/net/api';

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import ParadeItem from "components/data/ParadeItem";

let component;
let instance;
let store;

describe("component data/<ParadeItem />", () => {
    let title;
    let actionText;
    let fake;
    beforeEach(async () => {
        initAPI(fakes.browser.window);

        title = "some title"
        actionText = "some action text"
        fake = sinon.fake()

        component = mount(
            <ParadeItem
                title={title}
                actionText={actionText}
                actionClick={fake}
            >
                Something
            </ParadeItem>
        );

        await component.update();
        instance = component.instance()
    })

    afterEach(function () {
        // import and pass your custom axios instance to this method
        component.unmount();
    })

    it("should render the title", () => {
        // console.log(component.debug())
        expect(component.find("Typography#title").text()).to.equal(title);
    })
    
    it("should render the action text", () => {
        expect(component.find("CardActions#action-button").text()).to.equal(actionText);  
    })
    
    it("should call props.actionClick", () => {
        component.find("CardActions#action-button").find("Button").simulate("click")
        expect(fake.calledOnce)
    })
});