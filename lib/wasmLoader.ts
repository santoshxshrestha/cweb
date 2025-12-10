// WASM Loader for C Compiler
// This module handles loading and initializing the WebAssembly C compiler

export interface CompilationResult {
  success: boolean;
  output: string;
  error?: string;
  needs_input?: string;  // Prompt for input if needed
  state?: string;  // Internal state (for resuming)
}

let isInitialized = false;
let initPromise: Promise<void> | null = null;
let wasmBindgen: any = null;

/**
 * Initialize the WASM module
 * This must be called before using the compiler
 */
export async function initWasm(): Promise<void> {
  // Return existing initialization promise if already initializing
  if (initPromise) {
    return initPromise;
  }
  
  if (isInitialized) {
    return;
  }

  initPromise = (async () => {
    try {
      // Load the WASM JavaScript glue code as a script
      const script = document.createElement('script');
      script.type = 'module';
      script.textContent = `
        import init, * as wasm from '/wasm/c_compiler_wasm.js';
        await init('/wasm/c_compiler_wasm_bg.wasm');
        window.__wasm_c_compiler = wasm;
      `;
      
      // Wait for script to execute
      await new Promise<void>((resolve, reject) => {
        script.onload = () => setTimeout(resolve, 100); // Small delay to ensure init completes
        script.onerror = (e) => reject(new Error('Failed to load WASM script'));
        document.head.appendChild(script);
        // For inline scripts, onload doesn't fire, so we resolve after a timeout
        setTimeout(resolve, 500);
      });

      wasmBindgen = (window as any).__wasm_c_compiler;
      
      if (!wasmBindgen || !wasmBindgen.compile_and_run_c) {
        throw new Error('WASM module not properly initialized');
      }
      
      isInitialized = true;
      console.log('WASM module initialized successfully');
    } catch (error) {
      console.error('Failed to initialize WASM module:', error);
      initPromise = null; // Reset so we can retry
      throw new Error(`WASM initialization failed: ${error}`);
    }
  })();

  return initPromise;
}

/**
 * Compile and run C code
 * @param code - The C source code to compile and execute
 * @returns CompilationResult object with success status, output, and any errors
 */
export async function compileAndRunC(code: string): Promise<CompilationResult> {
  if (!isInitialized || !wasmBindgen) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }

  try {
    // Call the WASM function
    const resultJson = wasmBindgen.compile_and_run_c(code);
    
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
 * Provide input to a waiting program
 * @param input - The user input string
 * @returns CompilationResult with program continuation or completion
 */
export async function provideInput(input: string): Promise<CompilationResult> {
  if (!isInitialized || !wasmBindgen) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }

  try {
    // Call the WASM provide_input function
    const resultJson = wasmBindgen.provide_input(input);
    
    // Parse the JSON result
    const result: CompilationResult = JSON.parse(resultJson);
    
    return result;
  } catch (error) {
    console.error('Input error:', error);
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
  wasmBindgen = null;
  isInitialized = false;
  initPromise = null;
}
