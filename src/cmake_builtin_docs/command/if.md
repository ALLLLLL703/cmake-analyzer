CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/if.html)

# if

Conditionally execute a group of commands.

## Synopsis

~~~
if(<condition>)
  <commands>
elseif(<condition>) # optional block, can be repeated
  <commands>
else()              # optional block
  <commands>
endif()
~~~

Evaluates the `condition` argument of the `if` clause according to the [Condition syntax](https://cmake.org/cmake/help/v4.4/command/if.html#condition-syntax) described below. If the result is true, then the `commands` in the `if` block are executed. Otherwise, optional `elseif` blocks are processed in the same way. Finally, if no `condition` is true, `commands` in the optional `else` block are executed.

Per legacy, the [`else()`](https://cmake.org/cmake/help/v4.4/command/else.html#command:else) and [`endif()`](https://cmake.org/cmake/help/v4.4/command/endif.html#command:endif) commands admit an optional `<condition>` argument. If used, it must be a verbatim repeat of the argument of the opening `if` command.

## Condition Syntax

The following syntax applies to the `condition` argument of the `if`, `elseif` and [`while()`](https://cmake.org/cmake/help/v4.4/command/while.html#command:while) clauses.

Compound conditions are evaluated in the following order of precedence:

- [Parentheses](https://cmake.org/cmake/help/v4.4/command/if.html#parentheses).

- Unary tests such as:

- The [Existence Checks](https://cmake.org/cmake/help/v4.4/command/if.html#existence-checks) [`COMMAND`](https://cmake.org/cmake/help/v4.4/command/if.html#command), [`DEFINED`](https://cmake.org/cmake/help/v4.4/command/if.html#defined), [`DIAGNOSTIC`](https://cmake.org/cmake/help/v4.4/command/if.html#diagnostic), [`EXISTS`](https://cmake.org/cmake/help/v4.4/command/if.html#exists), [`POLICY`](https://cmake.org/cmake/help/v4.4/command/if.html#policy), [`TARGET`](https://cmake.org/cmake/help/v4.4/command/if.html#target), and [`TEST`](https://cmake.org/cmake/help/v4.4/command/if.html#test).

- The [File Operations](https://cmake.org/cmake/help/v4.4/command/if.html#file-operations) [`IS_READABLE`](https://cmake.org/cmake/help/v4.4/command/if.html#is-readable), [`IS_WRITABLE`](https://cmake.org/cmake/help/v4.4/command/if.html#is-writable), [`IS_EXECUTABLE`](https://cmake.org/cmake/help/v4.4/command/if.html#is-executable), [`IS_DIRECTORY`](https://cmake.org/cmake/help/v4.4/command/if.html#is-directory), [`IS_SYMLINK`](https://cmake.org/cmake/help/v4.4/command/if.html#is-symlink), and [`IS_ABSOLUTE`](https://cmake.org/cmake/help/v4.4/command/if.html#is-absolute).

- Binary tests such as those described in [Comparisons](https://cmake.org/cmake/help/v4.4/command/if.html#comparisons), [Version Comparisons](https://cmake.org/cmake/help/v4.4/command/if.html#version-comparisons), and [Path Comparisons](https://cmake.org/cmake/help/v4.4/command/if.html#path-comparisons), as well as [`IN_LIST`](https://cmake.org/cmake/help/v4.4/command/if.html#in-list) and [`IS_NEWER_THAN`](https://cmake.org/cmake/help/v4.4/command/if.html#is-newer-than).

- Unary logical operator [`NOT`](https://cmake.org/cmake/help/v4.4/command/if.html#not).

- Binary logical operators [`AND`](https://cmake.org/cmake/help/v4.4/command/if.html#and) and [`OR`](https://cmake.org/cmake/help/v4.4/command/if.html#or), from left to right, without any short-circuit.

### Basic Expressions

**if(\<constant\>)**

True if the constant is `1`, `ON`, `YES`, `TRUE`, `Y`, or a non-zero number (including floating point numbers). False if the constant is `0`, `OFF`, `NO`, `FALSE`, `N`, `IGNORE`, `NOTFOUND`, the empty string, or ends in the suffix `-NOTFOUND`. Named boolean constants are case-insensitive. If the argument is not one of these specific constants, it is treated as a variable or string (see [Variable Expansion](https://cmake.org/cmake/help/v4.4/command/if.html#variable-expansion) further below) and one of the following two forms applies.

**if(\<variable\>)**

True if given a variable that is defined to a value that is not a false constant. False otherwise, including if the variable is undefined. Note that macro arguments are not variables. [Environment Variables](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-environment-variables) also cannot be tested this way, e.g. `if(ENV{some_var})` will always evaluate to false.

**if(\<string\>)**

A quoted string always evaluates to false unless:

- The string's value is one of the true constants, or

- in CMake versions prior to 4.0, policy [`CMP0054`](https://cmake.org/cmake/help/v4.4/policy/CMP0054.html#policy:CMP0054) is not set to `NEW` and the string's value happens to be a variable name that is affected by [`CMP0054`](https://cmake.org/cmake/help/v4.4/policy/CMP0054.html#policy:CMP0054)'s behavior.

**if()**

False if no argument is provided.

### Logic Operators

**if(NOT \<condition\>)**

True if the condition is not true.

**if(\<cond1\> AND \<cond2\>)**

True if both conditions would be considered true individually.

**if(\<cond1\> OR \<cond2\>)**

True if either condition would be considered true individually.

**if((condition) AND (condition OR (condition)))**

The conditions inside the parenthesis are evaluated first and then the remaining condition is evaluated as in the other examples. Where there are nested parenthesis the innermost are evaluated as part of evaluating the condition that contains them.

### Existence Checks

**if(COMMAND \<command-name\>)**

True if the given name is a command, macro or function that can be invoked.

**if(DIAGNOSTIC \<category\>)**

Added in version 4.4.

True if the given name is an existing diagnostic category.

**if(POLICY \<policy-id\>)**

True if the given name is an existing policy (of the form `CMP<NNNN>`).

**if(TARGET \<target-name\>)**

True if the given name is an existing logical target name created by a call to the [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable), [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library), or [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) command that has already been invoked (in any directory).

**if(TEST \<test-name\>)**

Added in version 3.3.

True if the given name is an existing test name created by the [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) command.

**if(DEFINED \<name\>|CACHE{\<name\>}|ENV{\<name\>})**

True if a variable, cache variable or environment variable with given `<name>` is defined. The value of the variable does not matter. Note the following caveats:

- Macro arguments are not variables.

- It is not possible to test directly whether a `<name>` is a non-cache variable. The expression `if(DEFINED someName)` will evaluate to true if either a cache or non-cache variable `someName` exists. In comparison, the expression `if(DEFINED CACHE{someName})` will only evaluate to true if a cache variable `someName` exists. Both expressions need to be tested if you need to know whether a non-cache variable exists: `if(DEFINED someName AND NOT DEFINED CACHE{someName})`.

Added in version 3.14: Added support for `CACHE{<name>}` variables.

**if(\<variable|string\> IN_LIST \<variable\>)**

Added in version 3.3.

True if the given element is contained in the named list variable.

### File Operations

**if(EXISTS \<path-to-file-or-directory\>)**

True if the named file or directory exists and is readable. Behavior is well-defined only for explicit full paths (a leading `~/` is not expanded as a home directory and is considered a relative path). Resolves symbolic links, i.e. if the named file or directory is a symbolic link, returns true if the target of the symbolic link exists.

False if the given path is an empty string.

Note

Prefer [`if(IS_READABLE)`](https://cmake.org/cmake/help/v4.4/command/if.html#is-readable) to check file readability. `if(EXISTS)` may be changed in the future to only check file existence.

**if(IS_READABLE \<path-to-file-or-directory\>)**

Added in version 3.29.

True if the named file or directory is readable. Behavior is well-defined only for explicit full paths (a leading `~/` is not expanded as a home directory and is considered a relative path). Resolves symbolic links, i.e. if the named file or directory is a symbolic link, returns true if the target of the symbolic link is readable.

False if the given path is an empty string.

**if(IS_WRITABLE \<path-to-file-or-directory\>)**

Added in version 3.29.

True if the named file or directory is writable. Behavior is well-defined only for explicit full paths (a leading `~/` is not expanded as a home directory and is considered a relative path). Resolves symbolic links, i.e. if the named file or directory is a symbolic link, returns true if the target of the symbolic link is writable.

False if the given path is an empty string.

**if(IS_EXECUTABLE \<path-to-file-or-directory\>)**

Added in version 3.29.

True if the named file or directory is executable. Behavior is well-defined only for explicit full paths (a leading `~/` is not expanded as a home directory and is considered a relative path). Resolves symbolic links, i.e. if the named file or directory is a symbolic link, returns true if the target of the symbolic link is executable.

False if the given path is an empty string.

**if(\<file1\> IS_NEWER_THAN \<file2\>)**

True if `file1` is newer than `file2` or if one of the two files doesn't exist. Behavior is well-defined only for full paths. If the file time stamps are exactly the same, an `IS_NEWER_THAN` comparison returns true, so that any dependent build operations will occur in the event of a tie. This includes the case of passing the same file name for both `file1` and `file2`.

**if(IS_DIRECTORY \<path\>)**

True if `path` is a directory. Behavior is well-defined only for full paths.

False if the given path is an empty string.

**if(IS_SYMLINK \<path\>)**

True if the given path is a symbolic link. Behavior is well-defined only for full paths.

**if(IS_ABSOLUTE \<path\>)**

True if the given path is an absolute path. Note the following special cases:

- An empty `path` evaluates to false.

- On Windows hosts, any `path` that begins with a drive letter and colon (e.g. `C:`), a forward slash or a backslash will evaluate to true. This means a path like `C:no\base\dir` will evaluate to true, even though the non-drive part of the path is relative.

- On non-Windows hosts, any `path` that begins with a tilde (`~`) evaluates to true.

### Comparisons

**if(\<variable|string\> MATCHES \<regex\>)**

True if the given string or variable's value matches the given regular expression. See [Regex Specification](https://cmake.org/cmake/help/v4.4/command/string.html#regex-specification) for regex format. `()` groups are captured in [`CMAKE_MATCH_<n>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MATCH_n.html#variable:CMAKE_MATCH_%3Cn%3E) variables.

**if(\<variable|string\> LESS \<variable|string\>)**

True if the given string or variable's value parses as a real number (like a C `double`) and less than that on the right.

**if(\<variable|string\> GREATER \<variable|string\>)**

True if the given string or variable's value parses as a real number (like a C `double`) and greater than that on the right.

**if(\<variable|string\> EQUAL \<variable|string\>)**

True if the given string or variable's value parses as a real number (like a C `double`) and equal to that on the right.

**if(\<variable|string\> LESS_EQUAL \<variable|string\>)**

Added in version 3.7.

True if the given string or variable's value parses as a real number (like a C `double`) and less than or equal to that on the right.

**if(\<variable|string\> GREATER_EQUAL \<variable|string\>)**

Added in version 3.7.

True if the given string or variable's value parses as a real number (like a C `double`) and greater than or equal to that on the right.

**if(\<variable|string\> STRLESS \<variable|string\>)**

True if the given string or variable's value is lexicographically less than the string or variable on the right.

**if(\<variable|string\> STRGREATER \<variable|string\>)**

True if the given string or variable's value is lexicographically greater than the string or variable on the right.

**if(\<variable|string\> STREQUAL \<variable|string\>)**

True if the given string or variable's value is lexicographically equal to the string or variable on the right.

**if(\<variable|string\> STRLESS_EQUAL \<variable|string\>)**

Added in version 3.7.

True if the given string or variable's value is lexicographically less than or equal to the string or variable on the right.

**if(\<variable|string\> STRGREATER_EQUAL \<variable|string\>)**

Added in version 3.7.

True if the given string or variable's value is lexicographically greater than or equal to the string or variable on the right.

### Version Comparisons

**if(\<variable|string\> VERSION_LESS \<variable|string\>)**

Component-wise integer version number comparison (version format is `major[.minor[.patch[.tweak]]]`, omitted components are treated as zero). Any non-integer version component or non-integer trailing part of a version component effectively truncates the string at that point.

**if(\<variable|string\> VERSION_GREATER \<variable|string\>)**

Component-wise integer version number comparison (version format is `major[.minor[.patch[.tweak]]]`, omitted components are treated as zero). Any non-integer version component or non-integer trailing part of a version component effectively truncates the string at that point.

**if(\<variable|string\> VERSION_EQUAL \<variable|string\>)**

Component-wise integer version number comparison (version format is `major[.minor[.patch[.tweak]]]`, omitted components are treated as zero). Any non-integer version component or non-integer trailing part of a version component effectively truncates the string at that point.

**if(\<variable|string\> VERSION_LESS_EQUAL \<variable|string\>)**

Added in version 3.7.

Component-wise integer version number comparison (version format is `major[.minor[.patch[.tweak]]]`, omitted components are treated as zero). Any non-integer version component or non-integer trailing part of a version component effectively truncates the string at that point.

**if(\<variable|string\> VERSION_GREATER_EQUAL \<variable|string\>)**

Added in version 3.7.

Component-wise integer version number comparison (version format is `major[.minor[.patch[.tweak]]]`, omitted components are treated as zero). Any non-integer version component or non-integer trailing part of a version component effectively truncates the string at that point.

### Path Comparisons

**if(\<variable|string\> PATH_EQUAL \<variable|string\>)**

Added in version 3.24.

Lexicographically compares two CMake paths component-by-component without accessing the filesystem. Only if every component of both paths match will the two paths compare equal. Multiple path separators are effectively collapsed into a single separator, but note that backslashes are not converted to forward slashes. No other [path normalization](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#normalization) is performed. Trailing slashes are preserved, thus `/a/b` and `/a/b/` are not equal.

Component-wise comparison is superior to string-based comparison due to the handling of multiple path separators. In the following example, the expression evaluates to true using `PATH_EQUAL`, but false with `STREQUAL`:

~~~
# comparison is TRUE
if ("/a//b/c" PATH_EQUAL "/a/b/c")
   ...
endif()

# comparison is FALSE
if ("/a//b/c" STREQUAL "/a/b/c")
   ...
endif()
~~~

See [`cmake_path(COMPARE)`](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#command:cmake_path) for more details.

## Variable Expansion

The if command was written very early in CMake's history, predating the `${}` variable evaluation syntax, and for convenience evaluates variables named by its arguments as shown in the above signatures. Note that normal variable evaluation with `${}` applies before the if command even receives the arguments. Therefore code like

~~~
set(var1 OFF)
set(var2 "var1")
if(${var2})
~~~

appears to the if command as

~~~
if(var1)
~~~

and is evaluated according to the `if(<variable>)` case documented above. The result is `OFF` which is false. However, if we remove the `${}` from the example then the command sees

~~~
if(var2)
~~~

which is true because `var2` is defined to `var1` which is not a false constant.

Automatic evaluation applies in the other cases whenever the above-documented condition syntax accepts `<variable|string>`:

- The left hand argument to [`MATCHES`](https://cmake.org/cmake/help/v4.4/command/if.html#matches) is first checked to see if it is a defined variable. If so, the variable's value is used, otherwise the original value is used.

- If the left hand argument to [`MATCHES`](https://cmake.org/cmake/help/v4.4/command/if.html#matches) is missing it returns false without error

- Both left and right hand arguments to [`LESS`](https://cmake.org/cmake/help/v4.4/command/if.html#less), [`GREATER`](https://cmake.org/cmake/help/v4.4/command/if.html#greater), [`EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#equal), [`LESS_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#less-equal), and [`GREATER_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#greater-equal) are independently tested to see if they are defined variables. If so, their defined values are used; otherwise the original value is used.

- Both left and right hand arguments to [`STRLESS`](https://cmake.org/cmake/help/v4.4/command/if.html#strless), [`STRGREATER`](https://cmake.org/cmake/help/v4.4/command/if.html#strgreater), [`STREQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#strequal), [`STRLESS_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#strless-equal), and [`STRGREATER_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#strgreater-equal) are independently tested to see if they are defined variables. If so, their defined values are used; otherwise the original value is used.

- Both left and right hand arguments to [`VERSION_LESS`](https://cmake.org/cmake/help/v4.4/command/if.html#version-less), [`VERSION_GREATER`](https://cmake.org/cmake/help/v4.4/command/if.html#version-greater), [`VERSION_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#version-equal), [`VERSION_LESS_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#version-less-equal), and [`VERSION_GREATER_EQUAL`](https://cmake.org/cmake/help/v4.4/command/if.html#version-greater-equal) are independently tested to see if they are defined variables. If so, their defined values are used; otherwise the original value is used.

- The left hand argument to [`IN_LIST`](https://cmake.org/cmake/help/v4.4/command/if.html#in-list) is tested to see if it is a defined variable. If so, the variable's value is used; otherwise the original value is used.

- The right hand argument to [`NOT`](https://cmake.org/cmake/help/v4.4/command/if.html#not) is tested to see if it is a boolean constant. If so, the value is used; otherwise it is assumed to be a variable and it is dereferenced.

- The left and right hand arguments to [`AND`](https://cmake.org/cmake/help/v4.4/command/if.html#and) and [`OR`](https://cmake.org/cmake/help/v4.4/command/if.html#or) are independently tested to see if they are boolean constants. If so, they are used as such, otherwise they are assumed to be variables and are dereferenced.

Changed in version 3.1: To prevent ambiguity, potential variable or keyword names can be specified in a [Quoted Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#quoted-argument) or a [Bracket Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#bracket-argument). A quoted or bracketed variable or keyword will be interpreted as a string and not dereferenced or interpreted. See policy [`CMP0054`](https://cmake.org/cmake/help/v4.4/policy/CMP0054.html#policy:CMP0054).

There is no automatic evaluation for environment or cache [Variable References](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#variable-references). Their values must be referenced as `$ENV{<name>}` or `$CACHE{<name>}` wherever the above-documented condition syntax accepts `<variable|string>`.

## See also

- [`else()`](https://cmake.org/cmake/help/v4.4/command/else.html#command:else)

- [`elseif()`](https://cmake.org/cmake/help/v4.4/command/elseif.html#command:elseif)

- [`endif()`](https://cmake.org/cmake/help/v4.4/command/endif.html#command:endif)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
