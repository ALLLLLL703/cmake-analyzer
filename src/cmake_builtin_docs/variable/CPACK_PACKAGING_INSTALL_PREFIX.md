CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CPACK_PACKAGING_INSTALL_PREFIX.html)

# CPACK_PACKAGING_INSTALL_PREFIX

The prefix used in the built package.

Each CPack generator has a default value (like `/usr`). This default value may be overwritten from the `CMakeLists.txt` or the [`cpack(1)`](https://cmake.org/cmake/help/v4.4/manual/cpack.1.html#manual:cpack(1)) command line by setting an alternative value. Example:

~~~
set(CPACK_PACKAGING_INSTALL_PREFIX "/opt")
~~~

This is not the same purpose as [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) which is used when installing from the build tree without building a package.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
