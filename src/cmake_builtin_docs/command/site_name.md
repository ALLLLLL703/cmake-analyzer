CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/site_name.html)

# site_name

Set the given variable to the name of the computer.

~~~
site_name(variable)
~~~

On UNIX-like platforms, if the variable `HOSTNAME` is set, its value will be executed as a command expected to print out the host name, much like the `hostname` command-line tool.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
