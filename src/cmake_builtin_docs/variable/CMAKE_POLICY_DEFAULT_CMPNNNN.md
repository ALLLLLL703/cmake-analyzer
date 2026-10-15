CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_DEFAULT_CMPNNNN.html)

# CMAKE_POLICY_DEFAULT_CMP\<NNNN\>

Default for CMake Policy `CMP<NNNN>` when it is otherwise left unset.

Commands [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) and [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) by default leave policies introduced after the given version unset. Set `CMAKE_POLICY_DEFAULT_CMP<NNNN>` to `OLD` or `NEW` to specify the default for policy `CMP<NNNN>`, where `<NNNN>` is the policy number.

This variable should not be set by a project in CMake code as a way to set its own policies; use [`cmake_policy(SET)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#set) instead. This variable is meant to externally set policies for which a project has not itself been updated:

- Users running CMake may set this variable in the cache (e.g. `-DCMAKE_POLICY_DEFAULT_CMP<NNNN>=<OLD|NEW>`). Set it to `OLD` to quiet a policy warning while using old behavior or to `NEW` to try building the project with new behavior.

- Projects may set this variable before a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that adds a third-party project in order to set its policies without modifying third-party code.

See [`CMAKE_POLICY_VERSION_MINIMUM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_VERSION_MINIMUM.html#variable:CMAKE_POLICY_VERSION_MINIMUM) set policies to `NEW` based on the version of CMake that introduced them.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
