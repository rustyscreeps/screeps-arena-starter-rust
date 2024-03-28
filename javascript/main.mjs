import { initSync, wasm_loop } from "./screeps-arena-starter-rust";
import wasm_bytes from "./screeps-arena-starter-rust_bg.wasm.bin";
initSync(wasm_bytes);
export * from "./screeps-arena-starter-rust";

Error.stackTraceLimit = 100;

// This provides the function `console.error` that wasm_bindgen sometimes expects to exist,
// especially with type checks in debug mode. An alternative is to have this be `function () {}`
// and let the exception handler log the thrown JS exceptions, but there is some additional
// information that wasm_bindgen only passes here.
//
// There is nothing special about this function and it may also be used by any JS/Rust code as a convenience.
function console_error() {
    const processedArgs = Array.prototype.map.call(arguments, (arg) => {
        if (arg instanceof Error) {
            // On this version of Node, the `stack` property of errors contains
            // the message as well.
            return arg.stack;
        } else {
            return arg;
        }
    }).join(' ');
    console.log("ERROR:", processedArgs);
}

function loop () {
  // need to freshly override the fake console object each tick
  console.error = console_error;
  try {
      wasm_loop();
  } catch (error) {
      console.error("caught exception:", error);
      // we've already logged the more-descriptive stack trace from rust's panic_hook
      // if for some reason (like wasm init problems) you're not getting output from that
      // and need more information, uncomment the following:
      // if (error.stack) {
      //     console.error("stack trace:", error.stack);
      // }
  }
}

export { loop }
