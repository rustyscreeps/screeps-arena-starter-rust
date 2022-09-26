import { initSync } from "./screeps-arena-starter-rust";
import wasm_bytes from "./screeps-arena-starter-rust_bg.wasm.bin";
initSync(wasm_bytes);
export * from "./screeps-arena-starter-rust";
