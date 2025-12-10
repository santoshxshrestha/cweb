# C Online Compiler - WebAssembly

A WebAssembly-based C code compiler/interpreter that runs directly in the browser.

## Features

- Compile and execute C code in the browser using WebAssembly
- Simple C interpreter supporting:
  - Variable declarations (int)
  - Arithmetic operations (+, -, *, /)
  - printf statements with %d format specifier
  - Basic control flow
- Fast execution without server-side compilation
- Clean JSON API for easy integration

## Prerequisites

Before building this project, you need to install:

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack** - Build tool for WebAssembly
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

## Building the Project

1. Build the WASM module:
   ```bash
   wasm-pack build --target web
   ```

   This will create a `pkg` directory with:
   - `c_compiler_wasm_bg.wasm` - The WebAssembly binary
   - `c_compiler_wasm.js` - JavaScript bindings
   - `c_compiler_wasm.d.ts` - TypeScript definitions

2. Serve the project locally:
   ```bash
   # Using Python
   python3 -m http.server 8000
   
   # OR using Node.js
   npx http-server
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8000
   ```

## Usage from JavaScript

```javascript
import init, { compile_and_run_c } from './pkg/c_compiler_wasm.js';

// Initialize the WASM module
await init();

// Compile and run C code
const cCode = `
    #include <stdio.h>
    int main() {
        printf("Hello from C!\\n");
        return 0;
    }
`;

const resultJson = compile_and_run_c(cCode);
const result = JSON.parse(resultJson);

if (result.success) {
    console.log("Output:", result.output);
} else {
    console.error("Error:", result.error);
}
```

## API Reference

### `compile_and_run_c(code: string): string`

Compiles and executes the provided C code.

**Parameters:**
- `code` - String containing C source code

**Returns:**
- JSON string with the following structure:
  ```json
  {
    "success": true,
    "output": "Program output here",
    "error": null
  }
  ```
  Or on error:
  ```json
  {
    "success": false,
    "output": "",
    "error": "Error message here"
  }
  ```

## Supported C Features

Currently supports a subset of C:

- **Data Types:** int
- **Operators:** +, -, *, /
- **Functions:** printf (with %d, %i format specifiers)
- **Control:** Basic main function execution

### Example Programs

**Hello World:**
```c
#include <stdio.h>
int main() {
    printf("Hello, World!\n");
    return 0;
}
```

**Variables and Arithmetic:**
```c
#include <stdio.h>
int main() {
    int a = 10;
    int b = 20;
    int sum = a + b;
    printf("Sum: %d\n", sum);
    return 0;
}
```

## Development

### Project Structure

```
wasm/
├── src/
│   └── lib.rs          # Main WASM module with C interpreter
├── Cargo.toml          # Rust dependencies
├── index.html          # Demo web interface
└── README.md           # This file
```

### Running Tests

```bash
cargo test
```

### Development Build

For faster compilation during development:
```bash
wasm-pack build --target web --dev
```

### Production Build

For optimized production build:
```bash
wasm-pack build --target web --release
```

## Limitations

This is a simplified C interpreter designed for educational purposes. It does not support:
- Arrays and pointers
- Loops (for, while, do-while)
- Conditional statements (if-else, switch)
- Multiple functions
- Standard library functions beyond basic printf
- File I/O
- Memory management (malloc, free)
- Structures and unions
- Preprocessor directives (beyond ignoring #include)

For a full-featured C compiler in the browser, consider alternatives like:
- Emscripten
- WebAssembly System Interface (WASI) with clang
- Remote compilation services

## Future Enhancements

Planned features:
- [ ] Support for loops (for, while)
- [ ] Conditional statements (if-else)
- [ ] Arrays
- [ ] More printf format specifiers (%f, %s, %c)
- [ ] Multiple functions
- [ ] Better error messages with line numbers
- [ ] Syntax highlighting in the web interface
- [ ] Code completion

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
