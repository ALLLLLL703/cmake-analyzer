CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html)

# cmake_minimum_required

Require a minimum version of cmake.

~~~
cmake_minimum_required(VERSION <min>[...<policy_max>] [FATAL_ERROR])
~~~

Added in version 3.12: The optional `<policy_max>` version behavior; ignored in older CMake.

Sets the minimum required version of cmake for a project. Also updates the policy settings as explained below.

`<min>` and the optional `<policy_max>` are each CMake versions of the form `major.minor[.patch[.tweak]]`, and the `...` is literal.

If the running version of CMake is lower than the `<min>` required version it will stop processing the project and report an error. The optional `<policy_max>` version, if specified, must be at least the `<min>` version and sets the [Policy Version](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#policy-version). If the running version of CMake is older than 3.12, the extra `...` dots will be seen as version component separators, resulting in the `...<policy_max>` part being ignored and preserving the pre-3.12 behavior of basing policies on `<min>`.

This command will set the value of the [`CMAKE_MINIMUM_REQUIRED_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MINIMUM_REQUIRED_VERSION.html#variable:CMAKE_MINIMUM_REQUIRED_VERSION) variable to `<min>`.

The `FATAL_ERROR` option is accepted but ignored by CMake 2.6 and higher. It should be specified so CMake versions 2.4 and lower fail with an error instead of just a warning.

Note

Call the `cmake_minimum_required()` command at the beginning of the top-level `CMakeLists.txt` file even before calling the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command. It is important to establish version and policy settings before invoking other commands whose behavior they may affect. See also policy [`CMP0000`](https://cmake.org/cmake/help/v4.4/policy/CMP0000.html#policy:CMP0000).

Calling `cmake_minimum_required()` inside a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) limits some effects to the function scope when invoked. For example, the [`CMAKE_MINIMUM_REQUIRED_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MINIMUM_REQUIRED_VERSION.html#variable:CMAKE_MINIMUM_REQUIRED_VERSION) variable won't be set in the calling scope. Functions do not introduce their own policy scope though, so policy settings of the caller *will* be affected (see below). Due to this mix of things that do and do not affect the calling scope, calling `cmake_minimum_required()` inside a function is generally discouraged.

## Policy Version

`cmake_minimum_required(VERSION <min>[...<policy_max>])` implicitly invokes

~~~
cmake_policy(VERSION <min>[...<policy_max>])
~~~

This specifies that the current CMake code is written for the given range of CMake versions, `<min>[...<policy_max>]`. It sets the "policy version" to:

- the range's `<policy_max>` version, if specified, or to

- the `<min>` version, or to

- the value of the [`CMAKE_POLICY_VERSION_MINIMUM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_VERSION_MINIMUM.html#variable:CMAKE_POLICY_VERSION_MINIMUM) variable if it is higher than the other two versions.

The policy version effectively requests behavior preferred as of a given CMake version and tells newer CMake versions to warn about their new policies. All policies known to the running version of CMake and introduced in the policy version or earlier will be set to use `NEW` behavior. All policies introduced in later versions will be unset (unless the [`CMAKE_POLICY_DEFAULT_CMP<NNNN>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_DEFAULT_CMPNNNN.html#variable:CMAKE_POLICY_DEFAULT_CMP%3CNNNN%3E) variable sets a default).

Note

`...<policy_max>` does *not* signify that later CMake versions are forbidden. It merely specifies the highest CMake version for which the project or module has been actively updated and maintained.

Changed in version 4.0: Compatibility with versions of CMake older than 3.5 is removed. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.5 as their policy version (optionally via `...<policy_max>`) will produce an error in CMake 4.0 and above.

Changed in version 3.31: Compatibility with versions of CMake older than 3.10 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.10 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.31 and above.

Changed in version 3.27: Compatibility with versions of CMake older than 3.5 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.5 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.27 and above.

Changed in version 3.19: Compatibility with versions of CMake older than 2.8.12 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 2.8.12 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.19 and above.

## See Also

- [`cmake_policy()`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#command:cmake_policy)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
