CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CACHE.html)

# CACHE

Added in version 3.13.

Operator to read cache variables.

Use the syntax `$CACHE{VAR}` to read cache entry `VAR`. See the [cmake-language(7) variables](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-variables) documentation for more complete documentation of the interaction of normal variables and cache entries.

When evaluating [Variable References](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#variable-references) of the form `${VAR}`, CMake first searches for a normal variable with that name, and if not found CMake will search for a cache entry with that name. The `$CACHE{VAR}` syntax can be used to do direct cache lookup and ignore any existing normal variable.

See the [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) and [`unset()`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset) commands to see how to write or remove cache variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
