import init, { wasm_main } from "./pkg/webcam_mouse.js";

async function run() {
	await init();
	wasm_main();
}

run();
