CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/write_file.html)

# write_file

Deprecated since version 3.0: Use the [`file(WRITE)`](https://cmake.org/cmake/help/v4.4/command/file.html#write) command instead.

~~~
write_file(filename "message to write"... [APPEND])
~~~

The first argument is the file name, the rest of the arguments are messages to write. If the argument `APPEND` is specified, then the message will be appended.

NOTE 1: [`file(WRITE)`](https://cmake.org/cmake/help/v4.4/command/file.html#write) and [`file(APPEND)`](https://cmake.org/cmake/help/v4.4/command/file.html#append) do exactly the same as this one but add some more functionality.

NOTE 2: When using `write_file` the produced file cannot be used as an input to CMake (CONFIGURE_FILE, source file ...) because it will lead to an infinite loop. Use [`configure_file()`](https://cmake.org/cmake/help/v4.4/command/configure_file.html#command:configure_file) if you want to generate input files to CMake.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
