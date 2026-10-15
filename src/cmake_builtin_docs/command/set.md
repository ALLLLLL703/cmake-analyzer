CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/set.html)

# set

Set a normal, cache, or environment variable to a given value. See the [cmake-language(7) variables](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-variables) documentation for the scopes and interaction of normal variables and cache entries.

Signatures of this command that specify a `<value>...` placeholder expect zero or more arguments. Multiple arguments will be joined as a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) to form the actual variable value to be set.

## Set Normal Variable

**set(\<variable\> \<value\>... [PARENT_SCOPE])**

Set or unset `<variable>` in the current function or directory scope:

- If at least one `<value>...` is given, set the variable to that value.

- If no value is given, unset the variable. This is equivalent to [`unset(<variable>)`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset).

If the `PARENT_SCOPE` option is given the variable will be set in the scope above the current scope. Each new directory or [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) command creates a new scope. A scope can also be created with the [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) command. `set(PARENT_SCOPE)` will set the value of a variable into the parent directory, calling function, or encompassing scope (whichever is applicable to the case at hand). The previous state of the variable's value stays the same in the current scope (e.g., if it was undefined before, it is still undefined and if it had a value, it is still that value).

The [`block(PROPAGATE)`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block) and [`return(PROPAGATE)`](https://cmake.org/cmake/help/v4.4/command/return.html#command:return) commands can be used as an alternate method to the [`set(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) and [`unset(PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset) commands to update the parent scope.

Note

When evaluating [Variable References](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#variable-references) of the form `${VAR}`, CMake first searches for a normal variable with that name. If no such normal variable exists, CMake will then search for a cache entry with that name. Because of this, **unsetting a normal variable can expose a cache variable that was previously hidden**. To force a variable reference of the form `${VAR}` to return an empty string, use `set(<variable> "")`, which clears the normal variable but leaves it defined.

## Set Cache Entry

**set(CACHE{\<variable\>} [TYPE \<type\>] [HELP \<helpstring\>...] [FORCE] VALUE [\<value\>...])**

Added in version 4.2.

Sets the given cache `<variable>` (cache entry). The options are:

**`TYPE <type>`**

Specify the type of the cache entry. The `<type>` must be one of:

**`BOOL`**

Boolean `ON/OFF` value. [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) offers a checkbox.

**`FILEPATH`**

Path to a file on disk. [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) offers a file dialog.

**`PATH`**

Path to a directory on disk. [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) offers a file dialog.

**`STRING`**

A line of text. [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) offers a text field or a drop-down selection if the [`STRINGS`](https://cmake.org/cmake/help/v4.4/prop_cache/STRINGS.html#prop_cache:STRINGS) cache entry property is set.

**`INTERNAL`**

A line of text. [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) does not show internal entries. They may be used to store variables persistently across runs. Use of this type implies `FORCE`.

If `TYPE` is not specified, if the cache variable already exist and its type is not `UNINITIALIZED`, the type previously specified will be kept otherwise, `STRING` will be used.

**`HELP <helpstring>...`**

The `<helpstring>` must be specified as a line of text providing a quick summary of the option for presentation to [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)) users. If more than one string is given, they are concatenated into a single string with no separator between them.

If `HELP` is not specified, an empty string will be used.

**`FORCE`**

Since cache entries are meant to provide user-settable values this does not overwrite existing cache entries by default. Use the `FORCE` option to overwrite existing entries.

**`VALUE <value>...`**

List of values to be set to the cache `<variable>`. This argument must be always the last one.

If the cache entry does not exist prior to the call or the `FORCE` option is given then the cache entry will be set to the given value.

Note

The content of the cache variable will not be directly accessible if a normal variable of the same name already exists (see [rules of variable evaluation](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-variables)). If policy [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126) is set to `OLD`, any normal variable binding in the current scope will be removed.

It is possible for the cache entry to exist prior to the call but have no type set if it was created on the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command line by a user through the [`-D<var>=<value>`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-D) option without specifying a type. In this case the `set` command will add the type. Furthermore, if the `<type>` is `PATH` or `FILEPATH` and the `<value>` provided on the command line is a relative path, then the `set` command will treat the path as relative to the current working directory and convert it to an absolute path.

**set(\<variable\> \<value\>... CACHE \<type\> \<docstring\> [FORCE])**

This signature is supported for compatibility purpose. Use preferably the other one.

## Set Environment Variable

**set(ENV{\<variable\>} [\<value\>])**

Sets an [`Environment Variable`](https://cmake.org/cmake/help/v4.4/manual/cmake-env-variables.7.html#manual:cmake-env-variables(7)) to the given value. Subsequent calls of `$ENV{<variable>}` will return this new value.

This command affects only the current CMake process, not the process from which CMake was called, nor the system environment at large, nor the environment of subsequent build or test processes.

If no argument is given after `ENV{<variable>}` or if `<value>` is an empty string, then this command will clear any existing value of the environment variable.

Arguments after `<value>` are ignored. If extra arguments are found, then an author warning is issued.

## See Also

- [`unset()`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
