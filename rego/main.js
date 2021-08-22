import init, { run_app } from "./pkg/rego.js";

async function main() {
  await init("/pkg/rego_bg.wasm");
  run_app();
}
main();
