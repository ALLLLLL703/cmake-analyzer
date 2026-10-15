CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html)

# ctest_memcheck

Perform the [CTest MemCheck Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-memcheck-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_memcheck([BUILD <build-dir>] [APPEND]
               [START <start-number>]
               [END <end-number>]
               [STRIDE <stride-number>]
               [EXCLUDE <exclude-regex>]
               [INCLUDE <include-regex>]
               [EXCLUDE_LABEL <label-exclude-regex>]
               [INCLUDE_LABEL <label-include-regex>]
               [EXCLUDE_FIXTURE <regex>]
               [EXCLUDE_FIXTURE_SETUP <regex>]
               [EXCLUDE_FIXTURE_CLEANUP <regex>]
               [PARALLEL_LEVEL <level>]
               [RESOURCE_SPEC_FILE <file>]
               [TEST_LOAD <threshold>]
               [SCHEDULE_RANDOM <ON|OFF>]
               [STOP_ON_FAILURE]
               [STOP_TIME <time-of-day>]
               [RETURN_VALUE <result-var>]
               [CAPTURE_CMAKE_ERROR <result-var>]
               [REPEAT <mode>:<n>]
               [OUTPUT_JUNIT <file>]
               [PRESET <preset>]
               [PRESETS_FILE <path>]
               [DEFECT_COUNT <defect-count-var>]
               [QUIET]
               )
~~~

Run tests with a dynamic analysis tool and store results in `MemCheck.xml` for submission with the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command.

Most options are the same as those for the [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test) command.

The options unique to this command are:

**`DEFECT_COUNT <defect-count-var>`**

Added in version 3.8.

Store in the `<defect-count-var>` the number of defects found.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
