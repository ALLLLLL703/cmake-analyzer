CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_INDENT.html)

# CMAKE_MESSAGE_INDENT

Added in version 3.16.

The [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) command joins the strings from this list and for log levels of `NOTICE` and below, it prepends the resultant string to each line of the message.

Example:

~~~
list(APPEND listVar one two three)

message(VERBOSE [[Collected items in the "listVar":]])
list(APPEND CMAKE_MESSAGE_INDENT "  ")

foreach(item IN LISTS listVar)
  message(VERBOSE ${item})
endforeach()

list(POP_BACK CMAKE_MESSAGE_INDENT)
message(VERBOSE "No more indent")
~~~

Which results in the following output:

~~~
-- Collected items in the "listVar":
--   one
--   two
--   three
-- No more indent
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
