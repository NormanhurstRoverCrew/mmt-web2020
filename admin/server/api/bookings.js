import { Router } from 'express';

import bookings from '../lib/bookings';
import stripe from '../lib/stripe';

const router = Router();

router.route('/').get(bookings.index);
router.route('/:uid').delete(bookings.delete);
router.route('/:uid/charges').get(stripe.bookingCharges);
router.route('/:uid/payments').post(bookings.addPayment);


export default router;
