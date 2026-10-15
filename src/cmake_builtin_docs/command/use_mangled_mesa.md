CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/use_mangled_mesa.html)

# use_mangled_mesa

Disallowed since version 3.0. See CMake Policy [`CMP0030`](https://cmake.org/cmake/help/v4.4/policy/CMP0030.html#policy:CMP0030).

Copy mesa headers for use in combination with system GL.

~~~
use_mangled_mesa(PATH_TO_MESA OUTPUT_DIRECTORY)
~~~

The path to mesa includes, should contain `gl_mangle.h`. The mesa headers are copied to the specified output directory. This allows mangled mesa headers to override other GL headers by being added to the include directory path earlier.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
