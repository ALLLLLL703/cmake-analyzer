CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY.html)

# CMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY

Added in version 3.1.

Deprecated since version 3.16: Use the [`CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY) variable instead.

By default this variable is not set. If neither [`CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY) nor `CMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY` is set, then [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) will use the [System Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#system-package-registry) unless the `NO_CMAKE_SYSTEM_PACKAGE_REGISTRY` option is provided.

`CMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY` is ignored if [`CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY) is set.

In some cases, it is not desirable to use the [System Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#system-package-registry) when searching for packages. If the `CMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY` variable is `TRUE`, all the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands will skip the [System Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#system-package-registry) as if they were called with the `NO_CMAKE_SYSTEM_PACKAGE_REGISTRY` argument.

See also [Disabling the Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#disabling-the-package-registry).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
