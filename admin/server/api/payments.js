import { Router } from 'express';

import payments from '../lib/payments';

const router = Router();

router.route('/').get(payments.index);


export default router;
