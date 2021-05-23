import init, { run_app } from "./pkg/rego.js";

window.stripe_pay_card = (stripe, card, client_secret, callback) => {
	stripe
		.confirmCardPayment(client_secret, {
				payment_method: {
					card: card
				}
		})
		.then(function(result) {
				console.log(result);
				callback(result);
		});
};

async function main() {
  await init("/pkg/rego_bg.wasm");
  run_app();
}
main();
