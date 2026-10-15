CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/load_command.html)

# load_command

Disallowed since version 3.0. See CMake Policy [`CMP0031`](https://cmake.org/cmake/help/v4.4/policy/CMP0031.html#policy:CMP0031).

Load a command into a running CMake.

~~~
load_command(COMMAND_NAME <loc1> [loc2 ...])
~~~

The given locations are searched for a library whose name is cmCOMMAND_NAME. If found, it is loaded as a module and the command is added to the set of available CMake commands. Usually, [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) is used before this command to compile the module. If the command is successfully loaded a variable named `CMAKE_LOADED_COMMAND_<COMMAND_NAME>` will be set to the full path of the module that was loaded. Otherwise the variable will not be set.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
