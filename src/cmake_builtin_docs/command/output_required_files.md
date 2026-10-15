CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/output_required_files.html)

# output_required_files

Disallowed since version 3.0. See CMake Policy [`CMP0032`](https://cmake.org/cmake/help/v4.4/policy/CMP0032.html#policy:CMP0032).

Approximate C preprocessor dependency scanning.

This command exists only because ancient CMake versions provided it. CMake handles preprocessor dependency scanning automatically using a more advanced scanner.

~~~
output_required_files(srcfile outputfile)
~~~

Outputs a list of all the source files that are required by the specified `srcfile`. This list is written into `outputfile`. This is similar to writing out the dependencies for `srcfile` except that it jumps from `.h` files into `.cxx`, `.c` and `.cpp` files if possible.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
