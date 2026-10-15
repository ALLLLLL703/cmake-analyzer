CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE_BEFORE.html)

# CMAKE_PROJECT_INCLUDE_BEFORE

Added in version 3.15.

A CMake language file to be included as the first step of all [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command calls. This is intended for injecting custom code into project builds without modifying their source. See [Code Injection](https://cmake.org/cmake/help/v4.4/command/project.html#code-injection) for a more detailed discussion of files potentially included during a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call.

Added in version 3.29: This variable can be a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of CMake language files to be included sequentially. It can also now refer to module names to be found in [`CMAKE_MODULE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html#variable:CMAKE_MODULE_PATH) or as a builtin CMake module.

See also the [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE), [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE_BEFORE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE_BEFORE), [`CMAKE_PROJECT_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE.html#variable:CMAKE_PROJECT_INCLUDE), and [`CMAKE_PROJECT_TOP_LEVEL_INCLUDES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_TOP_LEVEL_INCLUDES.html#variable:CMAKE_PROJECT_TOP_LEVEL_INCLUDES) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
