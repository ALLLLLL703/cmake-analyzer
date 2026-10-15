CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/continue.html)

# continue

Added in version 3.2.

Continue to the top of enclosing foreach or while loop.

~~~
continue()
~~~

The `continue()` command allows a cmake script to abort the rest of the current iteration of a [`foreach()`](https://cmake.org/cmake/help/v4.4/command/foreach.html#command:foreach) or [`while()`](https://cmake.org/cmake/help/v4.4/command/while.html#command:while) loop, and start at the top of the next iteration.

See also the [`break()`](https://cmake.org/cmake/help/v4.4/command/break.html#command:break) command.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
