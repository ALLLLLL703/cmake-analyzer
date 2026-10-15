CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html)

# cmake_policy

Manage CMake Policy settings. See the [`cmake-policies(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-policies.7.html#manual:cmake-policies(7)) manual for defined policies.

As CMake evolves it is sometimes necessary to change existing behavior in order to fix bugs or improve implementations of existing features. The CMake Policy mechanism is designed to help keep existing projects building as new versions of CMake introduce changes in behavior. Each new policy (behavioral change) is given an identifier of the form `CMP<NNNN>` where `<NNNN>` is an integer index. Documentation associated with each policy describes the `OLD` and `NEW` behavior and the reason the policy was introduced. Projects may set each policy to select the desired behavior. When CMake needs to know which behavior to use it checks for a setting specified by the project. If no setting is available the `OLD` behavior is assumed and a warning is produced requesting that the policy be set.

## Setting Policies by CMake Version

The `cmake_policy` command is used to set policies to `OLD` or `NEW` behavior. While setting policies individually is supported, we encourage projects to set policies based on CMake versions:

**cmake_policy(VERSION \<min\>[...\<policy_max\>])**

Added in version 3.12: The optional `<policy_max>` version.

`<min>` and the optional `<policy_max>` are each CMake versions of the form `major.minor[.patch[.tweak]]`, and the `...` is literal. The `<min>` version must be at least `2.4` and at most the running version of CMake. The `<policy_max>` version, if specified, must be at least the `<min>` version but may exceed the running version of CMake. If the running version of CMake is older than 3.12, the extra `...` dots will be seen as version component separators, resulting in the `...<policy_max>` part being ignored and preserving the pre-3.12 behavior of basing policies on `<min>`.

This specifies that the current CMake code is written for the given range of CMake versions, `<min>[...<policy_max>]`. It sets the "policy version" to:

- the range's `<policy_max>` version, if specified, or to

- the `<min>` version, or to

- the value of the [`CMAKE_POLICY_VERSION_MINIMUM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_VERSION_MINIMUM.html#variable:CMAKE_POLICY_VERSION_MINIMUM) variable if it is higher than the other two versions.

The policy version effectively requests behavior preferred as of a given CMake version and tells newer CMake versions to warn about their new policies. All policies known to the running version of CMake and introduced in the policy version or earlier will be set to use `NEW` behavior. All policies introduced in later versions will be unset (unless the [`CMAKE_POLICY_DEFAULT_CMP<NNNN>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_DEFAULT_CMPNNNN.html#variable:CMAKE_POLICY_DEFAULT_CMP%3CNNNN%3E) variable sets a default).

Note

`...<policy_max>` does *not* signify that later CMake versions are forbidden. It merely specifies the highest CMake version for which the project or module has been actively updated and maintained.

Note that the [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) command implicitly calls `cmake_policy(VERSION)` too.

Changed in version 4.0: Compatibility with versions of CMake older than 3.5 is removed. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.5 as their policy version (optionally via `...<policy_max>`) will produce an error in CMake 4.0 and above.

Changed in version 3.31: Compatibility with versions of CMake older than 3.10 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.10 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.31 and above.

Changed in version 3.27: Compatibility with versions of CMake older than 3.5 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 3.5 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.27 and above.

Changed in version 3.19: Compatibility with versions of CMake older than 2.8.12 is deprecated. Calls to [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required) or [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version) that do not specify at least 2.8.12 as their policy version (optionally via `...<policy_max>`) will produce a deprecation warning in CMake 3.19 and above.

## Setting Policies Explicitly

**cmake_policy(SET CMP\<NNNN\> NEW|OLD)**

Tell CMake to use the `OLD` or `NEW` behavior for a given policy. Projects depending on the old behavior of a given policy may silence a policy warning by setting the policy state to `OLD`. Alternatively one may fix the project to work with the new behavior and set the policy state to `NEW`.

Note

The `OLD` behavior of a policy is [deprecated by definition](https://cmake.org/cmake/help/v4.4/manual/cmake-policies.7.html#cmake-policies-intro) and may be removed in a future version of CMake.

## Checking Policy Settings

**cmake_policy(GET CMP\<NNNN\> \<variable\>)**

Check whether a given policy is set to `OLD` or `NEW` behavior. The output `<variable>` value will be `OLD` or `NEW` if the policy is set, and empty otherwise.

## CMake Policy Stack

CMake keeps policy settings on a stack, so changes made by the `cmake_policy` command affect only the top of the stack. A new entry on the policy stack is managed automatically for each subdirectory to protect its parents and siblings. CMake also manages a new entry for scripts loaded by [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) and [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands except when invoked with the `NO_POLICY_SCOPE` option (see also policy [`CMP0011`](https://cmake.org/cmake/help/v4.4/policy/CMP0011.html#policy:CMP0011)). The `cmake_policy` command provides an interface to manage custom entries on the policy stack:

**cmake_policy(PUSH)**

Create a new entry on the policy stack.

**cmake_policy(POP)**

Remove the last policy stack entry created with `cmake_policy(PUSH)`.

Each `PUSH` must have a matching `POP` to erase any changes. This is useful to make temporary changes to policy settings. Calls to the [`cmake_minimum_required(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required), [`cmake_policy(VERSION)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#version), or [`cmake_policy(SET)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#set) commands influence only the current top of the policy stack.

Added in version 3.25: The [`block(SCOPE_FOR POLICIES)`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command offers a more flexible and more secure way to manage the policy stack. The pop action is done automatically when leaving the block scope, so there is no need to precede each [`return()`](https://cmake.org/cmake/help/v4.4/command/return.html#command:return) with a call to [`cmake_policy(POP)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#pop).

~~~
# stack management with cmake_policy()
function(my_func)
  cmake_policy(PUSH)
  cmake_policy(SET ...)
  if (<cond1>)
    ...
    cmake_policy(POP)
    return()
  elseif(<cond2>)
    ...
    cmake_policy(POP)
    return()
  endif()
  ...
  cmake_policy(POP)
endfunction()

# stack management with block()/endblock()
function(my_func)
  block(SCOPE_FOR POLICIES)
    cmake_policy(SET ...)
    if (<cond1>)
      ...
      return()
    elseif(<cond2>)
      ...
      return()
    endif()
    ...
  endblock()
endfunction()
~~~

Commands created by the [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) and [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro) commands record policy settings when they are created and use the pre-record policies when they are invoked. If the function or macro implementation sets policies, the changes automatically propagate up through callers until they reach the closest nested policy stack entry.

## See Also

- [`cmake_minimum_required()`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
