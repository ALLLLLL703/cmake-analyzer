CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/ENV.html)

# ENV

Operator to read environment variables.

Use the syntax `$ENV{VAR}` to read environment variable `VAR`.

To test whether an environment variable is defined, use the signature `if(DEFINED ENV{<name>})` of the [`if()`](https://cmake.org/cmake/help/v4.4/command/if.html#command:if) command.

Note

Environment variable names containing special characters like parentheses may need to be escaped. (Policy [`CMP0053`](https://cmake.org/cmake/help/v4.4/policy/CMP0053.html#policy:CMP0053) must also be enabled.) For example, to get the value of the Windows environment variable `ProgramFiles(x86)`, use:

~~~
set(ProgramFiles_x86 "$ENV{ProgramFiles\(x86\)}")
~~~

For general information on environment variables, see the [Environment Variables](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-environment-variables) section in the [`cmake-language(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#manual:cmake-language(7)) manual.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
