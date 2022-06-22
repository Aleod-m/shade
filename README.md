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

- [x] Lexer
- [x] basic syntax parser.
    - [x] Parse Simple unary exprs.
    - [x] Parse Simple statements.
- [ ] interpreter.
    - [x] Unary expr int + float.
    - [x] Assignement expr to identifier.
    - [x] Binary expr int + float.
- [ ] compile to naga intermediate repr.
- [ ] standard library.
- [ ] Image generator.
- [ ] Video generator.

## Syntax

### Inspirations
Shade is a functional language it has a syntax inspired by elm and nix. It has type inference as well as genericity.
Its syntax is designed to be as non intrusive as possible. There in not a single reserved keyword. This come to the cost of more characters to do what keywords do.

### Comments
Line comments are defined with `--`.
And block comments with `-{` for the oppening and `}-` for the closing.

### Functions
All function are anonymous and values. To declare a function use the `:` symbol. All functions take one argument.
Here is a function that doubles a value:
```
x: x * 2 
```
To make a function that takes no argument simply omit the variable name.
Here is a function that always return 2:
```
: 2
```

To make functions with multiple arguments we use a concept called currying. make a function that returns anoter function.
So here is a function that adds two numbers.
```
a: b: a + b
```
To call anonymous functions simply pass arguments as such:
```
(a: b: a + b) 1 2 -- 3
```

To keep the function around you can assign them to a identifier:
```
add = a: b:
    a + b
```

As mentioned previously shade supports currying:
```
add_one = add 1 

add_one 2 -- 3
```

There are several operators on functions.

The pipe operator allows to chain functions:
```
10 
    |> add_one
    |> print
```
is equivalent to:
```
print (add_one 10)
```

### Types and structures (in progress) 

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
You can make generic types with function with arguments.
```
vec3 = ty:
    { x =:ty
    , y =:ty
    , z =:ty
    }

vec3i =: vec3 int
```

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


