CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html)

# cmake_diagnostic

Added in version 4.4.

Manage CMake Diagnostic settings. See the [`cmake-diagnostics(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-diagnostics.7.html#manual:cmake-diagnostics(7)) manual for a list of available categories.

## Synopsis

~~~
Setting Diagnostics
  cmake_diagnostic(SET <category> <action> [RECURSE])
  cmake_diagnostic(PROMOTE <category> <action> [NO_RECURSE])
  cmake_diagnostic(DEMOTE <category> <action> [NO_RECURSE])

Checking Diagnostic Actions
  cmake_diagnostic(GET <diagnostic> <out-var>)

CMake Diagnostic Stack
  cmake_diagnostic(PUSH)
  cmake_diagnostic(POP)
~~~

## Setting Diagnostics

**cmake_diagnostic(SET CMD_\<CATEGORY\> \<action\> [RECURSE])**

**cmake_diagnostic(PROMOTE CMD_\<CATEGORY\> \<action\> [NO_RECURSE])**

**cmake_diagnostic(DEMOTE CMD_\<CATEGORY\> \<action\> [NO_RECURSE])**

Set or alter the action taken when a diagnostic belonging to a particular category is triggered.

The `SET` subcommand sets the action for the specified diagnostic category. The `PROMOTE` subcommand increases the severity for the specified diagnostic category, or does nothing if the action was already set to an equal or higher severity. The `DEMOTE` subcommand decreases the severity for the specified diagnostic category, or does nothing if the action was already set to an equal or lower severity.

The possible `<action>`s (in order of severity) are:

**`IGNORE`**

Do nothing.

**`WARN`**

Report a warning and continue processing.

**`SEND_ERROR`**

Report an error, continue processing, but skip generation.

The [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) executable will return a non-zero [exit code](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmake-exit-code).

**`FATAL_ERROR`**

Report an error, stop processing and generation.

The [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) executable will return a non-zero [exit code](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmake-exit-code).

Some diagnostic categories are hierarchical. The `RECURSE` and `NO_RECURSE` options determine whether changing the action for a diagnostic category also modifies any child categories. By default, the `PROMOTE` and `DEMOTE` subcommands are recursive, while the `SET` subcommand is not. Note that the alteration for child categories is independent of the prior action set on any parents; that is, `PROMOTE` and `DEMOTE`, when operating recursively, will operate on all child categories even if a parent category's action was not altered.

## Checking Diagnostic Actions

**cmake_diagnostic(GET CMD_\<CATEGORY\> \<variable\>)**

Check what action is currently specified for a diagnostic category. The output `<variable>` value will be one of `IGNORE`, `WARN`, `SEND_ERROR` or `FATAL_ERROR`.

## CMake Diagnostic Stack

CMake keeps diagnostic settings on a stack, so changes made by the `cmake_diagnostic` command affect only the top of the stack. A new entry on the diagnostic stack is managed automatically for each subdirectory to protect its parents and siblings. CMake also manages a new entry for scripts loaded by [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) and [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands except when invoked with the `NO_DIAGNOSTIC_SCOPE` option. The `cmake_diagnostic` command provides an interface to manage custom entries on the diagnostic stack:

**cmake_diagnostic(PUSH)**

Create a new entry on the diagnostic stack.

**cmake_diagnostic(POP)**

Remove the last diagnostic stack entry created with `cmake_diagnostic(PUSH)`.

Each `PUSH` must have a matching `POP` to erase any changes. This is useful to make temporary changes to diagnostic settings. Calls to the [`cmake_diagnostic(SET)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#set), [`cmake_diagnostic(PROMOTE)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#promote), or [`cmake_diagnostic(DEMOTE)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#demote) commands influence only the current top of the diagnostic stack.

The [`block(SCOPE_FOR DIAGNOSTICS)`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command offers a more flexible and more secure way to manage the diagnostic stack. The pop action is done automatically when leaving the block scope, so there is no need to precede each [`return()`](https://cmake.org/cmake/help/v4.4/command/return.html#command:return) with a call to [`cmake_diagnostic(POP)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#pop).

~~~
# stack management with cmake_diagnostic()
function(my_func)
  cmake_diagnostic(PUSH)
  cmake_diagnostic(SET ...)
  if (<cond1>)
    ...
    cmake_diagnostic(POP)
    return()
  elseif(<cond2>)
    ...
    cmake_diagnostic(POP)
    return()
  endif()
  ...
  cmake_diagnostic(POP)
endfunction()

# stack management with block()/endblock()
function(my_func)
  block(SCOPE_FOR DIAGNOSTICS)
    cmake_diagnostic(SET ...)
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

Commands created by the [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) and [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro) commands record diagnostic settings when they are created and use the pre-record diagnostics when they are invoked. If the function or macro implementation sets diagnostics, the changes automatically propagate up through callers until they reach the closest nested diagnostic stack entry.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
