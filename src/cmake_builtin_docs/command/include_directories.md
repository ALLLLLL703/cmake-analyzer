CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/include_directories.html)

# include_directories

Add include directories to the build.

~~~
include_directories([AFTER|BEFORE] [SYSTEM] dir1 [dir2 ...])
~~~

Add the given directories to those the compiler uses to search for include files. Relative paths are interpreted as relative to the current source directory.

The include directories are added to the [`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_dir/INCLUDE_DIRECTORIES.html#prop_dir:INCLUDE_DIRECTORIES) directory property for the current `CMakeLists` file. They are also added to the [`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INCLUDE_DIRECTORIES.html#prop_tgt:INCLUDE_DIRECTORIES) target property for each target in the current `CMakeLists` file. The target property values are the ones used by the generators.

By default the directories specified are appended onto the current list of directories. This default behavior can be changed by setting [`CMAKE_INCLUDE_DIRECTORIES_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INCLUDE_DIRECTORIES_BEFORE.html#variable:CMAKE_INCLUDE_DIRECTORIES_BEFORE) to `ON`. By using `AFTER` or `BEFORE` explicitly, you can select between appending and prepending, independent of the default.

If the `SYSTEM` option is given, the compiler will be told the directories are meant as system include directories on some platforms. Signaling this setting might achieve effects such as the compiler skipping warnings, or these fixed-install system files not being considered in dependency calculations - see compiler docs.

Arguments to `include_directories` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

Note

Prefer the [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories) command to add include directories to individual targets and optionally propagate/export them to dependents.

## See Also

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
