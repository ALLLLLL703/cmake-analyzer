CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MESSAGE_LOG_LEVEL.html)

# CMAKE_MESSAGE_LOG_LEVEL

Added in version 3.17.

When set, this variable specifies the logging level used by the [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) command. Valid values are the same as those for the [`--log-level`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-log-level) command line option of the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) program. If this variable is set and the [`--log-level`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-log-level) command line option is given, the command line option takes precedence.

The main advantage to using this variable is to make a log level persist between CMake runs. Setting it as a cache variable will ensure that subsequent CMake runs will continue to use the chosen log level.

Projects should not set this variable, it is intended for users so that they may control the log level according to their own needs.

Added in version 3.25: See the [`cmake_language(GET_MESSAGE_LOG_LEVEL)`](https://cmake.org/cmake/help/v4.4/command/cmake_language.html#get-message-log-level) command for a way to query the current message logging level.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
