**Envious Programming Language**

A new programming language that focuses on simplicity without compromising on speed. This language is meant to be born from the desires of the community. In a sense, it is a language that will hopefully address the issues that developers have by having developers contribute ideas about the language and its features.

> This project is also a way for me to learn more about compilers and VMs.

**Details About EnvyLang**

Envy is heavily influenced by Math. A lot of the symbols used comes directly from mathametical theory.
In that right, Envy is easy to understand. Envy targets the LLVM or Low-Level Virtual Machine.

**Brief Introduction To Envious Syntax**

Envious has a fairly standard syntax with a couple of changes.

Here is the famous hello world program in Envy.

```
print('Hello, World!')
```

> Here, we can see that, just like in Math, functions are applied with the parameters surrounded by parenthesis.

> Additionally, one data type can be seen: the string type.

**What Are The Different Types In Envious?**

In Envious, there are 6 different types:

-   int
-   float
-   boolean
-   string
-   void
-   any

Void and Any are actually special types in that they can not be directly given as values. This may be changed in the future though.

**What Can I Do With Envy?**

> An interesting thing about the Envious Programming Language is that everything is considered an expression.
> That means that most of these things return a value. Of course, there are exceptions to this rule!

One of the most common requirements in programs are variables.
Variables let us define and maintain state. To define a variable in Envy, the following syntax can be used:

```
let x := 1
```

> Here, the int 1 is bound to the name 'x'. Therefore, any further references to this name is equivalent to using this value. There are exceptions that we will see later though.

This expression is known as the **Let Expression**.

Another common requirement is the ability to compare different values. This can be done through traditional mathematical symbols: <, <=, >, >=, =, and !=.

This is known as the **Equality Expression**.

An example can be seen below:

```
1 < 1 = true
```

> Here, the less than operator is used to check if 1 is less than 1. This is false. Then, the equality operator is used to check if false is equal to true. This is also false. Thus, the overall result of the above expression is false.

What if you wanted to perform some action if and only if some condition is true? Well, you can use an **If Expression**.

An If Expression can be created using the keyword 'if'.

Here is an example:

```
if 1 = 1 print('1 Is Equal To 1!')
```

The if expression has two parts: the condition and the action.

In the example above, the condition can be seen as 1 = 1. The action can be thought up as the code to run if the condition is true.

The only restriction is that the condition must result in a boolean value.

What if you want to run an expression when the condition is false? Use the else part of the If Expression.

Here is an example:

```
if 1 = 2 print('1 Is Equal To 2!') else print('1 Is Not Equal To 2!')
```

Sometimes, you want to run more than one line of code. In these cases, you can use the **Block Expression**.

A Block Expression is just a bunch of expressions surrounded by a pair of curly braces.

Here is an example:

```
{
    print('This Is A Block Expression.')
    print('This Might Not Have Been As Exciting As You May Have Initially Thought.')
}
```

> Did you notice that we used an expression without even knowing about it? The **Function Call** expression.

The print function that we used to print messages earlier was actually an expression.

A common question is how does one create their one functions? Well, this is where the **Define Expression** comes in handy.

The Define Expression allows us to create functions with parameters and return values from those functions.

Here is an example:

```
define f(x: Int) :: Int = x * 2
```

The define expression has multiple parts:

1. The name of the function. In this case, the name is f
2. The parameters. The parameters are enclosed in parenthesis and have the following format, name: type
3. The return type. This is optional. If the return value is omitted, then the return type is inferred.
4. Finally, the expression. This is the code to execute when the function is called. Because this is an expression,
   any previous expressions that you learned about work here. For example, a block expression can be used to have
   multiple lines of code. Within the block expression, another define expression may be created to have a function
   within a function. The possibilities are endless.
