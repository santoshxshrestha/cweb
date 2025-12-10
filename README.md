# C Online Compiler

An online C compiler that runs entirely in your browser using WebAssembly. Write, compile, and execute C programs without any backend server.

## Features

- **In-Browser Compilation**: Compile and run C code directly in your browser using WebAssembly
- **Real-Time Editing**: Code editor with syntax highlighting for C programming
- **Multiple Themes**: Switch between dark (Monokai) and light (GitHub) themes
- **Vim Mode**: Optional Vim keybindings for power users
- **Auto-Completion**: IntelliSense-style autocomplete for C keywords and standard library functions
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

Currently supports:
- `printf()` with format specifiers (%d, %i)
- Variable declarations (int)
- Basic arithmetic operations (+, -, *, /)
- Simple expressions and assignments

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
