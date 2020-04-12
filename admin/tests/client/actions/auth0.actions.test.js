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
import { push } from "connected-react-router";

import { auth0Actions } from 'actions/';

const mockStore = configureStore([thunk]);

let store;

describe("auth0 actions", () => {
	beforeEach(async () => {
		moxios.install()

		initAPI(fakes.browser.window)

		store = mockStore({
			auth: {
				access_token: "something",
				expires_at: 1225234234
			}
		});
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		moxios.uninstall(axios);
	})

	describe("createAuth0", () => {
		let window = {location: {href: "https://somedomain.com/something/else.html"}}
		let WebAuth;
		before(() => {
			WebAuth = auth0Actions.createAuth0(window)
		})

		it("returns an auth0.WebAuth", () => {
			expect(WebAuth.constructor.name).to.eql("WebAuth")
		})

		describe("WebAuth", () => {
			it("redirectUri => https://somedomain.com/callback", () => {
				expect(WebAuth.baseOptions.redirectUri).to.be.equal("https://somedomain.com/callback")
			})

			it("audience => https://somedomain.com/", () => {
					expect(WebAuth.baseOptions.audience).to.be.equal("https://somedomain.com/")
			})

			describe("scopes", () => {
				let scopes = [
					'openid',
					'profile',
					'full',
				]

				let WebAuthScopes;

				before(() => {
					WebAuthScopes = WebAuth.baseOptions.scope.split(' ')
				})

				scopes.forEach((scope) => {
					it("'" + scope + "' should be present", () => {
						expect(WebAuthScopes).to.contain(scope, "the WebAuth object is missing the '" + scope + "' scope" );
					})
				})
			})
		})
	})

	describe("login()", () => {
		let createAuth0;
		let authorize;

		beforeEach(() => {
			store = mockStore({})
			authorize = sinon.spy()
			createAuth0 = sinon.stub(auth0Actions, 'createAuth0')
			createAuth0.withArgs(fakes.browser.window).returns({authorize})
		})

		afterEach(() => {
			createAuth0.restore()
		})

		it ("should call WebAuth.authorize() once", () => {
			store.dispatch(auth0Actions.login(fakes.browser.window))
			assert(authorize.calledOnce)
		})
	})

	describe("logout()", () => {
		beforeEach(() => {
			sinon.stub(auth0Actions, 'login').returns({ type: "FAKE_LOGIN" })
			store = mockStore({});
		})

		afterEach(() => {
			auth0Actions.login.restore()
		})

		it("should empty the auth state(AUTH:LOGOUT)", () => {

			store.dispatch(auth0Actions.logout(fakes.browser.localStorage))
			const actions = store.getActions();

			expect(actions[0]).to.be.eql({ type: "AUTH:LOGOUT" })
		})

		it("should call the auth0Actions.login() method once", () => {
			store.dispatch(auth0Actions.logout(fakes.browser.localStorage))

			assert(auth0Actions.login.calledOnce)

			const actions = store.getActions();

			expect(actions[1]).to.be.eql({ type: "FAKE_LOGIN" })
		})

		it("should remove all auth items from localStorage", () => {
			fakes.browser.window.location.href = "https://admin.mmt.***REMOVED***/";
			sinon.spy(fakes.browser.localStorage, 'removeItem');

			store.dispatch(auth0Actions.logout(fakes.browser.localStorage))

			const removedItems = _.map(fakes.browser.localStorage.removeItem.args, (item) => {
				return item[0];
			})

			expect(removedItems).to.have.members([
				'access_token',
				'id_token',
				'expires_at',
				'name',
				'avatar',
				'scopes',
				'email'
			])

			fakes.browser.localStorage.removeItem.restore();
		})
	});

	describe("loadState", () => {
		let getItem;

		beforeEach(() => {
			sinon.stub(auth0Actions, 'login').returns({ type: "FAKE_LOGIN" })
			getItem = sinon.stub(fakes.browser.localStorage, 'getItem');
			store = mockStore({});
		})

		afterEach(() => {
			auth0Actions.login.restore()
		})

		describe("when localStorage does not contain any auth items", () => {
			beforeEach(() => {
				getItem.returns(null);
			});

			afterEach(() => {
				getItem.restore()
			});

			it("should dispatch the login method", () => {

				store.dispatch(auth0Actions.loadState(fakes.browser.window, fakes.browser.localStorage))
				const actions = store.getActions();

				expect(actions[0]).to.be.eql({ type: "FAKE_LOGIN" })
			})

		});

		describe("when localStorage contains items", () => {
			beforeEach(() => {
				store = mockStore({});

				getItem.withArgs('access_token').returns('someaccesstoken');
				getItem.withArgs('id_token').returns('someidtoken');
				getItem.withArgs('name').returns('somename');
				getItem.withArgs('avatar').returns('linktoanavatar');
				getItem.withArgs('scopes').returns('[\"profile\",\"admin\"]');
				getItem.withArgs('email').returns('joblo@gmail.com');
			});

			afterEach(() => {
				getItem.restore()
			});


			describe("expires_at < current time", () => {
				beforeEach(() => {
					store = mockStore({});
					let expiry = new Date().getTime() - 6000
					getItem.withArgs('expires_at').returns(expiry)
				});

				it("should dispatch the sign in reducer making the user authenticated", () => {

					store.dispatch(auth0Actions.loadState(fakes.browser.window, fakes.browser.localStorage))
					const actions = store.getActions();

					expect(actions[0]).to.be.eql({ type: "FAKE_LOGIN" })
				})
			});

			describe("expires_at > current time", () => {
				let expiry;

				beforeEach(() => {
					store = mockStore({});
					expiry = new Date().getTime() + 6000
					getItem.withArgs('expires_at').returns(expiry)
				});

				it("should dispatch the login method", () => {

					store.dispatch(auth0Actions.loadState(fakes.browser.window, fakes.browser.localStorage))
					const actions = store.getActions();

					expect(actions[0].type).to.be.eql("AUTH:LOGIN:LOCAL");
					expect(actions[0].auth).to.be.eql({
						access_token: "someaccesstoken",
						avatar: "linktoanavatar",
						email: "joblo@gmail.com",
						expires_at: expiry,
						id_token: "someidtoken",
						name: "somename",
						scopes: [
							"profile",
							"admin",
						]
					})
				})
			});
		});
	});

	describe("handleAuthentication", (() => {
		let createAuth0;
		let parseHash;

		beforeEach(() => {
			createAuth0 = sinon.stub(auth0Actions, 'createAuth0');
			sinon.stub(auth0Actions, 'setSession').returns({ type: "FAKE_SET_SESSION" })

			store = mockStore({});
		})

		afterEach(() => {
			auth0Actions.setSession.restore()
			createAuth0.restore();
		})

		describe("when a valid hash is parsed", () => {
			beforeEach(() => {
				parseHash = (callback) => {
					callback(null, {
						accessToken: "abcdefg",
						idToken: "efghijk"
					})
				}
				createAuth0.withArgs(fakes.browser.window).returns({ parseHash: parseHash })
			})

			afterEach(() => {

			})

			it("should dispatch setSession", () => {
				store.dispatch(auth0Actions.handleAuthentication(fakes.browser.window, fakes.browser.history, fakes.browser.localStorage))

				expect(store.getActions()).to.deep.include({ type: 'FAKE_SET_SESSION' })
			})

			it("should dispatch push route to '/'", () => {
				store.dispatch(auth0Actions.handleAuthentication(fakes.browser.window, fakes.browser.history, fakes.browser.localStorage))

				expect(store.getActions()).to.deep.include(push('/'))
			})
		})

		describe("when an invalid hash is parsed", () => {

			beforeEach(() => {
				parseHash = (callback) => {
					callback({ msg: "Error: Invalid hash" }, {})
				}
				createAuth0.withArgs(fakes.browser.window).returns({ parseHash: parseHash })
			})

			afterEach(() => {

			})

			it("should dispatch push an error to the store", () => {
				store.dispatch(auth0Actions.handleAuthentication(fakes.browser.window, fakes.browser.history, fakes.browser.localStorage))

				const errorAction = _.find(store.getActions(), (action) => action.type === "AUTH:LOGIN:ERROR")

				expect(errorAction.err.msg).to.contain("Error")
			})
		})
	}))

	describe("setSession", (() => {
		let createAuth0;
		let parseHash;

		let authResult = {
			accessToken: "TESTTOKEN",
			idToken: "",
			expiresAt: 0,
			idTokenPayload: {
				nickname: "Something",
				picture: "http://images.google.com/random",
				email: "joblo@gmail.com"
			},
			scopes: ['profile','all']
		}

		beforeEach(() => {
			store = mockStore({});
		})

		afterEach(() => {
		})

		it("should set axios.defaults.common Authorization", () => {
			store.dispatch(auth0Actions.setSession(authResult, fakes.browser.localStorage))
			expect(axios.defaults.headers.common.Authorization).to.contain("Bearer")
			expect(axios.defaults.headers.common.Authorization).to.contain("TESTTOKEN")
		})

		it("should dispatch AUTH:LOGIN with the authResult object", () => {
			store.dispatch(auth0Actions.setSession(authResult, fakes.browser.localStorage))

			const authLogin = _.find(store.getActions(), (action) => action.type === "AUTH:LOGIN")

			expect(authLogin)
			expect(authLogin.auth).to.equal(authResult)
		})

		describe("sets localStorage items", () => {
			beforeEach(() => {
				sinon.spy(fakes.browser.localStorage, 'setItem');
				store = mockStore({});

			})

			afterEach(() => {
				fakes.browser.localStorage.setItem.restore()
			})


			it("should set the auth items in localStorage", () => {
				store.dispatch(auth0Actions.setSession(authResult, fakes.browser.localStorage))

				const setItems = _.map(fakes.browser.localStorage.setItem.args, (item) => {
					return item[0];
				})
	
				expect(setItems).to.have.members([
					'access_token',
					'id_token',
					'expires_at',
					'name',
					'avatar',
					'scopes',
					'email'
				])
			})
		})
	}))
});