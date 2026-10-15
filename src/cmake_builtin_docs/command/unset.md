CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/unset.html)

# unset

Unset a variable, cache variable, or environment variable.

## Unset Normal Variable

**unset(\<variable\> [PARENT_SCOPE])**

Removes a normal variable from the current scope, causing it to become undefined.

If `PARENT_SCOPE` is present then the variable is removed from the scope above the current scope. See the same option in the [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) command for further details.

Note

When evaluating [Variable References](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#variable-references) of the form `${VAR}`, CMake first searches for a normal variable with that name. If no such normal variable exists, CMake will then search for a cache entry with that name. Because of this, **unsetting a normal variable can expose a cache variable that was previously hidden**. To force a variable reference of the form `${VAR}` to return an empty string, use `set(<variable> "")`, which clears the normal variable but leaves it defined.

## Unset Cache Entry

**unset(CACHE{\<variable\>})**

Added in version 4.2.

Removes `<variable>` from the cache, causing it to become undefined.

**unset(\<variable\> CACHE)**

This signature is supported for compatibility purpose. Use preferably the other one.

## Unset Environment Variable

**unset(ENV{\<variable\>})**

Removes `<variable>` from the currently available [`Environment Variables`](https://cmake.org/cmake/help/v4.4/manual/cmake-env-variables.7.html#manual:cmake-env-variables(7)). Subsequent calls of `$ENV{<variable>}` will return the empty string.

This command affects only the current CMake process, not the process from which CMake was called, nor the system environment at large, nor the environment of subsequent build or test processes.

## See Also

- [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
