import { Router } from 'express';

import email from '../lib/email';

const router = Router();

router.route('/:id').post(email.send);

export default router;
