CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_CONTEXT.html)

# CMAKE_MESSAGE_CONTEXT

Added in version 3.17.

When enabled by the [`cmake --log-context`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-log-context) command line option or the [`CMAKE_MESSAGE_CONTEXT_SHOW`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_CONTEXT_SHOW.html#variable:CMAKE_MESSAGE_CONTEXT_SHOW) variable, the [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) command converts the `CMAKE_MESSAGE_CONTEXT` list into a dot-separated string surrounded by square brackets and prepends it to each line for messages of log levels `NOTICE` and below.

For logging contexts to work effectively, projects should generally `APPEND` and `POP_BACK` an item to the current value of `CMAKE_MESSAGE_CONTEXT` rather than replace it. Projects should not assume the message context at the top of the source tree is empty, as there are scenarios where the context might have already been set (e.g. hierarchical projects).

Warning

Valid context names are restricted to anything that could be used as a CMake variable name. All names that begin with an underscore or the string `cmake_` are also reserved for use by CMake and should not be used by projects.

Example:

~~~
function(bar)
  list(APPEND CMAKE_MESSAGE_CONTEXT "bar")
  message(VERBOSE "bar VERBOSE message")
endfunction()

function(baz)
  list(APPEND CMAKE_MESSAGE_CONTEXT "baz")
  message(DEBUG "baz DEBUG message")
endfunction()

function(foo)
  list(APPEND CMAKE_MESSAGE_CONTEXT "foo")
  bar()
  message(TRACE "foo TRACE message")
  baz()
endfunction()

list(APPEND CMAKE_MESSAGE_CONTEXT "top")

message(VERBOSE "Before `foo`")
foo()
message(VERBOSE "After `foo`")

list(POP_BACK CMAKE_MESSAGE_CONTEXT)
~~~

Which results in the following output:

~~~
-- [top] Before `foo`
-- [top.foo.bar] bar VERBOSE message
-- [top.foo] foo TRACE message
-- [top.foo.baz] baz DEBUG message
-- [top] After `foo`
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
