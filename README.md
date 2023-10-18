# Shade a functional language for Graphics and more.

Shade is a language i had in mind for a while. I wanted to have a functional language with more than just the language.
What i saw is that a lot of times i found myself using the same bits of code in a lots of my shaders. So i wanted to write
a new language that provide a standard library and modules and such.

## Rust i choose you.

I went with rust for writing the compiler. I choose it for two reason. It can iterop fearly well with C/C++. And i was playing with
wgpu at the time which is implemented in Rust. Hence my choice.

## Usefull for Graphics but not only.

As i wanted to use shade for creating games i wanted it to not only be used as a shading language but also as a scripting language.

### A shader language

The principal goal of the shading side of the language is to automate the buffer layout creation.

All variable used in the input of the vertex shader will be put in the buffer. All the global varibles will be put in the Uniform buffer.

The api for creating a shader module is not set yet. But a few things are already defined.

As the language doesn't have any reserved keywords the user will be required to specify theentry points.

### A scripting language

The scripting part of the language is fairly classic.

All the types declared in the script will be a component in an ecs system of your choice.
Or will need to be provided to the functions. All the functions will be systems.


## Road Map

- [ ] Full syntax.
    - [x] Functions.
    - [x] Type declartions.
    - [ ] Higher order types.

- [x] Lexer
    - [x] basic function syntax.
    - [ ] arithmetic operations.
    - [ ] 

- [x] basic syntax parser.
    - [x] Parse Simple unary exprs.
    - [x] Parse Simple statements.

- [ ] interpreter.

- [ ] compile to naga intermediate repr.

- [ ] standard library.

## Syntax

### Inspirations
Shade is a functional language it has a syntax inspired by elm and nix. It has type inference as well as genericity.
Its syntax is designed to be as non intrusive as possible. There are few reserved keywords. This implicates 
the use of more special characters and operators to do what keywords do.

### Comments
Line comments are defined with `--`
and block comments with `-{` for the oppening and `}-` for the closing.

No syntax yet for Documentation comments.

### Functions
All function are anonymous and values. To declare a function use the `:` symbol. All functions take one argument.
Here is a function that doubles a value:

```
x: x * 2 
```

To make a function that takes no argument simply omit the variable name.
Here is a function that always return 2:

```
():2 -- This is also equivalent to (): 2 and 2.
```

To make functions with multiple arguments we use a concept called currying, making a function returning another function.
So here is a function that adds two numbers.
```
a: b: a + b
```

To call anonymous functions simply pass arguments as such:
```
(a: b: a + b) 1 2 -- This expression returns 3.
```

To keep the function around you can assign them to a identifier:
```
add = a: b: a + b
```

As mentioned previously shade supports currying:
```
add_one = add 1 

add_one 2 -- This expression returns 3.
```


There are several operators on functions.

The pipe operator allows to pass an argument as the last argument:
```
10 
  .add_one
  .print  -- prints 10 in the terminal.
```
This is equivalent to:
```
print (add_one 10)
```
The reversed pipe operator allows to pass an argument as the first argument:
```
div = a: b: a / b

10 
    <| div 2
    <| print -- prints 1
```

This is equivalent to:
```
print (div (10) 2)
```


The dot permits to call a function where the type of the first argument is valid for this function.
```
add = a: b: 5
10 .add 5
   |> print -- prints 15 in the terminal
```

### Primitive Types, Data structures and First Order Types (in progress) 

#### Primitive Types

Here are the primitive types of the language:
    - Int
    - Nat
    - Array denoted by `[]`
    - Char denoted by `''`
    - Strings denoted by `""`
    - Real

To assign a type to a binding we use the symbol `@`:

```
a @ Int
```
It signify `a` is of type int.

To define the type of a function you simply use the types:
```
add_one @ Int -> Int = a: a + 1
```
You can split the type from the declaration:
```
add_one @ Int -> Int 
add_one = a : a + 1
```
To make the type infered simply omit the type declaration:
```
add_one = a: a + 1
```

You can shorten types by declaring type aliases using the `$` symbol: 
```
IntList $ [Int]
```
In this case it is not shortened but it commes handy with data structures later.

For Example you can define `List` as:
```
ListOf $ a: [a]
IntList $ ListOf Int
```
In this case List is a generic type.
Notice that we use a function that takes a type as argument and return another type.
With higher order types we can limit the types that can be passed in this type function.

#### Data structures

There are two types of data structures:
    - Records
    - Enum
```
-- This is a record 
{ a@ Int
, b@ Int 
}

-- This is an enum with all possible variants.
| INT Int 
, REAL Real 
, Unit 
, Record { a @ Int, b @ Int } 
, List [ Int ]
|

-- An enum can have constant variants holding values
| ONE 1 
, TWO 2 
|

-- An enum with only unit variants is abstracted as a bit field.
| TOOGLE_A
, TOOGLE_B
|

```

You can deconstruct functions argument passing with a record as folows.
```
sq_len = { x, y, z }:  x*x + y*y + z*z
```
This reqire that the struct passed in argument has the fields `x`, `y` and `z`.


You can allow for other(unused) fields as such:
```
sq_len = { x, y, z, ... }:  x*x + y*y + z*z
```

You can make optional arguments with providing defaults values:
```
sq_len = { x, y, z, w ? 0 }:  x*x + y*y + z*z + w*w
```


Same thing for Enums:
```
add_one = INT a ->
    INT (a+1)
add_one = REAL a ->
    FLOAT (a+1)
```
The compiler will check that all cases are covered.


#### Control Flow
All the keywords used by the language are used for control flow as its the only place where they improve readability.

If syntax (the else is attached to the closest if):
```
if true then
    print "ok"
else if false then 
    print "will not print"
else 
    print "not"
```

If wanted otherwise do:
```
if true then
    if false then 
        print "im not"
    else 
        print "im prited"

if false then
    (if true then print "im not")
else 
    print "im prited"
```

Match syntax
```
a @ | A 'a', B Int |
print_a @ a -> Effect _ = a: match a with
    A a -> print "A {}" a;
    B b -> print "b {}" b;
```

Let/in syntax
```
let a = 2 in print a -- Prints 2
```

#### Type Land.

##### Type Operators.

Shade allows for usefull things in type land. The merge operator `//`.
Applied on two record type declaration it merges the two in one it emits a compile error if the two records have a field named identicaly. 

Eg.
```
point2d $ f: { x @ f, y @ f }
size2d $ f: { h @ f, w @ f }
rect2d $ f: (point2d f) // (size2d f)

-- exact same definition as.
rect2d $ f: 
    { x @ f
    , y @ f
    , h @ f
    , w @ f
    }
```

Applied on two enums types it meges them similarly. 
```
Even = | Zero, Two |
Odd = | One, Three |
Positives = Event // Odd
-- exact same definition as.
Positives $ 
    | Zero
    , One
    , Two
    , Three
    |
```

##### Type Predicates.
TODO

##### Type Functions.
TODO

#### Rank polymorphism.
TODO

#### Combinators 
The language defines a bunch of combinators that are considered usefull for expressiveness. But whats a combinator? Its a special type of function that treat only with its arguments and mangle them around. Know there are a basis of combinators (Some ). Most combinators can be applied at compile time but be carefull its easy to make compile time explode using to much of them (can possibly cause infinite recursion).

Eg: `f: g: x: y: f (g x y)`

##### Identity 
The most usefull and simingly simple combinator is `id`. Its defined as:
`id = x: x`
You might wonder where it would be usefull? But wait for a bit i will show you.
##### Compose.
`.`
##### Blackbird. 
(Compose but with inner function with 2 argument.)
`..`
##### ._
(A more general form of compose. It allows the inner function to have has many arguments as it wants.)
`._`

##### Fork
`|-`

