CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_upload.html)

# ctest_upload

Upload files to a dashboard server as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_upload(FILES <file>... [QUIET] [CAPTURE_CMAKE_ERROR <result-var>])
~~~

The options are:

**`FILES <file>...`**

Specify a list of files to be sent along with the build results to the dashboard server.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.7.

Store in the `<result-var>` variable `-1` if there are any errors running the command and prevent [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) from returning non-zero if an error occurs.

**`QUIET`**

Added in version 3.3.

Suppress any CTest-specific non-error output that would have been printed to the console otherwise.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
