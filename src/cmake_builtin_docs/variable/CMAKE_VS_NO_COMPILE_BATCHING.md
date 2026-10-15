CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_NO_COMPILE_BATCHING.html)

# CMAKE_VS_NO_COMPILE_BATCHING

Added in version 3.24.

Turn off compile batching when using [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators).

This variable is used to initialize the [`VS_NO_COMPILE_BATCHING`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_NO_COMPILE_BATCHING.html#prop_tgt:VS_NO_COMPILE_BATCHING) property on all targets when they are created. See that target property for additional information.

## Example

This shows setting the property for the target `foo` using the variable.

~~~
set(CMAKE_VS_NO_COMPILE_BATCHING ON)
add_library(foo SHARED foo.cpp)
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
