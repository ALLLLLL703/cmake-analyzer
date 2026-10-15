CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_CONTEXT_SHOW.html)

# CMAKE_MESSAGE_CONTEXT_SHOW

Added in version 3.17.

Setting this variable to true enables showing a context with each line logged by the [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) command (see [`CMAKE_MESSAGE_CONTEXT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_CONTEXT.html#variable:CMAKE_MESSAGE_CONTEXT) for how the context itself is specified).

This variable is an alternative to providing the `--log-context` option on the [`cmake`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command line. Whereas the command line option will apply only to that one CMake run, setting `CMAKE_MESSAGE_CONTEXT_SHOW` to true as a cache variable will ensure that subsequent CMake runs will continue to show the message context.

Projects should not set `CMAKE_MESSAGE_CONTEXT_SHOW`. It is intended for users so that they may control whether or not to include context with messages.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
