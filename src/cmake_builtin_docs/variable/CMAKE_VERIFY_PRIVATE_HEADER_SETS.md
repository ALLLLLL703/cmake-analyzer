CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VERIFY_PRIVATE_HEADER_SETS.html)

# CMAKE_VERIFY_PRIVATE_HEADER_SETS

Added in version 4.3.

This variable is used to initialize the [`VERIFY_PRIVATE_HEADER_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/VERIFY_PRIVATE_HEADER_SETS.html#prop_tgt:VERIFY_PRIVATE_HEADER_SETS) property of targets when they are created. Setting it to true enables private header set verification.

Projects should not normally set this variable, it is intended as a developer control to be set on the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command line or other equivalent methods. The developer must have the ability to enable or disable header set verification according to the capabilities of their own machine and compiler.

Verification of a dependency's header sets is not typically of interest to developers. Therefore, [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable) explicitly sets `CMAKE_VERIFY_PRIVATE_HEADER_SETS` and [`CMAKE_VERIFY_INTERFACE_HEADER_SETS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VERIFY_INTERFACE_HEADER_SETS.html#variable:CMAKE_VERIFY_INTERFACE_HEADER_SETS) to false for the duration of its call, but restores their original values before returning. If a project brings a dependency directly into the main build (e.g. calling [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) on a vendored project from a git submodule), it should also do likewise. For example:

~~~
# Save original setting so we can restore it later
set(want_interface_header_set_verification ${CMAKE_VERIFY_INTERFACE_HEADER_SETS})
set(want_private_header_set_verification ${CMAKE_VERIFY_PRIVATE_HEADER_SETS})

# Include the vendored dependency with header set verification disabled
set(CMAKE_VERIFY_INTERFACE_HEADER_SETS OFF)
set(CMAKE_VERIFY_PRIVATE_HEADER_SETS OFF)
add_subdirectory(...)   # Vendored sources, e.g. from git submodules

# Add the project's own sources. Restore the developer's original choice
# for whether to enable header set verification.
set(CMAKE_VERIFY_INTERFACE_HEADER_SETS ${want_interface_header_set_verification})
set(CMAKE_VERIFY_PRIVATE_HEADER_SETS ${want_private_header_set_verification})
add_subdirectory(src)
~~~

By default, this variable is not set, which will result in private header set verification being disabled.

See also [`CMAKE_VERIFY_INTERFACE_HEADER_SETS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VERIFY_INTERFACE_HEADER_SETS.html#variable:CMAKE_VERIFY_INTERFACE_HEADER_SETS).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
