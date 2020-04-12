import jwt from 'express-jwt';
import jwks from 'jwks-rsa';

const jwtCheck = jwt({
	secret: jwks.expressJwtSecret({
		cache: true,
		rateLimit: true,
		jwksRequestsPerMinute: 5,
		jwksUri: 'https://normorovers.au.auth0.com/.well-known/jwks.json',
	}),
	issuer: 'https://normorovers.au.auth0.com/',
	audience: [
		'http://localhost:8081/',
		'http://localhost:3001/',
		'https://admin.mmt.***REMOVED***/',
		'https://event.mmt.***REMOVED***/',
		'https://normorovers.au.auth0.com/userinfo',
	],
	algorithms: ['RS256'],
});

export default jwtCheck;
