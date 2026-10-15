CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/include.html)

# include

Load and run CMake code from a file or module.

~~~
include(<file|module> [OPTIONAL] [RESULT_VARIABLE <var>]
                      [NO_POLICY_SCOPE] [NO_DIAGNOSTIC_SCOPE])
~~~

Loads and runs CMake code from the file given. Variable reads and writes access the scope of the caller (dynamic scoping). If `OPTIONAL` is present, then no error is raised if the file does not exist. If `RESULT_VARIABLE` is given the variable `<var>` will be set to the full filename which has been included or `NOTFOUND` if it failed.

If a module is specified instead of a file, the file with name `<modulename>.cmake` is searched first in [`CMAKE_MODULE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html#variable:CMAKE_MODULE_PATH), then in the CMake module directory. There is one exception to this: if the file which calls `include()` is located itself in the CMake builtin module directory, then first the CMake builtin module directory is searched and [`CMAKE_MODULE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html#variable:CMAKE_MODULE_PATH) afterwards. See also policy [`CMP0017`](https://cmake.org/cmake/help/v4.4/policy/CMP0017.html#policy:CMP0017).

See the [`cmake_policy()`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#command:cmake_policy) command documentation for discussion of the `NO_POLICY_SCOPE` option.

See the [`cmake_diagnostic()`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#command:cmake_diagnostic) command documentation for discussion of the `NO_DIAGNOSTIC_SCOPE` option.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
