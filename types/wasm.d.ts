// Type declarations for the WASM module

declare module '/wasm/c_compiler_wasm.js' {
  export function compile_and_run_c(code: string): string;
  export default function init(path: string): Promise<void>;
  export function initSync(module: WebAssembly.Module): void;
}
