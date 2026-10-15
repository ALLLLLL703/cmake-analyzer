CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/function.html)

# function

Start recording a function for later invocation as a command.

~~~
function(<name> [<arg1> ...])
  <commands>
endfunction()
~~~

Defines a function named `<name>` that takes arguments named `<arg1>`, ... The `<commands>` in the function definition are recorded; they are not executed until the function is invoked.

Per legacy, the [`endfunction()`](https://cmake.org/cmake/help/v4.4/command/endfunction.html#command:endfunction) command admits an optional `<name>` argument. If used, it must be a verbatim repeat of the argument of the opening `function` command.

A function opens a new scope: see [`set(var PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) for details.

See the [`cmake_policy()`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#command:cmake_policy) command documentation for the behavior of policies inside functions.

See the [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro) command documentation for differences between CMake functions and macros.

## Invocation

The function invocation is case-insensitive. A function defined as

~~~
function(foo)
  <commands>
endfunction()
~~~

can be invoked through any of

~~~
foo()
Foo()
FOO()
cmake_language(CALL foo)
~~~

and so on. However, it is strongly recommended to stay with the case chosen in the function definition. Typically functions use all-lowercase names.

Added in version 3.18: The [`cmake_language(CALL ...)`](https://cmake.org/cmake/help/v4.4/command/cmake_language.html#command:cmake_language) command can also be used to invoke the function.

## Arguments

When the function is invoked, the recorded `<commands>` are first modified by replacing formal parameters (`${arg1}`, ...) with the arguments passed, and then invoked as normal commands.

In addition to referencing the formal parameters you can reference the `ARGC` variable which will be set to the number of arguments passed into the function as well as `ARGV0`, `ARGV1`, `ARGV2`, ... which will have the actual values of the arguments passed in. This facilitates creating functions with optional arguments.

Furthermore, `ARGV` holds the list of all arguments given to the function and `ARGN` holds the list of arguments past the last expected argument. Referencing to `ARGV#` arguments beyond `ARGC` have undefined behavior. Checking that `ARGC` is greater than `#` is the only way to ensure that `ARGV#` was passed to the function as an extra argument.

## See Also

- [`cmake_parse_arguments()`](https://cmake.org/cmake/help/v4.4/command/cmake_parse_arguments.html#command:cmake_parse_arguments)

- [`endfunction()`](https://cmake.org/cmake/help/v4.4/command/endfunction.html#command:endfunction)

- [`return()`](https://cmake.org/cmake/help/v4.4/command/return.html#command:return)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
