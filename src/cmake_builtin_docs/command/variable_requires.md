CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/variable_requires.html)

# variable_requires

Disallowed since version 3.0. See CMake Policy [`CMP0035`](https://cmake.org/cmake/help/v4.4/policy/CMP0035.html#policy:CMP0035).

Use the [`if()`](https://cmake.org/cmake/help/v4.4/command/if.html#command:if) command instead.

Assert satisfaction of an option's required variables.

~~~
variable_requires(TEST_VARIABLE RESULT_VARIABLE
                  REQUIRED_VARIABLE1
                  REQUIRED_VARIABLE2 ...)
~~~

The first argument (`TEST_VARIABLE`) is the name of the variable to be tested, if that variable is false nothing else is done. If `TEST_VARIABLE` is true, then the next argument (`RESULT_VARIABLE`) is a variable that is set to true if all the required variables are set. The rest of the arguments are variables that must be true or not set to `NOTFOUND` to avoid an error. If any are not true, an error is reported.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
