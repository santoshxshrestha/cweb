# CWeb

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

The C interpreter supports a comprehensive set of C features:

### Data Types
- `int` - Integer variables
- `float` / `double` - Floating-point numbers
- `char` - Character variables
- `long` / `short` - Integer variations
- **Arrays** - Single-dimensional arrays (e.g., `int arr[10]`)
- **Pointers** - Simulated pointer support with address-of (`&`) and dereference (`*`) operators

### Control Flow
- `if` / `else` / `else if` statements - Full conditional execution
- `for` loops - Definite iteration with initialization, condition, and increment
- `while` loops - Indefinite iteration
- `do-while` loops - Post-condition loops
- `switch` / `case` / `default` - Multi-way branching
- `break` - Exit loops early
- `continue` - Skip to next iteration

### Operators

#### Arithmetic
- `+`, `-`, `*`, `/`, `%` (modulo)
- Compound assignment: `+=`, `-=`, `*=`, `/=`, `%=`
- Increment/Decrement: `++`, `--`

#### Comparison
- `<`, `>`, `<=`, `>=`, `==`, `!=`

#### Logical
- `&&` (AND), `||` (OR), `!` (NOT)

#### Bitwise
- `&` (AND), `|` (OR), `^` (XOR), `~` (NOT)
- `<<` (left shift), `>>` (right shift)

#### Other
- `? :` (ternary operator)
- `&` (address-of) - Get memory address of a variable
- `*` (dereference) - Access value at pointer address

### Pointer System

CWeb features a **simulated pointer system** that mimics real pointer behavior:

- **Address-of operator (`&`)**: Get the "address" of any variable
- **Dereference operator (`*`)**: Read/write values through pointers
- **Pointer arithmetic**: Use pointers with array indexing
- **NULL pointers**: Initialize pointers to 0 or NULL
- **Realistic addresses**: Simulated memory addresses (e.g., `0x1000`, `0x1008`)
- **Segmentation faults**: Invalid pointer access generates error messages

This gives you a realistic feel of working with pointers without actual memory manipulation!

### Standard Library Functions

#### Input/Output (stdio.h)
- `printf()` - Formatted output with specifiers: `%p`, `%d`, `%i`, `%f`, `%lf`, `%c`, `%s`, `%ld`, `%u`, `%x`, `%o`
- `puts()` - Print string with newline
- `scanf()` - Input handling (basic support)
- `gets()` - Get string input (basic support)

#### String Functions (string.h)
- `strlen()` - Get string length
- `strcpy()` - Copy strings
- `strcat()` - Concatenate strings
- `strcmp()` - Compare strings (basic support)

#### Math Functions (math.h)
- `sqrt()` - Square root
- `pow()` - Power function
- `abs()` / `fabs()` - Absolute value
- `sin()`, `cos()`, `tan()` - Trigonometric functions
- `ceil()` - Ceiling function
- `floor()` - Floor function
- `exp()` - Exponential function
- `log()` - Natural logarithm

#### Standard Library (stdlib.h)
- `rand()` - Generate pseudo-random numbers
- `srand()` - Seed random number generator (basic support)

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

**If-Else Statement:**
```c
#include <stdio.h>

int main() {
    int num = 15;
    if(num > 10) {
        printf("Number is greater than 10\n");
    } else {
        printf("Number is 10 or less\n");
    }
    return 0;
}
```

**Switch Statement:**
```c
#include <stdio.h>

int main() {
    int day = 3;
    switch(day) {
        case 1:
            printf("Monday\n");
            break;
        case 2:
            printf("Tuesday\n");
            break;
        case 3:
            printf("Wednesday\n");
            break;
        default:
            printf("Other day\n");
    }
    return 0;
}
```

**Arrays:**
```c
#include <stdio.h>

int main() {
    int arr[5];
    for(int i = 0; i < 5; i++) {
        arr[i] = i * 2;
    }
    
    for(int i = 0; i < 5; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    return 0;
}
```

**Logical Operators:**
```c
#include <stdio.h>

int main() {
    int a = 10, b = 20;
    if(a > 5 && b < 30) {
        printf("Both conditions are true\n");
    }
    
    if(a > 15 || b > 15) {
        printf("At least one condition is true\n");
    }
    return 0;
}
```

**Bitwise Operations:**
```c
#include <stdio.h>

int main() {
    int a = 5;  // 0101 in binary
    int b = 3;  // 0011 in binary
    
    printf("a & b = %d\n", a & b);  // AND: 1
    printf("a | b = %d\n", a | b);  // OR: 7
    printf("a ^ b = %d\n", a ^ b);  // XOR: 6
    printf("a << 1 = %d\n", a << 1); // Left shift: 10
    return 0;
}
```

**Math Operations:**
```c
#include <stdio.h>
#include <math.h>

int main() {
    double x = 16.0;
    printf("sqrt(16) = %f\n", sqrt(x));
    printf("pow(2, 3) = %f\n", pow(2, 3));
    printf("ceil(4.3) = %f\n", ceil(4.3));
    printf("floor(4.7) = %f\n", floor(4.7));
    return 0;
}
```

**String Functions:**
```c
#include <stdio.h>
#include <string.h>

int main() {
    char str1[20] = "Hello";
    char str2[20] = " World";
    
    printf("Length of str1: %d\n", strlen(str1));
    strcat(str1, str2);
    printf("Concatenated: %s\n", str1);
    return 0;
}
```

**Break and Continue:**
```c
#include <stdio.h>

int main() {
    // Break example
    for(int i = 0; i < 10; i++) {
        if(i == 5) break;
        printf("%d ", i);
    }
    printf("\n");
    
    // Continue example
    for(int i = 0; i < 10; i++) {
        if(i % 2 == 0) continue;
        printf("%d ", i);  // Only odd numbers
    }
    printf("\n");
    return 0;
}
```

**Ternary Operator:**
```c
#include <stdio.h>

int main() {
    int a = 10, b = 20;
    int max = (a > b) ? a : b;
    printf("Maximum: %d\n", max);
    return 0;
}
```

**Compound Assignment:**
```c
#include <stdio.h>

int main() {
    int x = 10;
    x += 5;  // x = x + 5
    printf("After += 5: %d\n", x);
    x *= 2;  // x = x * 2
    printf("After *= 2: %d\n", x);
    return 0;
}
```

**Pointers - Basic Usage:**
```c
#include <stdio.h>

int main() {
    int x = 42;
    int *ptr = &x;  // ptr now holds the address of x
    
    printf("Value of x: %d\n", x);
    printf("Address of x: %p\n", &x);
    printf("Value of ptr: %p\n", ptr);
    printf("Value at ptr: %d\n", *ptr);  // Dereference pointer
    
    *ptr = 100;  // Modify x through pointer
    printf("New value of x: %d\n", x);
    
    return 0;
}
```

**Pointers with Arrays:**
```c
#include <stdio.h>

int main() {
    int arr[5] = {10, 20, 30, 40, 50};
    int *ptr = &arr[0];  // Pointer to first element
    
    printf("First element: %d\n", *ptr);
    printf("Second element: %d\n", *(ptr + 1));  // Pointer arithmetic
    
    // Using array as pointer
    printf("Third element: %d\n", arr[2]);
    printf("Address of third element: %p\n", &arr[2]);
    
    return 0;
}
```

**Pointer Arithmetic:**
```c
#include <stdio.h>

int main() {
    int numbers[5] = {1, 2, 3, 4, 5};
    int *p = &numbers[0];
    
    for(int i = 0; i < 5; i++) {
        printf("%d ", *(p + i));  // Access array via pointer arithmetic
    }
    printf("\n");
    
    return 0;
}
```

**Swapping with Pointers:**
```c
#include <stdio.h>

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int main() {
    int x = 10, y = 20;
    printf("Before swap: x=%d, y=%d\n", x, y);
    
    // Note: Function calls are simulated - you can manually inline
    int temp = x;
    x = y;
    y = temp;
    
    printf("After swap: x=%d, y=%d\n", x, y);
    return 0;
}
```

**NULL Pointer Check:**
```c
#include <stdio.h>

int main() {
    int *ptr = NULL;  // or int *ptr = 0;
    
    if(ptr == NULL) {
        printf("Pointer is NULL\n");
    }
    
    int x = 42;
    ptr = &x;
    
    if(ptr != NULL) {
        printf("Pointer is valid, value: %d\n", *ptr);
    }
    
    return 0;
}
```

## Unsupported C Features

While the interpreter now supports many C features including simulated pointers, the following are still **NOT supported**:

### Data Types & Structures
- ❌ **Pointer to pointer** (`**ptr`) - Multi-level pointers not fully supported
- ❌ **Function pointers** - Cannot store/call functions via pointers
- ❌ **Void pointers** (`void *`) - Generic pointers not supported
- ❌ **Multi-dimensional arrays** - Only 1D arrays supported
- ❌ **Structs** - No structure definitions or member access
- ❌ **Unions** - Not supported
- ❌ **Enums** - Not supported
- ❌ **Typedef** - Cannot create custom type aliases
- ❌ **Static/Const/Volatile qualifiers** - Not supported
- ❌ **Unsigned types** - Limited support
- ❌ **Size_t, ptrdiff_t** - Not supported

### Functions
- ❌ **User-defined functions** - Cannot define custom functions beyond `main()`
- ❌ **Function parameters** - No function arguments
- ❌ **Function return values** - Limited support
- ❌ **Recursion** - Not supported
- ❌ **Function pointers** - Not supported
- ❌ **Variadic functions** - Not supported

### Preprocessor Directives
- ❌ **#define macros** - Macros are ignored
- ❌ **#ifdef, #ifndef, #endif** - Conditional compilation not supported
- ❌ **#include** - Headers are recognized but not actually included
- ❌ **#pragma** - Not supported
- ❌ **Macro functions** - Not supported

### Memory Management
- ❌ **malloc/calloc/realloc/free** - No true dynamic memory allocation (pointers are simulated)
- ❌ **sizeof operator** - Not supported
- ❌ **Memory addresses as integers** - Addresses are simulated, not real

### Standard Library (Partial Support)
Many standard library functions have limited or no support:

#### stdio.h (limited)
- ❌ `fprintf`, `sprintf`, `snprintf`
- ❌ `fopen`, `fclose`, `fread`, `fwrite`
- ❌ `getchar`, `putchar`
- ❌ `fgets`, `fputs`
- ❌ `fseek`, `ftell`, `rewind`
- ❌ File I/O operations

#### string.h (minimal)
- ❌ `strncpy`, `strncat`, `strncmp`
- ❌ `strchr`, `strrchr`, `strstr`
- ❌ `strtok`
- ❌ `memcpy`, `memmove`, `memset`

#### stdlib.h (minimal)
- ❌ `malloc`, `calloc`, `realloc`, `free`
- ❌ `atoi`, `atof`, `atol`
- ❌ `exit`, `abort`
- ❌ `system`
- ❌ `qsort`, `bsearch`

#### math.h (partial)
- ❌ `round`, `fmod`
- ❌ `log10`
- ❌ `asin`, `acos`, `atan`, `atan2`
- ❌ `sinh`, `cosh`, `tanh`

#### time.h
- ❌ All time-related functions

#### ctype.h
- ❌ `isalpha`, `isdigit`, `isspace`, etc.
- ❌ `toupper`, `tolower`

### Advanced Features
- ❌ **Multiple source files** - Only single file compilation
- ❌ **Header files** - Cannot create/include custom headers
- ❌ **External linkage** - No linking with other code
- ❌ **Inline assembly** - Not supported
- ❌ **Compiler optimizations** - No optimization levels
- ❌ **Debugging symbols** - No debugging support
- ❌ **Command-line arguments** - `argc`/`argv` not supported

### Input/Output Limitations
- ❌ **Interactive input** - `scanf()` and `gets()` have very limited support
- ❌ **File operations** - Cannot read/write files
- ❌ **Binary I/O** - Not supported
- ❌ **Buffering control** - Not supported

### Scope & Storage
- ❌ **Global variables** - Limited or no support
- ❌ **Static variables** - Not supported
- ❌ **Extern variables** - Not supported
- ❌ **Register variables** - Not supported
- ❌ **Scope rules** - Simplified scope handling

## What's New vs Original Unsupported List

✅ **Now Supported:**
- **Pointers** - Simulated pointer system with `&`, `*`, NULL, pointer arithmetic
- Arrays (1D)
- else/else if statements
- do-while loops
- switch/case/default
- break and continue statements
- Logical operators (&&, ||, !)
- Bitwise operators (&, |, ^, ~, <<, >>)
- Ternary operator (? :)
- Compound assignment operators (+=, -=, *=, /=, %=)
- More string functions (strcpy, strcat)
- More math functions (ceil, floor, exp, log)
- rand() function
- More printf format specifiers (%x, %o, %p)

## Limitations Summary

⚠️ **This is an educational interpreter with simulated features.** It now supports:
- Basic to intermediate C programming constructs
- **Simulated pointers** - Realistic pointer behavior without actual memory access
- Arrays, control flow, and operators
- Many standard library functions
- Educational algorithm implementations

**How Pointers Work**: Pointers are simulated using a fake memory system. Each variable gets a simulated "address" (like `0x1000`), and pointer operations (`&`, `*`) work on this simulated memory. This provides a realistic learning experience without actual memory manipulation!

**Still Missing:** Multi-level pointers, structs, user-defined functions, true dynamic memory (malloc/free), file I/O, and advanced C features.

**Use Case**: Learning C syntax, understanding pointers conceptually, implementing algorithms, and understanding control flow. For full C programming with real memory access, use a real compiler like GCC or Clang.

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

Built with Next.js, Rust, and WebAssembly. CWeb - A modern online C compiler running entirely in your browser.
