CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/list.html)

# list

Operations on [semicolon-separated lists](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists).

## Synopsis

~~~
Reading
  list(LENGTH <list> <out-var>)
  list(GET <list> <element index> [<index> ...] <out-var>)
  list(JOIN <list> <glue> <out-var>)
  list(SUBLIST <list> <begin> <length> <out-var>)

Search
  list(FIND <list> <value> <out-var>)

Modification
  list(APPEND <list> [<element>...])
  list(FILTER <list> <INCLUDE|EXCLUDE> <MODE>)
  list(INSERT <list> <index> [<element>...])
  list(POP_BACK <list> [<out-var>...])
  list(POP_FRONT <list> [<out-var>...])
  list(PREPEND <list> [<element>...])
  list(REMOVE_ITEM <list> <value>...)
  list(REMOVE_AT <list> <index>...)
  list(REMOVE_DUPLICATES <list>)
  list(TRANSFORM <list> <ACTION> [...])

Ordering
  list(REVERSE <list>)
  list(SORT <list> [...])
~~~

## Introduction

The list subcommands [`APPEND`](https://cmake.org/cmake/help/v4.4/command/list.html#append), [`INSERT`](https://cmake.org/cmake/help/v4.4/command/list.html#insert), [`FILTER`](https://cmake.org/cmake/help/v4.4/command/list.html#filter), [`PREPEND`](https://cmake.org/cmake/help/v4.4/command/list.html#prepend), [`POP_BACK`](https://cmake.org/cmake/help/v4.4/command/list.html#pop-back), [`POP_FRONT`](https://cmake.org/cmake/help/v4.4/command/list.html#pop-front), [`REMOVE_AT`](https://cmake.org/cmake/help/v4.4/command/list.html#remove-at), [`REMOVE_ITEM`](https://cmake.org/cmake/help/v4.4/command/list.html#remove-item), [`REMOVE_DUPLICATES`](https://cmake.org/cmake/help/v4.4/command/list.html#remove-duplicates), [`REVERSE`](https://cmake.org/cmake/help/v4.4/command/list.html#reverse) and [`SORT`](https://cmake.org/cmake/help/v4.4/command/list.html#sort) may create new values for the list within the current CMake variable scope. Similar to the [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) command, the `list` command creates new variable values in the current scope, even if the list itself is actually defined in a parent scope. To propagate the results of these operations upwards, use [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) with `PARENT_SCOPE`, [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) with `CACHE INTERNAL`, or some other means of value propagation.

Note

A list in cmake is a `;` separated group of strings. To create a list, the [`set()`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set) command can be used. For example, `set(var a b c d e)` creates a list with `a;b;c;d;e`, and `set(var "a b c d e")` creates a string or a list with one item in it. (Note that macro arguments are not variables, and therefore cannot be used in `LIST` commands.)

Individual elements may not contain an unequal number of `[` and `]` characters, and may not end in a backslash (`\`). See [semicolon-separated lists](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) for details.

Note

When specifying index values, if `<element index>` is 0 or greater, it is indexed from the beginning of the list, with 0 representing the first list element. If `<element index>` is -1 or lesser, it is indexed from the end of the list, with -1 representing the last list element. Be careful when counting with negative indices: they do not start from 0. -0 is equivalent to 0, the first list element.

## Reading

**list(LENGTH \<list\> \<output variable\>)**

Returns the list's length.

**list(GET \<list\> \<element index\> [\<element index\> ...] \<output variable\>)**

Returns the list of elements specified by indices from the list.

**list(JOIN \<list\> \<glue\> \<output variable\>)**

Added in version 3.12.

Returns a string joining all list's elements using the glue string. To join multiple strings, which are not part of a list, use [`string(JOIN)`](https://cmake.org/cmake/help/v4.4/command/string.html#join).

**list(SUBLIST \<list\> \<begin\> \<length\> \<output variable\>)**

Added in version 3.12.

Returns a sublist of the given list. If `<length>` is 0, an empty list will be returned. If `<length>` is -1 or the list is smaller than `<begin>+<length>` then the remaining elements of the list starting at `<begin>` will be returned.

## Search

**list(FIND \<list\> \<value\> \<output variable\>)**

Returns the index of the element specified in the list or `-1` if it wasn't found.

## Modification

**list(APPEND \<list\> [\<element\> ...])**

Appends elements to the list. If no variable named `<list>` exists in the current scope its value is treated as empty and the elements are appended to that empty list.

**list(FILTER \<list\> \<INCLUDE|EXCLUDE\> \<MODE\>)**

Added in version 3.6.

Includes or removes items from the list that match the mode's pattern.

`<MODE>` must be one of the following:

**`REGEX`**

Items will be matched against the given regular expression.

~~~
list(FILTER <list> <INCLUDE|EXCLUDE> REGEX <regular_expression>)
~~~

For more information on regular expressions look under [string(REGEX)](https://cmake.org/cmake/help/v4.4/command/string.html#regex-specification).

**`PREDICATE`**

Specify a user-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) as a predicate.

~~~
list(FILTER <list> <INCLUDE|EXCLUDE> PREDICATE <function>)
~~~

Added in version 4.4.

`<function>` is a user-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) that acts as a unary predicate. The function must accept exactly two parameters: the input value and the name of an output variable. The function must set the output variable to a boolean value in the calling scope. The output variable is interpreted using standard CMake boolean evaluation. If the function does not set the output variable, it is an error.

Example:

~~~
function(file_exists path result)
  if(EXISTS "${path}")
    set(${result} TRUE PARENT_SCOPE)
  else()
    set(${result} FALSE PARENT_SCOPE)
  endif()
endfunction()

set(candidate_files main.c missing.c utils.c)
list(FILTER candidate_files INCLUDE PREDICATE file_exists)
~~~

**list(INSERT \<list\> \<element_index\> \<element\> [\<element\> ...])**

Inserts elements to the list to the specified index. It is an error to specify an out-of-range index. Valid indexes are *0* to *N* where *N* is the length of the list, inclusive. An empty list has length 0. If no variable named `<list>` exists in the current scope its value is treated as empty and the elements are inserted in that empty list.

**list(POP_BACK \<list\> [\<out-var\>...])**

Added in version 3.15.

If no variable name is given, removes exactly one element. Otherwise, with *N* variable names provided, assign the last *N* elements' values to the given variables and then remove the last *N* values from `<list>`.

**list(POP_FRONT \<list\> [\<out-var\>...])**

Added in version 3.15.

If no variable name is given, removes exactly one element. Otherwise, with *N* variable names provided, assign the first *N* elements' values to the given variables and then remove the first *N* values from `<list>`.

**list(PREPEND \<list\> [\<element\> ...])**

Added in version 3.15.

Insert elements to the 0th position in the list. If no variable named `<list>` exists in the current scope its value is treated as empty and the elements are prepended to that empty list.

**list(REMOVE_ITEM \<list\> \<value\> [\<value\> ...])**

Removes all instances of the given items from the list.

**list(REMOVE_AT \<list\> \<index\> [\<index\> ...])**

Removes items at given indices from the list.

**list(REMOVE_DUPLICATES \<list\>)**

Removes duplicated items in the list. The relative order of items is preserved, but if duplicates are encountered, only the first instance is preserved.

**list(TRANSFORM \<list\> \<ACTION\> [\<SELECTOR\>] [OUTPUT_VARIABLE \<output variable\>])**

Added in version 3.12.

Transforms the list by applying an `<ACTION>` to all or, by specifying a `<SELECTOR>`, to the selected elements of the list, storing the result in-place or in the specified output variable.

Note

The `TRANSFORM` sub-command does not change the number of elements in the list. If a `<SELECTOR>` is specified, only some elements will be changed, the other ones will remain the same as before the transformation.

`<ACTION>` specifies the action to apply to the elements of the list. `<ACTION>` must be one of the following:

**[`APPEND`](https://cmake.org/cmake/help/v4.4/command/string.html#append), [`PREPEND`](https://cmake.org/cmake/help/v4.4/command/string.html#prepend)**

Append, prepend specified value to each element of the list.

**list(TRANSFORM \<list\> (APPEND|PREPEND) \<value\> ...)**

**[`TOLOWER`](https://cmake.org/cmake/help/v4.4/command/string.html#tolower), [`TOUPPER`](https://cmake.org/cmake/help/v4.4/command/string.html#toupper)**

Convert each element of the list to lower, upper characters.

**list(TRANSFORM \<list\> (TOLOWER|TOUPPER) ...)**

**[`STRIP`](https://cmake.org/cmake/help/v4.4/command/string.html#strip)**

Remove leading and trailing spaces from each element of the list.

**list(TRANSFORM \<list\> STRIP ...)**

**[`GENEX_STRIP`](https://cmake.org/cmake/help/v4.4/command/string.html#genex-strip)**

Strip any [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) from each element of the list.

**list(TRANSFORM \<list\> GENEX_STRIP ...)**

**[`REPLACE`](https://cmake.org/cmake/help/v4.4/command/string.html#regex-replace)**

Match the regular expression as many times as possible and substitute the replacement expression for the match for each element of the list (same semantic as [`string(REGEX REPLACE)`](https://cmake.org/cmake/help/v4.4/command/string.html#regex-replace)).

**list(TRANSFORM \<list\> REPLACE \<regular_expression\> \<replace_expression\> ...)**

Changed in version 4.1: The `^` anchor now matches only at the beginning of the input element instead of the beginning of each repeated search. See policy [`CMP0186`](https://cmake.org/cmake/help/v4.4/policy/CMP0186.html#policy:CMP0186).

**`APPLY`**

Invoke a user-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) for each element of the list. The function must accept exactly two parameters: the input value and the name of an output variable. The function must set the output variable in the calling scope.

**list(TRANSFORM \<list\> APPLY \<function\> ...)**

Added in version 4.4.

`<function>` is a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) with exactly two formal parameters. Set the output variable via [`set(<variable> <value> PARENT_SCOPE)`](https://cmake.org/cmake/help/v4.4/command/set.html#command:set):

~~~
function(<function> <input> <output>)
  # Transform <input>, store result in ${<output>}
  set(${<output>} "<result>" PARENT_SCOPE)
endfunction()
~~~

Before each invocation, the output variable is unset in the calling scope to prevent stale values.

Example:

~~~
function(make_absolute in out)
  cmake_path(ABSOLUTE_PATH in BASE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
  set(${out} "${in}" PARENT_SCOPE)
endfunction()

set(mylist main.c utils.c io.c)
list(TRANSFORM mylist APPLY make_absolute)
# mylist is now absolute paths relative to CMAKE_CURRENT_SOURCE_DIR
~~~

`<SELECTOR>` determines which elements of the list will be transformed. Only one type of selector can be specified at a time. When given, `<SELECTOR>` must be one of the following:

**`AT`**

Specify a list of indexes.

~~~
list(TRANSFORM <list> <ACTION> AT <index> [<index> ...] ...)
~~~

**`FOR`**

Specify a range with, optionally, an increment used to iterate over the range.

~~~
list(TRANSFORM <list> <ACTION> FOR <start> <stop> [<step>] ...)
~~~

**`REGEX`**

Specify a regular expression. Only elements matching the regular expression will be transformed.

~~~
list(TRANSFORM <list> <ACTION> REGEX <regular_expression> ...)
~~~

**`PREDICATE`**

Specify a user-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) as a predicate. Only elements for which the function returns a true value will be transformed.

~~~
list(TRANSFORM <list> <ACTION> PREDICATE <function> ...)
~~~

Added in version 4.4.

`<function>` is a user-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) with exactly two formal parameters: the input value and the name of an output variable. The function must set the output variable to a boolean value. Standard CMake boolean evaluation is used. If the function does not set the output variable, it is an error. The function is evaluated for each element in turn, immediately before that element would be transformed.

Example:

~~~
function(is_relative path result)
  if(NOT IS_ABSOLUTE "${path}")
    set(${result} TRUE PARENT_SCOPE)
  else()
    set(${result} FALSE PARENT_SCOPE)
  endif()
endfunction()

set(search_paths /usr/include src lib /opt/lib)
list(TRANSFORM search_paths PREPEND "${CMAKE_CURRENT_SOURCE_DIR}/"
     PREDICATE is_relative)
~~~

## Ordering

**list(REVERSE \<list\>)**

Reverses the contents of the list in-place.

**list(SORT \<list\> [COMPARE \<compare\>] [CASE \<case\>] [ORDER \<order\>])**

**list(SORT \<list\> COMPARATOR \<function\> [CASE \<case\>] [ORDER \<order\>])**

Sorts the list in-place alphabetically.

Added in version 3.13: Added the `COMPARE`, `CASE`, and `ORDER` options.

Added in version 3.18: Added the `COMPARE NATURAL` option.

Use the `COMPARE` keyword to select the comparison method for sorting. The `<compare>` option should be one of:

**`STRING`**

Sorts a list of strings alphabetically. This is the default behavior if the `COMPARE` option is not given.

**`FILE_BASENAME`**

Sorts a list of pathnames of files by their basenames.

**`NATURAL`**

Sorts a list of strings using natural order (see `strverscmp(3)` manual), i.e. such that contiguous digits are compared as whole numbers. For example: the following list *10.0 1.1 2.1 8.0 2.0 3.1* will be sorted as *1.1 2.0 2.1 3.1 8.0 10.0* if the `NATURAL` comparison is selected where it will be sorted as *1.1 10.0 2.0 2.1 3.1 8.0* with the `STRING` comparison.

Use the `CASE` keyword to select a case sensitive or case insensitive sort mode. The `<case>` option should be one of:

**`SENSITIVE`**

List items are sorted in a case-sensitive manner. This is the default behavior if the `CASE` option is not given.

**`INSENSITIVE`**

List items are sorted case insensitively. The order of items which differ only by upper/lowercase is not specified.

To control the sort order, the `ORDER` keyword can be given. The `<order>` option should be one of:

**`ASCENDING`**

Sorts the list in ascending order. This is the default behavior when the `ORDER` option is not given.

**`DESCENDING`**

Sorts the list in descending order.

Instead of the built-in `COMPARE` methods, a user-defined comparison [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) may be used with the `COMPARATOR` keyword.

Added in version 4.4.

`COMPARATOR` is mutually exclusive with `COMPARE`. The `CASE` and `ORDER` options may still be used alongside `COMPARATOR`: the `CASE` filter is applied to both values before they are passed to the function, and `ORDER DESCENDING` reverses the comparison by swapping the two arguments.

The function must accept exactly three parameters: two input values and the name of an output variable. It must set the output variable to a boolean value (`TRUE` if the first value should come before the second, `FALSE` otherwise) in the calling scope. If the function does not set the output variable, it is an error.

The comparator must define a [strict weak ordering](https://en.wikipedia.org/wiki/Weak_ordering#Strict_weak_orderings). In particular:

- It must return `FALSE` when comparing an element to itself (irreflexivity).

- If it returns `TRUE` for `(a, b)`, it must return `FALSE` for `(b, a)` (asymmetry).

An error is raised if the comparator violates these requirements.

Example:

~~~
function(version_less a b result)
  if("${a}" VERSION_LESS "${b}")
    set(${result} TRUE PARENT_SCOPE)
  else()
    set(${result} FALSE PARENT_SCOPE)
  endif()
endfunction()

set(versions 3.1 1.2 2.0 1.10)
list(SORT versions COMPARATOR version_less)
# versions is now: 1.2;1.10;2.0;3.1
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
