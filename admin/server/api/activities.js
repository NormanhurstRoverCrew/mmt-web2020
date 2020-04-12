import { Router } from 'express';

import activities from '../lib/activities';

const router = Router();

router.route('/').get(activities.index);


export default router;
