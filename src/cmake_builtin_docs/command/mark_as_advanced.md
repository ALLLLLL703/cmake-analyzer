CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/mark_as_advanced.html)

# mark_as_advanced

Mark cmake cached variables as advanced.

~~~
mark_as_advanced([CLEAR|FORCE] <var1> ...)
~~~

Sets the advanced/non-advanced state of the named cached variables.

An advanced variable will not be displayed in any of the cmake GUIs unless the `show advanced` option is on. In script mode, the advanced/non-advanced state has no effect.

If the keyword `CLEAR` is given then advanced variables are changed back to unadvanced. If the keyword `FORCE` is given then the variables are made advanced. If neither `FORCE` nor `CLEAR` is specified, new values will be marked as advanced, but if a variable already has an advanced/non-advanced state, it will not be changed.

Changed in version 3.17: Variables passed to this command which are not already in the cache are ignored. See policy [`CMP0102`](https://cmake.org/cmake/help/v4.4/policy/CMP0102.html#policy:CMP0102).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
