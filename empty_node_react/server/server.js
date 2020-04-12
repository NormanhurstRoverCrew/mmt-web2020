import express from 'express';
import path from 'path';
import cors from 'cors';

import api from './api';

const PORT = 3000;

const app = express();

// Print Method and route. eg. POST /api/some/path
app.use((req, res, next) => {
	// eslint-disable-next-line no-console
	console.log(req.method, req.originalUrl);
	next();
});

app.use(cors());

app.use(express.static('dist'));
// /////////

app.use('/api', api);

// For distributing the bundle once it is built
app.get('*', (req, res) => {
	res.sendFile(path.join(__dirname, '..', 'dist', 'index.html'));
});

// eslint-disable-next-line no-console
app.listen(PORT, () => console.log('Express Server started!'));
