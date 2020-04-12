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

import {styled as STicketsByTimes, TicketsByTimes, mapStateToProps} from "components/data/TicketsByTime";
import countCreatedAt from "lib/data/countCreatedAt";

let component;
let instance;
let store;

describe("component data/<TicketsByTime />", () => {
    let title;
    let actionText;
    let fake;
    beforeEach(async () => {
        initAPI(fakes.browser.window);

        // component = mount(
        //     <TicketsByTimes/>
        // );

        // await component.update();
        // instance = component.instance()
    })

    afterEach(function () {
        // import and pass your custom axios instance to this method
        // component.unmount();
    })

    describe("something", () => {
        beforeEach(async () => {
            initAPI(fakes.browser.window);
    
            component = shallow(
                <STicketsByTimes/>
            );
    
            await component.update();
            instance = component.instance()
        })
    
        afterEach(function () {
            // import and pass your custom axios instance to this method
            component.unmount();
        })
        
        it("does not error", () => {

        })
    })

    describe("mapStateToProps.ticketsGraph()", () => {
        it("should return all times created at for tickets", () => {
            let state = {
                bookings: [
                    {
                        tickets: [
                            { created_at: new Date(2019, 1, 1) },
                            { created_at: new Date(2019, 1, 2) },
                            { created_at: new Date(2019, 1, 5) },
                        ]
                    }
                ]
            }
            let stateProps = mapStateToProps(state)
            let output = stateProps.ticketsGraph;

            expect(output).to.deep.equal(countCreatedAt(state.bookings[0].tickets))
        })
    })
});