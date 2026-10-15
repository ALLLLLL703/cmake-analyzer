CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html)

# CMAKE_\<LANG\>_COMPILER

The full path to the compiler for `LANG`.

This is the command that will be used as the `<LANG>` compiler. Once set, you can not change this variable.

Note

With [IDE Build Tool Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ide-build-tool-generators), including [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) and [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode), this variable is computed automatically and should not be set. Instead, specify a toolset using [`cmake -T`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-T).

## Usage

With [Command-Line Build Tool Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#command-line-build-tool-generators) this variable can be set by the user during the first time a build tree is configured.

If a non-full path value is supplied then CMake will resolve the full path of the compiler.

The variable could be set in a user supplied toolchain file or via [`-D`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-D) on the command line.

Note

Options that are required to make the compiler work correctly can be included as items in a list; they can not be changed.

~~~
#set within user supplied toolchain file
set(CMAKE_C_COMPILER /full/path/to/qcc --arg1 --arg2)
~~~

or

~~~
$ cmake ... -DCMAKE_C_COMPILER='qcc;--arg1;--arg2'
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
