import __wbg_init from "./gen/background.js";

// run the wasm initializer before calling wasm methods
// the initializer is generated by wasm_pack
(async () => {
  await __wbg_init();
})();
