import init, { run_app } from '../../src/view/pkg/crab.js';
async function main() {
   await init('/pkg/crab_bg.wasm');
   run_app();
}
main()
