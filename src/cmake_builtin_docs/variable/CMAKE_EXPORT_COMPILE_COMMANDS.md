CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_COMPILE_COMMANDS.html)

# CMAKE_EXPORT_COMPILE_COMMANDS

Added in version 3.5.

Enable/Disable output of compile commands during generation.

If enabled, generates a `compile_commands.json` file containing the exact compiler calls for all translation units of the project in machine-readable form. The format of the JSON file looks like:

~~~
[
  {
    "directory": "/home/user/development/project",
    "command": "/usr/bin/c++ ... -c ../foo/foo.cc",
    "file": "../foo/foo.cc",
    "output": "../foo.dir/foo.cc.o"
  },

  ...

  {
    "directory": "/home/user/development/project",
    "command": "/usr/bin/c++ ... -c ../foo/bar.cc",
    "file": "../foo/bar.cc",
    "output": "../foo.dir/bar.cc.o"
  }
]
~~~

This is initialized by the [`CMAKE_EXPORT_COMPILE_COMMANDS`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_EXPORT_COMPILE_COMMANDS.html#envvar:CMAKE_EXPORT_COMPILE_COMMANDS) environment variable, and initializes the [`EXPORT_COMPILE_COMMANDS`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXPORT_COMPILE_COMMANDS.html#prop_tgt:EXPORT_COMPILE_COMMANDS) target property for all targets.

Note

This option is implemented only by [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) and [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators). It is ignored on other generators.

This option currently does not work well in combination with the [`UNITY_BUILD`](https://cmake.org/cmake/help/v4.4/prop_tgt/UNITY_BUILD.html#prop_tgt:UNITY_BUILD) target property or the [`CMAKE_UNITY_BUILD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_UNITY_BUILD.html#variable:CMAKE_UNITY_BUILD) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
