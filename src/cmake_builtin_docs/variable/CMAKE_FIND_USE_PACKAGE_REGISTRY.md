CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_PACKAGE_REGISTRY.html)

# CMAKE_FIND_USE_PACKAGE_REGISTRY

Added in version 3.16.

Controls the default behavior of the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) command for whether or not to search paths provided by the [User Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#user-package-registry).

By default this variable is not set and the behavior will fall back to that determined by the deprecated [`CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY) variable. If that is also not set, then [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) will use the [User Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#user-package-registry) unless the `NO_CMAKE_PACKAGE_REGISTRY` option is provided.

This variable takes precedence over [`CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY) when both are set.

In some cases, for example to locate only system wide installations, it is not desirable to use the [User Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#user-package-registry) when searching for packages. If the `CMAKE_FIND_USE_PACKAGE_REGISTRY` variable is `FALSE`, all the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands will skip the [User Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#user-package-registry) as if they were called with the `NO_CMAKE_PACKAGE_REGISTRY` argument.

See also [Disabling the Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#disabling-the-package-registry) and the [`CMAKE_FIND_USE_CMAKE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_PATH.html#variable:CMAKE_FIND_USE_CMAKE_PATH), [`CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH), [`CMAKE_FIND_USE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_INSTALL_PREFIX.html#variable:CMAKE_FIND_USE_INSTALL_PREFIX), [`CMAKE_FIND_USE_CMAKE_SYSTEM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_SYSTEM_PATH.html#variable:CMAKE_FIND_USE_CMAKE_SYSTEM_PATH), [`CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH), [`CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY), and [`CMAKE_FIND_USE_PACKAGE_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_PACKAGE_ROOT_PATH.html#variable:CMAKE_FIND_USE_PACKAGE_ROOT_PATH) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
