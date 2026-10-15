CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/build_command.html)

# build_command

Get a command line to build the current project. This is mainly intended for internal use by the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module.

~~~
build_command(<variable>
              [CONFIGURATION <config>]
              [PARALLEL_LEVEL <parallel>]
              [TARGET <target>]
              [PROJECT_NAME <projname>] # legacy, causes warning
             )
~~~

Sets the given `<variable>` to a command-line string of the form:

~~~
<cmake> --build . [--config <config>] [--parallel <parallel>] [--target <target>...]
~~~

where `<cmake>` is the location of the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command-line tool, and `<config>`, `<parallel>` and `<target>` are the values provided to the `CONFIGURATION`, `PARALLEL_LEVEL` and `TARGET` options, if any. In CMake versions prior to 4.0, a trailing `-- -i` option was added for [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) if policy [`CMP0061`](https://cmake.org/cmake/help/v4.4/policy/CMP0061.html#policy:CMP0061) was not set to `NEW`.

When invoked, this [`cmake --build`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-build) command line will launch the underlying build system tool.

Added in version 3.21: The `PARALLEL_LEVEL` argument can be used to set the [`--parallel`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-build-j) flag.

~~~
build_command(<cachevariable> <makecommand>)
~~~

This second signature is deprecated, but still available for backwards compatibility. Use the first signature instead.

It sets the given `<cachevariable>` to a command-line string as above but without the [`--target`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-build-t) option. The `<makecommand>` is ignored but should be the full path to devenv, nmake, make or one of the end user build tools for legacy invocations.

Note

In CMake versions prior to 3.0 this command returned a command line that directly invokes the native build tool for the current generator. Their implementation of the `PROJECT_NAME` option had no useful effects, so CMake now warns on use of the option.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
