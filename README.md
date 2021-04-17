**Envious Programming Language**

A new programming language that focuses on simplicity without compromising on speed. This language is meant to be born from the desires of the community. In a sense, it is a language that will hopefully address the issues that developers have by having developers contribute ideas about the language and its features.

> This project is also a way for me to learn more about compilers and VMs.

**Details about EnvyLang**

Envy is heavily influenced by Math. A lot of the symbols used comes directly from mathametical theory.
In that right, Envy is easy to understand. Envy targets the LLVM or Low-Level Virtual Machine.

Also, note that this language is under active development and that there will be many changes in the future.

**Current status of the language**

Currently, there are 4 different parts of the project that have be partially completed.
- Lexer
- Parser
- Type checker
- Code generator

More specifically, the implemented features are listed below.

- Mutable variables
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

**Let expression**

The let expression allows the declaration of a new variable or the mutation of a previously defined variable. For example, if I wanted to create a new variable `x`, the following expression can be used:

```rust
let x = 123
```
> This will define a new variable `x` that is defined within its scope (typically the surrounding function)

