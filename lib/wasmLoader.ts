// WASM Loader for C Compiler
// This module handles loading and initializing the WebAssembly C compiler

export interface CompilationResult {
  success: boolean;
  output: string;
  error?: string;
}

interface WasmModule {
  compile_and_run_c: (code: string) => string;
}

let wasmModule: WasmModule | null = null;
let isInitialized = false;

/**
 * Initialize the WASM module
 * This must be called before using the compiler
 */
export async function initWasm(): Promise<void> {
  if (isInitialized) {
    return;
  }

  try {
    // Dynamically load the WASM module
    const response = await fetch('/wasm/c_compiler_wasm_bg.wasm');
    const wasmBytes = await response.arrayBuffer();
    
    // Load the JavaScript glue code
    const scriptResponse = await fetch('/wasm/c_compiler_wasm.js');
    const scriptText = await scriptResponse.text();
    
    // Create a module from the script
    const blob = new Blob([scriptText], { type: 'application/javascript' });
    const scriptUrl = URL.createObjectURL(blob);
    
    const wasmInit = await import(/* @vite-ignore */ scriptUrl);
    
    // Initialize with the WASM bytes
    await wasmInit.default(wasmBytes);
    
    wasmModule = wasmInit;
    isInitialized = true;
    
    console.log('WASM module initialized successfully');
    
    // Clean up the blob URL
    URL.revokeObjectURL(scriptUrl);
  } catch (error) {
    console.error('Failed to initialize WASM module:', error);
    throw new Error(`WASM initialization failed: ${error}`);
  }
}

/**
 * Compile and run C code
 * @param code - The C source code to compile and execute
 * @returns CompilationResult object with success status, output, and any errors
 */
export async function compileAndRunC(code: string): Promise<CompilationResult> {
  if (!isInitialized || !wasmModule) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }

  try {
    // Call the WASM function
    const resultJson = wasmModule.compile_and_run_c(code);
    
    // Parse the JSON result
    const result: CompilationResult = JSON.parse(resultJson);
    
    return result;
  } catch (error) {
    console.error('Compilation error:', error);
    return {
      success: false,
      output: '',
      error: `Runtime error: ${error}`,
    };
  }
}

/**
 * Check if the WASM module is initialized
 */
export function isWasmInitialized(): boolean {
  return isInitialized;
}

/**
 * Reset the WASM module (useful for testing)
 */
export function resetWasm(): void {
  wasmModule = null;
  isInitialized = false;
}
