CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_coverage.html)

# ctest_coverage

Perform the [CTest Coverage Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-coverage-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_coverage([BUILD <build-dir>] [APPEND]
               [LABELS <label>...]
               [RETURN_VALUE <result-var>]
               [CAPTURE_CMAKE_ERROR <result-var>]
               [QUIET]
               )
~~~

Collect coverage tool results and stores them in `Coverage.xml` for submission with the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command.

The options are:

**`BUILD <build-dir>`**

Specify the top-level build directory. If not given, the [`CTEST_BINARY_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BINARY_DIRECTORY.html#variable:CTEST_BINARY_DIRECTORY) variable is used.

**`APPEND`**

Mark `Coverage.xml` for append to results previously submitted to a dashboard server since the last [`ctest_start()`](https://cmake.org/cmake/help/v4.4/command/ctest_start.html#command:ctest_start) call. Append semantics are defined by the dashboard server in use. This does *not* cause results to be appended to a `.xml` file produced by a previous call to this command.

**`LABELS`**

Filter the coverage report to include only source files labeled with at least one of the labels specified.

**`RETURN_VALUE <result-var>`**

Store in the `<result-var>` variable `0` if coverage tools ran without error and non-zero otherwise.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.7.

Store in the `<result-var>` variable `-1` if there are any errors running the command and prevent [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) from returning non-zero if an error occurs.

**`QUIET`**

Added in version 3.3.

Suppress any CTest-specific non-error output that would have been printed to the console otherwise. The summary indicating how many lines of code were covered is unaffected by this option.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
