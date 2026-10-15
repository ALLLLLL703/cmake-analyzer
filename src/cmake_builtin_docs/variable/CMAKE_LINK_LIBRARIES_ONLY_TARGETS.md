CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARIES_ONLY_TARGETS.html)

# CMAKE_LINK_LIBRARIES_ONLY_TARGETS

Added in version 3.23.

Set this variable to initialize the [`LINK_LIBRARIES_ONLY_TARGETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARIES_ONLY_TARGETS.html#prop_tgt:LINK_LIBRARIES_ONLY_TARGETS) property of non-imported targets when they are created. Setting it to true enables an additional check that all items named by [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) that can be target names are actually names of existing targets. See the target property documentation for details.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
