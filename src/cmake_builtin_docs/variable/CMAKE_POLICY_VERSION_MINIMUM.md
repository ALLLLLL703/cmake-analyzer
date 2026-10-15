CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_VERSION_MINIMUM.html)

# CMAKE_POLICY_VERSION_MINIMUM

Added in version 4.0.

Specify a minimum [Policy Version](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#policy-version) for a project without modifying its calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) and [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version).

This variable should not be set by a project in CMake code as a way to set its own policy version. Use [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) and/or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) for that. This variable is meant to externally set policies for which a project has not itself been updated:

- Users running CMake may set this variable in the cache, e.g., `-DCMAKE_POLICY_VERSION_MINIMUM=3.5`, to try configuring a project that has not been updated to set at least that policy version itself.

   

  Alternatively, users may set the [`CMAKE_POLICY_VERSION_MINIMUM`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_POLICY_VERSION_MINIMUM.html#envvar:CMAKE_POLICY_VERSION_MINIMUM) environment variable to initialize the cache entry in new build trees automatically.

- Projects may set this variable before a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that adds a third-party project in order to set its policy version without modifying third-party code.

See [`CMAKE_POLICY_DEFAULT_CMP<NNNN>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_DEFAULT_CMPNNNN.html#variable:CMAKE_POLICY_DEFAULT_CMP%3CNNNN%3E) to set individual policies.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
