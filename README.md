# Shade a functional language for Graphics and more.

Shade is a language i had in mind for a while. I wanted to have a functional language with more than just the language.
What i saw is that a lot of times i found myself using the same bits of code in a lots of my shaders. So i wanted to write
a new language that provide a standard library and modules and such.

## Rust i choose you.

I went with rust for writing the compiler. I choose it for two reason. It can iterop fearly well with C/C++. And i was playing with
wgpu at the time which is implemented in Rust. Hence my choice.

## Usefull for Graphics but not only.

As i wanted to use shade for creating games i wanted it to not only be used as a shading language but also as a scripting language.

## Road Map

- [x] Lexer
- [ ] basic syntax parser.
    - [ ] Parse Simple unary exprs.
    - [ ] Parse Simple statements.
- [ ] interpreter.
    - [x] Unary expr int + float.
    - [x] Assignement expr to identifier.
    - [x] Binary expr int + float.
- [ ] compile to naga intermediate repr.
- [ ] standard library.
- [ ] Image generator.
- [ ] Video generator.

## Syntax

Shade is a functional language it has a syntax inspired by elm and nix. It has type inference as well as genericity.
Its syntax is designed to be as non intrusive as possible. There in not a single reserved keyword. This come to the cost of more characters to do what keywords do.

Here is how we declare a function that adds two things that can be added:
```
add = a: b:
    a + b
```

It supports currying:
```
add_one = add 1 
```

Functions can be chained:
```
10 
    |> add_one
    |> print
```

There are three types of data structures:
    - Records
    - Enum
    - Lists
```
a_and_b =:
    { a= : int
    , b= : int 
    }

int_or_float =:
    | INT int
    | FLOAT float 

int_list =: [1 2 3 4]
```


You can also notice that type declarations are also functions with no arguments.


To match on the elements of an enum we use the `@` symbol.
```
val: enum = INT
matchfn = int_or_float val: 
  @ val 
  | INT a -> "is int"
  | INT b -> "is float"
  | _ -> "default"
```

Fields of a record can be accessed with a dot:
```
```

You can deconstruct functions argument passing with a record as folows.
```
add = {a, b, ...}:
    a + b
```
This function will take all values that are records with fields `a` and `b`.
Same thing for Enums:
```
add_one = INT a: 
    INT (a+1)
add_one = FLOAT a:
    FLOAT (a+1)
```
