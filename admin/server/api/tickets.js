import { Router } from 'express';

import tickets from '../lib/tickets';

const router = Router();

router.route('/:uid').patch(tickets.update);
router.route('/:uid').delete(tickets.delete);
router.route('/').post(tickets.newTicket);

export default router;
