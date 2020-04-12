import { Router } from 'express';

import teams from '../lib/teams';

const router = Router();

router.route('/').get(teams.index);
router.route('/:id').patch(teams.update);
router.route('/:id/ticket').post(teams.addTicket);


export default router;
