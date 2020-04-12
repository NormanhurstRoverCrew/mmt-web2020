import express from 'express';
import path from 'path';
import cors from 'cors';
import * as Sentry from '@sentry/node';

import axios from 'axios';
import fs from 'fs';
import _ from 'underscore';

import api from './api';

import jwtCheck from './lib/auth/jwt';

Sentry.init({ dsn: 'https://b9b82efea5c44dcea771e159b491664f@sentry.io/1483117' });

axios.defaults.url = 'http://backend:3000/api/';

const PORT = 3000;
let IP;
const http = require('http');

const options = {
	host: 'ipv4bot.whatismyipaddress.com',
	port: 80,
	path: '/',
};

http.get(options, (res) => {
	res.on('data', (chunk) => {
		IP = `${chunk}`;
	});
}).on('error', (e) => {
	console.log(`error: ${e.message}`);
});

const app = express();

// must be first
app.use(Sentry.Handlers.requestHandler());

// AUTH0//////////
app.use('/api', jwtCheck);

// Print Method and route. eg. POST /api/some/path
app.use((req, res, next) => {
	// eslint-disable-next-line no-console
	const ip = req.headers['x-real-ip'];
	if (ip === IP) return;
	console.log(`${ip}\t${req.method}\t${req.originalUrl}`);
	next();
});


// If we do not get the correct credentials, we’ll return an appropriate message
app.use((err, req, res, next) => {
	if (err.name === 'UnauthorizedError') {
		console.log('AUTH ERROR: Missing or invalid Token', err);
		res.status(401).json({ message: 'Missing or invalid token' });
	} else {
		console.log(err);
	}
});


app.use(cors());

// to parse json content from the client in the body
app.use(express.json());

app.use(express.static('dist'));
// /////////

app.use('/api', api);

app.use(Sentry.Handlers.errorHandler());

// Optional fallthrough error handler
app.use((err, req, res, next) => {
	// The error id is attached to `res.sentry` to be returned
	// and optionally displayed to the user for support.
	res.statusCode = 500;
	res.end(`${res.sentry}\n`);
});

const distPath = path.join(__dirname, '..', 'dist-client');

if (fs.existsSync(distPath)) {
	console.log('Express serving prod files');
	let items = fs.readdirSync(distPath);
	items = _.without(items, 'index.html');
	items.forEach((file) => {
		app.get(`*/${file}`, (req, res) => {
			res.set('Cache-Control', 'public, max-age=86400'); // cach for 24 hours
			res.sendFile(path.join(distPath, file));
		});
	});

	app.get('*', (req, res) => {
		res.sendFile(path.join(__dirname, '..', 'dist-client', 'index.html'));
	});
} else {
	console.log('Express did not find pre built bundles. Assuming were in dev?');
}

// eslint-disable-next-line no-console
app.listen(PORT, () => console.log('Express Server started!'));
