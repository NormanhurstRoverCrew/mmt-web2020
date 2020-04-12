import chai, { assert, expect } from 'chai';
var should = chai.should();

import React from "react";
import { create } from "react-test-renderer";
import moxios from 'moxios';

import HelloWorld from "components/HelloWorld"

let component;
let instance;

describe("HelloWorld component", () => {
    beforeEach(function () {
        moxios.install()

        component = create(
            <HelloWorld />
        );
        instance = component.getInstance();
    })

    afterEach(function () {
        // import and pass your custom axios instance to this method
        moxios.uninstall()
        component.unmount();
    })

    it("should show H1 'Hello, World!'", async () => {

        expect(component.root.findAll(element => element.type === "h1").length).to.equal(1, "There is no H1 Tag in HelloWorld")
        const h1 = component.root.find(element => element.type === "h1");
        expect(h1.props.children.join('')).to.equal("Hello, World!")
    });

    it("should show a link to an API response", async () => {
        // use instance to fire class methods forcefully...
        // await instance.handleClick();

        expect(component.root.findAll(e => e.type === "a")).lengthOf(1, "No link");
        expect(component.root.find(e => e.type === "a").props.href).to.contain("/api/")
    })
});