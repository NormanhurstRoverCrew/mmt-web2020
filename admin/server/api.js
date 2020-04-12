import _ from 'underscore';
import { Router } from 'express';
import bookings from './api/bookings';
import teams from './api/teams';
import tickets from './api/tickets';
import email from './api/email';
import stripe from './api/stripe';
import payments from './api/payments';
import activities from './api/activities';

const router = Router();

router.use((req, res, next) => {
	const { permissions } = req.user;
	if (_.contains(permissions, 'enabled')) {
		next();
	} else {
		res.send({ error: 'Unauthorized: Please contact the webmaster if you believe you should have access.' });
	}
});

router.use('/bookings', bookings);
router.use('/tickets', tickets);
router.use('/email', email);
router.use('/stripe', stripe);
router.use('/payments', payments);
router.use('/teams', teams);
router.use('/activities', activities);
router.get('/isadmin', (req, res) => {
	res.send(req.user);
});

export default router;
