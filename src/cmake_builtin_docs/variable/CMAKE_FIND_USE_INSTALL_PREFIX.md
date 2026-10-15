CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_INSTALL_PREFIX.html)

# CMAKE_FIND_USE_INSTALL_PREFIX

Added in version 3.24.

Controls the default behavior of the following commands for whether or not to search the locations in the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) and [`CMAKE_STAGING_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STAGING_PREFIX.html#variable:CMAKE_STAGING_PREFIX) variables.

- [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program)

- [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library)

- [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file)

- [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path)

- [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

This is useful in cross-compiling environments.

Due to backwards compatibility with [`CMAKE_FIND_NO_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_NO_INSTALL_PREFIX.html#variable:CMAKE_FIND_NO_INSTALL_PREFIX), the behavior of the find command change based on if this variable exists.

CMAKE_FIND_USE_INSTALL_PREFIX |  CMAKE_FIND_NO_INSTALL_PREFIX |  Search |

Not Defined |  On |  NO |

Not Defined |  Off || Not Defined |  YES |

Off |  On |  NO |

Off |  Off || Not Defined |  NO |

On |  On |  YES |

On |  Off || Not Defined |  YES |

By default this variable is not defined. Explicit options given to the above commands take precedence over this variable.

See also the [`CMAKE_FIND_USE_CMAKE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_PATH.html#variable:CMAKE_FIND_USE_CMAKE_PATH), [`CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH), [`CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH), [`CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY), [`CMAKE_FIND_USE_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_PACKAGE_REGISTRY.html#variable:CMAKE_FIND_USE_PACKAGE_REGISTRY), and [`CMAKE_FIND_USE_PACKAGE_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_PACKAGE_ROOT_PATH.html#variable:CMAKE_FIND_USE_PACKAGE_ROOT_PATH) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
