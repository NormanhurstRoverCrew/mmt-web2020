import { createMemoryHistory } from 'history';
import initStore from 'store/initStore';
import initialState from 'store/initialState';
import {mocks as mock_browser} from 'mock-browser';

const mockBrowser = new mock_browser.MockBrowser();

const history = createMemoryHistory('/');

const store = initStore(history, initialState)

const browser = {
    window: mockBrowser.getWindow(),
    localStorage: mockBrowser.getLocalStorage(),
    history: history,
    location: mockBrowser.getLocation(),
}

export default {history, store, browser}