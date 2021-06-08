**Envious Programming Language**

A new programming language that focuses on simplicity without compromising on speed. This language is meant to be born from the desires of the community. In a sense, it is a language that will hopefully address the issues that developers have with other programming languages.

> Perhaps more importantly, this project is a way for me to learn more about compilers and VMs.

**Details about EnvyLang**

Envy is heavily influenced by Math. A lot of the symbols used comes directly from mathametical theory.
In that right, Envy is easy to understand. Envy targets the LLVM or Low-Level Virtual Machine.

Also, note that this language is under active development and that there will be many changes in the future.

**Current status of the language**

Currently, there are 4 different parts of the project that have been partially completed.
- Lexer
- Parser
- Type checker
- Code generator

**Implemented Features**

- Variable with mutabality
- If and else expressions
- While loops
- Functions
- External function definitions
- Static type checking

**The different types of expressions**

Envious is an expression based language. Therefore, most of the statements written are expressions. Here is a detailed description of each expression.

**Define expression**

The define expression creates a new function that is very similar to math.

The syntax of the define expression is as follows:
```
define add(x: Int, y: Int) :: Int = x + y 
```

Some notable parts of the function is the inclusion of the `define` keyword, the function name, a comma-separated list of parameters that have the name of the parameter followed by the type, the return type of the function, and the body which consistes of a single expression.

In other programming languages, this is better known as a `function`.

**Let expression**

The let expression allows the declaration of a new variable or the mutation of a previously defined variable. For example, the variable `x` can be defined using the following expression:

```rust
let x = 123
```
> This will define a new variable `x` that is defined within its scope (typically the surrounding function)

A let expression does not return any value, unlike Java.

**If expression**

The if expressions allows certain expressions be run based on a condition. For example, the message printed out to the console can be
changed based on whether it is sunny or not.

```rust
let isSunny = true
if isSunny then
    print('Y')
else
    print('N')
```

There are three different parts to an if expression: the condition, the then expession, and the optional else expression.
The condition must be a boolean expression and follows the if keyword. The then expression describes the expression to execute if the condition is true and must be preceded by the then keyword. Lastly, in the event that an expression must be run if the condition is false, the else expression can be used.

The if expression returns the value of the branch that was chosen. This implies that the two branches must result in the same type.

**While expression**

The while expression allows for a certain expression to be repeated based on a given condition. Although the word expression is used, a while expression does not return any value.

Currently, the while expression is the only looping mechanism implemented.

The syntax for the while expression is as follows:

```rust
let condition = true
while condition
    expression
```

**Block expression**

The block expression allows multiple expressions to be run. This is most useful when combined with other expressions. The block expression returns the value of the last expression in the block.

A block expression can be constructed by surrounding a group of expressions with a pair of curly braces.

**Application expression**

Lastly, an application expression represents an application of a function with its parameters. This expression returns the value of the function and thus, has the same type as the return value of the function.

A function can be applied as follows:

```rust
function(parameters)
```

In this case, `function` refers to the name of the function and `parameters` refers to the comma separated paramers that are passed to the function.

**TUI and CLI**

In addition to the compiler, there are two seperate modules, namely the TUI (terminal user interface) and the CLI (command line interface)

The TUI allows the user to quickly prototype code in a REPL like environment and see colored error messages and the generated code.
The behavior of the TUI should change soon to show the output of the code as opposed to the generated code.

The CLI provides an interface for the user to interact with the compiler. It provides options to compile, build, and run any given files.

** Quick Start **
The easiest way to play around with Envy is via the TUI. In order to get it running locally, you'll need to install the dependencies. The following directions are currently OS X only:

1. Install [homebrew](https://brew.sh/)
2. Install Rust: `$ brew install rust`
3. Install cmake: `$ brew install cmake`
4. Install LLVM v10.0.0 from the [LLVM Download Page](https://releases.llvm.org/) and unzip it into your home directory.
5. Install the `llvm-sys` package. At the time of writing, the current required version is 10.0.0:
```bash
cd ~ && mkdir -p llvm-10.0.0.src/build
cd llvm-10.0.0.src/build
cmake .. -DCMAKE_INSTALL_PREFIX=$HOME/llvm-10.0.0
cmake --build . --target install # Note that this may take in the neighborhood of 90 minutes.
```
6. Build envious: `$ cd path_to_envious_root && LLVM_SYS_100_PREFIX=$HOME/llvm-10.0.0 cargo build
7. Now enter the TUI: `$ cd path_to_envious_root/target/debug && ./envious -t`
8. If everything worked, you should see something like the following:

