# WebC

A modern online C compiler that runs entirely in your browser using WebAssembly. Write, compile, and execute C programs without any backend server.

## Features

- **In-Browser Compilation**: Compile and run C code directly in your browser using WebAssembly
- **Real-Time Editing**: Code editor with syntax highlighting for C programming
- **8 Editor Themes**: Choose from Monokai, GitHub Light, Dracula, Tomorrow Night, Solarized Dark/Light, Terminal, and Twilight
- **Vim Mode**: Optional Vim keybindings for power users
- **Auto-Completion**: IntelliSense-style autocomplete for C keywords and standard library functions
- **Resizable Panels**: Drag the resize handle to adjust editor/output panel sizes
- **Fullscreen Mode**: Distraction-free coding experience
- **Toggle Output Panel**: Hide output panel for maximum code editing space
- **Zero Backend**: Everything runs client-side - no server required

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Rust and wasm-pack (for building the WASM module)

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd compiler
```

2. Install dependencies:
```bash
npm install
```

3. Build the WASM module:
```bash
./build-wasm.sh
```

Or manually:
```bash
cd wasm
wasm-pack build --target web --release
cp pkg/c_compiler_wasm_bg.wasm ../public/wasm/
cp pkg/c_compiler_wasm.js ../public/wasm/
cd ..
```

4. Run the development server:
```bash
npm run dev
```

5. Open [http://localhost:3000](http://localhost:3000) in your browser

## Project Structure

```
compiler/
├── app/                    # Next.js app directory
│   ├── page.tsx           # Main compiler page
│   └── layout.tsx         # Root layout
├── components/            # React components
│   └── CodeEditor.tsx     # Code editor component (Ace Editor)
├── lib/                   # Utility libraries
│   └── wasmLoader.ts      # WASM module loader
├── wasm/                  # Rust WASM compiler
│   ├── src/
│   │   └── lib.rs        # C interpreter/compiler logic
│   ├── Cargo.toml        # Rust dependencies
│   └── build.sh          # WASM build script
├── public/               # Static files
│   └── wasm/            # Compiled WASM files
└── build-wasm.sh        # Build script for WASM module
```

## How It Works

1. **Rust/WASM Backend**: The C compiler is written in Rust and compiled to WebAssembly. It provides a simple C interpreter that can execute basic C programs.

2. **Next.js Frontend**: The UI is built with Next.js and React, featuring a code editor (Ace Editor) and output panel.

3. **Runtime Execution**: When you click "Run Code", the C source code is passed to the WASM module, which parses and executes it, then returns the output.

## Supported C Features

The C interpreter now supports a comprehensive set of features:

### Data Types
- `int` - Integer variables
- `float` / `double` - Floating-point numbers
- `char` - Character variables

### Control Flow
- `if` statements - Conditional execution
- `for` loops - Definite iteration with initialization, condition, and increment
- `while` loops - Indefinite iteration

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%` (modulo)
- Comparison: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Increment/Decrement: `++`, `--`

### Standard Library Functions

#### Input/Output (stdio.h)
- `printf()` - Formatted output with specifiers: `%d`, `%i`, `%f`, `%lf`, `%c`, `%s`, `%ld`, `%u`
- `puts()` - Print string with newline
- `scanf()` - Input handling (basic support)

#### String Functions (string.h)
- `strlen()` - Get string length

#### Math Functions (math.h)
- `sqrt()` - Square root
- `pow()` - Power function
- `abs()` - Absolute value
- `sin()`, `cos()`, `tan()` - Trigonometric functions (basic support)

### Example Programs

**Hello World with Variables:**
```c
#include <stdio.h>

int main() {
    int x = 42;
    printf("The answer is %d\n", x);
    return 0;
}
```

**For Loop:**
```c
#include <stdio.h>

int main() {
    for(int i = 0; i < 5; i++) {
        printf("%d ", i);
    }
    printf("\n");
    return 0;
}
```

**Math Operations:**
```c
#include <stdio.h>

int main() {
    int a = 10;
    int b = 20;
    int sum = a + b;
    int product = a * b;
    printf("Sum: %d, Product: %d\n", sum, product);
    return 0;
}
```

**Conditional Statements:**
```c
#include <stdio.h>

int main() {
    int num = 15;
    if(num > 10) {
        printf("Number is greater than 10\n");
    }
    return 0;
}
```

## Building for Production

```bash
npm run build
npm start
```

## Development

To rebuild the WASM module after making changes to the Rust code:

```bash
./build-wasm.sh
```

## Technologies Used

- **Next.js 16** - React framework
- **TypeScript** - Type-safe JavaScript
- **Rust** - Systems programming language
- **WebAssembly** - Binary instruction format for web
- **wasm-pack** - Build tool for Rust/WASM
- **wasm-bindgen** - Bridge between Rust and JavaScript
- **Ace Editor** - Code editor component
- **Tailwind CSS** - Utility-first CSS framework

## Contributing

Contributions are welcome! Areas for improvement:
- Add support for more C features (loops, conditionals, functions, etc.)
- Improve error messages and debugging
- Add more standard library functions
- Optimize WASM binary size
- Add code examples and tutorials

## License

MIT

## Acknowledgments

Built with Next.js, Rust, and WebAssembly.
