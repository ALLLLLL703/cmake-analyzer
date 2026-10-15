CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_DEBUG_MODE_NO_IMPLICIT_CONFIGURE_LOG.html)

# CMAKE_FIND_DEBUG_MODE_NO_IMPLICIT_CONFIGURE_LOG

Added in version 4.1.

The following commands will report configure log events when they experience a transition between found and not-found states or when the result is first defined:

- [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program)

- [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library)

- [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file)

- [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path)

- [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

The `CMAKE_FIND_DEBUG_MODE_NO_IMPLICIT_CONFIGURE_LOG` boolean variable suppresses these implicit events from the configure log when set to a true value.

~~~
set(CMAKE_FIND_DEBUG_MODE_NO_IMPLICIT_CONFIGURE_LOG TRUE)
find_program(...)
set(CMAKE_FIND_DEBUG_MODE_NO_IMPLICIT_CONFIGURE_LOG FALSE)
~~~

Default is unset.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
