import initSync from "./gen/background.js";

// run the wasm initializer before calling wasm methods
// the initializer is generated by wasm_pack
initSync();
