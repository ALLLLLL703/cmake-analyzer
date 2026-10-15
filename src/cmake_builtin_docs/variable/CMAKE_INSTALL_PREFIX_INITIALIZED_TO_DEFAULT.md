CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT.html)

# CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT

Added in version 3.7.1.

CMake sets this variable to a `TRUE` value when the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) has just been initialized to its default value, typically on the first run of CMake within a new build tree and the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_INSTALL_PREFIX.html#envvar:CMAKE_INSTALL_PREFIX) environment variable is not set on the first run of CMake. This can be used by project code to change the default without overriding a user-provided value:

~~~
if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
  set_property(CACHE CMAKE_INSTALL_PREFIX PROPERTY VALUE "/my/default")
endif()
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
