class HelloWorld {
	constructor(long) {
		this.long = long;

		this.index = this.index.bind(this);
	}

	index(req, res) {
		if (this.long) {
			res.send({ msg: 'Hello, World from the express API' });
		} else {
			res.send({ msg: 'Hello, World!' });
		}
	}
}
export default HelloWorld;
