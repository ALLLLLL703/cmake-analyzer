CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FASTBUILD_IDE_ARGS.html)

# CMAKE_FASTBUILD_IDE_ARGS

Added in version 4.2.

Specifies the command-line arguments used when invoking `fbuild` from IDE projects.

If not set, the following arguments are used:

~~~
-ide -cache -summary -dist
~~~

Example:

~~~
set(CMAKE_FASTBUILD_IDE_ARGS "-ide -cache -summary -dist -clean")
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
