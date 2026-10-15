CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/block.html)

# block

Added in version 3.25.

Evaluate a group of commands with a dedicated variable and/or policy scope.

~~~
block([SCOPE_FOR [DIAGNOSTICS] [POLICIES] [VARIABLES]]
      [PROPAGATE <var-name>...])
  <commands>
endblock()
~~~

All commands between `block()` and the matching [`endblock()`](https://cmake.org/cmake/help/v4.4/command/endblock.html#command:endblock) are recorded without being invoked. Once the [`endblock()`](https://cmake.org/cmake/help/v4.4/command/endblock.html#command:endblock) is evaluated, the recorded list of commands is invoked inside the requested scopes, then the scopes created by the `block()` command are removed.

**`SCOPE_FOR`**

Specify which scopes must be created.

**`DIAGNOSTICS`**

Added in version 4.4.

Create a new diagnostic scope. This is equivalent to [`cmake_diagnostic(PUSH)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#push) with an automatic [`cmake_diagnostic(POP)`](https://cmake.org/cmake/help/v4.4/command/cmake_diagnostic.html#pop) when leaving the block scope.

**`POLICIES`**

Create a new policy scope. This is equivalent to [`cmake_policy(PUSH)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#push) with an automatic [`cmake_policy(POP)`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#pop) when leaving the block scope.

**`VARIABLES`**

Create a new variable scope.

If `SCOPE_FOR` is not specified, this is equivalent to:

~~~
block(SCOPE_FOR VARIABLES POLICIES DIAGNOSTICS)
~~~

**`PROPAGATE`**

When a variable scope is created by the [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command, this option sets or unsets the specified variables in the parent scope. This is equivalent to [`set(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) or [`unset(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset) commands.

~~~
set(var1 "INIT1")
set(var2 "INIT2")
set(var3 "INIT3")

block(PROPAGATE var1 var2)
  set(var1 "VALUE1")
  unset(var2)
  set(var3 "VALUE3")
endblock()

# Now var1 holds VALUE1, var2 is unset, and var3 holds the initial value INIT3
~~~

This option is only allowed when a variable scope is created. An error will be raised in the other cases.

When the `block()` is inside a [`foreach()`](https://cmake.org/cmake/help/v4.4/command/foreach.html#command:foreach) or [`while()`](https://cmake.org/cmake/help/v4.4/command/while.html#command:while) command, the [`break()`](https://cmake.org/cmake/help/v4.4/command/break.html#command:break) and [`continue()`](https://cmake.org/cmake/help/v4.4/command/continue.html#command:continue) commands can be used inside the block.

~~~
while(TRUE)
  block()
     ...
     # the break() command will terminate the while() command
     break()
  endblock()
endwhile()
~~~

## See Also

- [`endblock()`](https://cmake.org/cmake/help/v4.4/command/endblock.html#command:endblock)

- [`return()`](https://cmake.org/cmake/help/v4.4/command/return.html#command:return)

- [`cmake_policy()`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#command:cmake_policy)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
