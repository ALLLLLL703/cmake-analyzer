CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_NO_PACKAGE_REGISTRY.html)

# CMAKE_EXPORT_NO_PACKAGE_REGISTRY

Added in version 3.1.

Disable the [`export(PACKAGE)`](https://cmake.org/cmake/help/v4.4/command/export.html#package) command when [`CMP0090`](https://cmake.org/cmake/help/v4.4/policy/CMP0090.html#policy:CMP0090) is not set to `NEW`.

In some cases, for example for packaging and for system wide installations, it is not desirable to write the user package registry. If the `CMAKE_EXPORT_NO_PACKAGE_REGISTRY` variable is enabled, the [`export(PACKAGE)`](https://cmake.org/cmake/help/v4.4/command/export.html#package) command will do nothing.

If [`CMP0090`](https://cmake.org/cmake/help/v4.4/policy/CMP0090.html#policy:CMP0090) is set to `NEW` this variable does nothing, and the [`CMAKE_EXPORT_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_PACKAGE_REGISTRY.html#variable:CMAKE_EXPORT_PACKAGE_REGISTRY) variable controls the behavior instead.

See also [Disabling the Package Registry](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#disabling-the-package-registry).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
