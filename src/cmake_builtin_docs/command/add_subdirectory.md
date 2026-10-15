CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html)

# add_subdirectory

Add a subdirectory to the build.

~~~
add_subdirectory(source_dir [binary_dir] [EXCLUDE_FROM_ALL] [SYSTEM])
~~~

Adds a subdirectory to the build. The `source_dir` specifies the directory in which the source `CMakeLists.txt` and code files are located. If it is a relative path, it will be evaluated with respect to the current directory (the typical usage), but it may also be an absolute path. The `binary_dir` specifies the directory in which to place the output files. If it is a relative path, it will be evaluated with respect to the current output directory, but it may also be an absolute path. If `binary_dir` is not specified, the value of `source_dir`, before expanding any relative path, will be used (the typical usage). The `CMakeLists.txt` file in the specified source directory will be processed immediately by CMake before processing in the current input file continues beyond this command.

If the `EXCLUDE_FROM_ALL` argument is provided then the [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_dir/EXCLUDE_FROM_ALL.html#prop_dir:EXCLUDE_FROM_ALL) property will be set on the added directory. This will exclude the directory from a default build. See the directory property [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_dir/EXCLUDE_FROM_ALL.html#prop_dir:EXCLUDE_FROM_ALL) for full details.

Added in version 3.25: If the `SYSTEM` argument is provided, the [`SYSTEM`](https://cmake.org/cmake/help/v4.4/prop_dir/SYSTEM.html#prop_dir:SYSTEM) directory property of the subdirectory will be set to true. This property is used to initialize the [`SYSTEM`](https://cmake.org/cmake/help/v4.4/prop_tgt/SYSTEM.html#prop_tgt:SYSTEM) property of each non-imported target created in that subdirectory.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
