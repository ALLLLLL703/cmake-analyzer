CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/build_name.html)

# build_name

Disallowed since version 3.0. See CMake Policy [`CMP0036`](https://cmake.org/cmake/help/v4.4/policy/CMP0036.html#policy:CMP0036).

Use `${CMAKE_SYSTEM}` and `${CMAKE_CXX_COMPILER}` instead.

~~~
build_name(variable)
~~~

Sets the specified variable to a string representing the platform and compiler settings. These values are now available through the [`CMAKE_SYSTEM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM.html#variable:CMAKE_SYSTEM) and [`CMAKE_CXX_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html#variable:CMAKE_%3CLANG%3E_COMPILER) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
