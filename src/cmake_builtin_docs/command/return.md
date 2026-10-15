CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/return.html)

# return

Return from a file, directory or function.

~~~
return([PROPAGATE <var-name>...])
~~~

When this command is encountered in an included file (via [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) or [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)), it causes processing of the current file to stop and control is returned to the including file. If it is encountered in a file which is not included by another file, e.g. a `CMakeLists.txt`, deferred calls scheduled by [`cmake_language(DEFER)`](https://cmake.org/cmake/help/v4.4/command/cmake_language.html#defer) are invoked and control is returned to the parent directory if there is one.

If `return()` is called in a function, control is returned to the caller of that function. Note that a [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro), unlike a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function), is expanded in place and therefore cannot handle `return()`.

Policy [`CMP0140`](https://cmake.org/cmake/help/v4.4/policy/CMP0140.html#policy:CMP0140) controls the behavior regarding the arguments of the command. All arguments are ignored unless that policy is set to `NEW`.

**`PROPAGATE`**

Added in version 3.25.

This option sets or unsets the specified variables in the parent directory or function caller scope. This is equivalent to [`set(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) or [`unset(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset) commands, except for the way it interacts with the [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command, as described below.

The `PROPAGATE` option can be very useful in conjunction with the [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command. A `return` will propagate the specified variables through any enclosing block scopes created by the [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) commands. Inside a function, this ensures the variables are propagated to the function's caller, regardless of any blocks within the function. If not inside a function, it ensures the variables are propagated to the parent file or directory scope. For example:

  CMakeLists.txt 

~~~
cmake_minimum_required(VERSION 3.25)
project(example)

set(var1 "top-value")

block(SCOPE_FOR VARIABLES)
  add_subdirectory(subDir)
  # var1 has the value "block-nested"
endblock()

# var1 has the value "top-value"
~~~

    subDir/CMakeLists.txt 

~~~
function(multi_scopes result_var1 result_var2)
  block(SCOPE_FOR VARIABLES)
    # This would only propagate out of the immediate block, not to
    # the caller of the function.
    #set(${result_var1} "new-value" PARENT_SCOPE)
    #unset(${result_var2} PARENT_SCOPE)

    # This propagates the variables through the enclosing block and
    # out to the caller of the function.
    set(${result_var1} "new-value")
    unset(${result_var2})
    return(PROPAGATE ${result_var1} ${result_var2})
  endblock()
endfunction()

set(var1 "some-value")
set(var2 "another-value")

multi_scopes(var1 var2)
# Now var1 will hold "new-value" and var2 will be unset

block(SCOPE_FOR VARIABLES)
  # This return() will set var1 in the directory scope that included us
  # via add_subdirectory(). The surrounding block() here does not limit
  # propagation to the current file, but the block() in the parent
  # directory scope does prevent propagation going any further.
  set(var1 "block-nested")
  return(PROPAGATE var1)
endblock()
~~~

## See Also

- [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block)

- [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
