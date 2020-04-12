import { Router } from 'express';
import HelloWorld from './lib/hello_world';

const router = Router();

const hw = new HelloWorld(true);

console.log(hw, hw.long);

// const bookings = require('./api/bookings.js')
// router.use("/bookings", bookings)
router.get('/hello', hw.index);

export default router;
