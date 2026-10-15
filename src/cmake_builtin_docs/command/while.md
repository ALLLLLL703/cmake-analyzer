CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/while.html)

# while

Evaluate a group of commands while a condition is true

~~~
while(<condition>)
  <commands>
endwhile()
~~~

All commands between while and the matching [`endwhile()`](https://cmake.org/cmake/help/v4.4/command/endwhile.html#command:endwhile) are recorded without being invoked. Once the [`endwhile()`](https://cmake.org/cmake/help/v4.4/command/endwhile.html#command:endwhile) is evaluated, the recorded list of commands is invoked as long as the `<condition>` is true.

The `<condition>` has the same syntax and is evaluated using the same logic as described at length for the [`if()`](https://cmake.org/cmake/help/v4.4/command/if.html#command:if) command.

The commands [`break()`](https://cmake.org/cmake/help/v4.4/command/break.html#command:break) and [`continue()`](https://cmake.org/cmake/help/v4.4/command/continue.html#command:continue) provide means to escape from the normal control flow.

Per legacy, the [`endwhile()`](https://cmake.org/cmake/help/v4.4/command/endwhile.html#command:endwhile) command admits an optional `<condition>` argument. If used, it must be a verbatim repeat of the argument of the opening `while` command.

## See Also

- [`break()`](https://cmake.org/cmake/help/v4.4/command/break.html#command:break)

- [`continue()`](https://cmake.org/cmake/help/v4.4/command/continue.html#command:continue)

- [`foreach()`](https://cmake.org/cmake/help/v4.4/command/foreach.html#command:foreach)

- [`endwhile()`](https://cmake.org/cmake/help/v4.4/command/endwhile.html#command:endwhile)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
