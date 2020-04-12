import { Router } from 'express';

import stripe from '../lib/stripe';

const router = Router();

router.route('/').get(stripe.index);
router.route('/charges').get(stripe.charges);

export default router;
