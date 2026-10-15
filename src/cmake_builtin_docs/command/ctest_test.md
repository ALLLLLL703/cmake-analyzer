CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_test.html)

# ctest_test

Perform the [CTest Test Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-test-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_test([BUILD <build-dir>] [APPEND]
           [START <start-number>]
           [END <end-number>]
           [STRIDE <stride-number>]
           [EXCLUDE <exclude-regex>]
           [INCLUDE <include-regex>]
           [EXCLUDE_LABEL <label-exclude-regex>]
           [INCLUDE_LABEL <label-include-regex>]
           [EXCLUDE_FROM_FILE <filename>]
           [INCLUDE_FROM_FILE <filename>]
           [EXCLUDE_FIXTURE <regex>]
           [EXCLUDE_FIXTURE_SETUP <regex>]
           [EXCLUDE_FIXTURE_CLEANUP <regex>]
           [PARALLEL_LEVEL [<level>]]
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
           [PRESETS_FILE <file>]
           [QUIET]
           )
~~~

Run tests in the project build tree and store results in `Test.xml` for submission with the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command.

The options are:

**`BUILD <build-dir>`**

Specify the top-level build directory. If not given, the [`CTEST_BINARY_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BINARY_DIRECTORY.html#variable:CTEST_BINARY_DIRECTORY) variable is used.

**`APPEND`**

Mark `Test.xml` for append to results previously submitted to a dashboard server since the last [`ctest_start()`](https://cmake.org/cmake/help/v4.4/command/ctest_start.html#command:ctest_start) call. Append semantics are defined by the dashboard server in use. This does *not* cause results to be appended to a `.xml` file produced by a previous call to this command.

**`START <start-number>`**

Specify the beginning of a range of test numbers.

**`END <end-number>`**

Specify the end of a range of test numbers.

**`STRIDE <stride-number>`**

Specify the stride by which to step across a range of test numbers.

**`EXCLUDE <exclude-regex>`**

Specify a regular expression matching test names to exclude.

**`INCLUDE <include-regex>`**

Specify a regular expression matching test names to include. Tests not matching this expression are excluded.

**`EXCLUDE_LABEL <label-exclude-regex>`**

Specify a regular expression matching test labels to exclude.

**`INCLUDE_LABEL <label-include-regex>`**

Specify a regular expression matching test labels to include. Tests not matching this expression are excluded.

**`EXCLUDE_FROM_FILE <filename>`**

Added in version 3.29.

Do NOT run tests listed with their exact name in the given file.

**`INCLUDE_FROM_FILE <filename>`**

Added in version 3.29.

Only run the tests listed with their exact name in the given file.

**`EXCLUDE_FIXTURE <regex>`**

Added in version 3.7.

If a test in the set of tests to be executed requires a particular fixture, that fixture's setup and cleanup tests would normally be added to the test set automatically. This option prevents adding setup or cleanup tests for fixtures matching the `<regex>`. Note that all other fixture behavior is retained, including test dependencies and skipping tests that have fixture setup tests that fail.

**`EXCLUDE_FIXTURE_SETUP <regex>`**

Added in version 3.7.

Same as `EXCLUDE_FIXTURE` except only matching setup tests are excluded.

**`EXCLUDE_FIXTURE_CLEANUP <regex>`**

Added in version 3.7.

Same as `EXCLUDE_FIXTURE` except only matching cleanup tests are excluded.

**`PARALLEL_LEVEL [<level>]`**

Run tests in parallel, limited to a given level of parallelism.

Added in version 3.29: The `<level>` may be omitted, or `0`, to let [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) use a default level of parallelism, or unbounded parallelism, respectively, as documented by the [`ctest --parallel`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-j) option.

If given, this argument takes precedence over a test preset's [`jobs`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-testpresets-execution-jobs) field. Both this argument and that field take precedence over the [`CTEST_PARALLEL_LEVEL`](https://cmake.org/cmake/help/v4.4/envvar/CTEST_PARALLEL_LEVEL.html#envvar:CTEST_PARALLEL_LEVEL) environment variable, and over an explicit [`ctest --parallel`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-j) given on the command line. See [`CTEST_PARALLEL_LEVEL`](https://cmake.org/cmake/help/v4.4/envvar/CTEST_PARALLEL_LEVEL.html#envvar:CTEST_PARALLEL_LEVEL) for the full precedence order.

**`RESOURCE_SPEC_FILE <file>`**

Added in version 3.16.

Specify a [resource specification file](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-resource-specification-file). See [Resource Allocation](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-resource-allocation) for more information.

**`TEST_LOAD <threshold>`**

Added in version 3.4.

While running tests in parallel, try not to start tests when they may cause the CPU load to pass above a given threshold. If not specified the [`CTEST_TEST_LOAD`](https://cmake.org/cmake/help/v4.4/variable/CTEST_TEST_LOAD.html#variable:CTEST_TEST_LOAD) variable will be checked, and then the [`--test-load`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-test-load) command-line argument to [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)). See also the `TestLoad` setting in the [CTest Test Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-test-step).

**`REPEAT <mode>:<n>`**

Added in version 3.17.

Run tests repeatedly based on the given `<mode>` up to `<n>` times. The modes are:

**`UNTIL_FAIL`**

Require each test to run `<n>` times without failing in order to pass. This is useful in finding sporadic failures in test cases.

**`UNTIL_PASS`**

Allow each test to run up to `<n>` times in order to pass. Repeats tests if they fail for any reason. This is useful in tolerating sporadic failures in test cases.

**`AFTER_TIMEOUT`**

Allow each test to run up to `<n>` times in order to pass. Repeats tests only if they timeout. This is useful in tolerating sporadic timeouts in test cases on busy machines.

**`SCHEDULE_RANDOM <ON|OFF>`**

Launch tests in a random order. This may be useful for detecting implicit test dependencies.

**`STOP_ON_FAILURE`**

Added in version 3.18.

Stop the execution of the tests once one has failed.

**`STOP_TIME <time-of-day>`**

Specify a time of day at which the tests should all stop running.

**`RETURN_VALUE <result-var>`**

Store in the `<result-var>` variable `0` if all tests passed. Store non-zero if anything went wrong.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.7.

Store in the `<result-var>` variable `-1` if there are any errors running the command and prevent [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) from returning non-zero if an error occurs.

**`OUTPUT_JUNIT <file>`**

Added in version 3.21.

Write test results to `<file>` in JUnit XML format. If `<file>` is a relative path, it will be placed in the build directory. If `<file>` already exists, it will be overwritten. Note that the resulting JUnit XML file is **not** uploaded to CDash because it would be redundant with CTest's `Test.xml` file.

**`COVERAGE_TOOL <tool>`**

Added in version 4.4.

Specify the `<tool>` used for collecting coverage during the running of the tests. See the CTest [CoverageTool](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-coveragetool) setting for details.

**`PRESET <preset>`**

Added in version 4.4.

Specify a [`preset`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) to use when running tests. Any value set in the CTest script will take priority over a corresponding setting from the preset. For example, the `INCLUDE` argument will override the [`filter.include.name`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-testpresets-filter-include-name) setting from the chosen preset.

This option also applies to the [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck) command. See also the [`CTEST_TEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_TEST_PRESET.html#variable:CTEST_TEST_PRESET) and [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

In general, only preset fields which have equivalent options to `ctest_test` are supported. An exception is the [`environment`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-testpresets-environment), which can hold a value for the [`CTEST_NO_TESTS_ACTION`](https://cmake.org/cmake/help/v4.4/envvar/CTEST_NO_TESTS_ACTION.html#envvar:CTEST_NO_TESTS_ACTION) environment variable and will be respected, despite `ctest_test` not having an equivalent option to the [`noTestsAction`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-testpresets-execution-notestsaction) presets field.

**`PRESETS_FILE <file>`**

Added in version 4.4.

Specify a [`presets`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) file to use instead of the default `CMakePresets.json` in the source directory. A relative path is interpreted relative to the source directory. Has no effect unless a preset is selected via the `PRESET` argument or the [`CTEST_TEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_TEST_PRESET.html#variable:CTEST_TEST_PRESET) or [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

This option also applies to the [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck) command. See also the [`CTEST_PRESETS_FILE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESETS_FILE.html#variable:CTEST_PRESETS_FILE) variable.

**`QUIET`**

Added in version 3.3.

Suppress any CTest-specific non-error messages that would have otherwise been printed to the console. Output from the underlying test command is not affected. Summary info detailing the percentage of passing tests is also unaffected by the `QUIET` option.

See also the [`CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE.html#variable:CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE), [`CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE.html#variable:CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE) and [`CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION.html#variable:CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION) variables, along with their corresponding [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) command line options [`--test-output-size-passed`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-test-output-size-passed), [`--test-output-size-failed`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-test-output-size-failed), and [`--test-output-truncation`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-test-output-truncation).

## Additional Test Measurements

CTest can parse the output of your tests for extra measurements to report to CDash.

When run as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client), CTest will include these custom measurements in the `Test.xml` file that gets uploaded to CDash.

Check the [CDash test measurement documentation](https://github.com/Kitware/CDash/blob/master/docs/test_measurements.md) for more information on the types of test measurements that CDash recognizes.

Added in version 3.22: CTest can parse custom measurements from tags named `<CTestMeasurement>` or `<CTestMeasurementFile>`. The older names `<DartMeasurement>` and `<DartMeasurementFile>` are still supported.

The following example demonstrates how to output a variety of custom test measurements.

~~~
std::cout <<
  "<CTestMeasurement type=\"numeric/double\" name=\"score\">28.3</CTestMeasurement>"
  << std::endl;

std::cout <<
  "<CTestMeasurement type=\"text/string\" name=\"color\">red</CTestMeasurement>"
  << std::endl;

std::cout <<
  "<CTestMeasurement type=\"text/link\" name=\"CMake URL\">https://cmake.org</CTestMeasurement>"
  << std::endl;

std::cout <<
  "<CTestMeasurement type=\"text/preformatted\" name=\"Console Output\">" <<
  "line 1.\n" <<
  "  \033[31;1m line 2. Bold red, and indented!\033[0;0ml\n" <<
  "line 3. Not bold or indented...\n" <<
  "</CTestMeasurement>" << std::endl;
~~~

### Image Measurements

The following example demonstrates how to upload test images to CDash.

~~~
std::cout <<
  "<CTestMeasurementFile type=\"image/jpg\" name=\"TestImage\">" <<
  "/dir/to/test_img.jpg</CTestMeasurementFile>" << std::endl;

std::cout <<
  "<CTestMeasurementFile type=\"image/gif\" name=\"ValidImage\">" <<
  "/dir/to/valid_img.gif</CTestMeasurementFile>" << std::endl;

std::cout <<
  "<CTestMeasurementFile type=\"image/png\" name=\"AlgoResult\">" <<
  "/dir/to/img.png</CTestMeasurementFile>"
  << std::endl;
~~~

Images will be displayed together in an interactive comparison mode on CDash if they are provided with two or more of the following names.

- `TestImage`

- `ValidImage`

- `BaselineImage`

- `DifferenceImage2`

By convention, `TestImage` is the image generated by your test, and `ValidImage` (or `BaselineImage`) is basis of comparison used to determine if the test passed or failed.

If another image name is used it will be displayed by CDash as a static image separate from the interactive comparison UI.

### Attached Files

Added in version 3.21.

The following example demonstrates how to upload non-image files to CDash.

~~~
std::cout <<
  "<CTestMeasurementFile type=\"file\" name=\"TestInputData1\">" <<
  "/dir/to/data1.csv</CTestMeasurementFile>\n"                   <<
  "<CTestMeasurementFile type=\"file\" name=\"TestInputData2\">" <<
  "/dir/to/data2.csv</CTestMeasurementFile>"                     << std::endl;
~~~

If the name of the file to upload is known at configure time, you can use the [`ATTACHED_FILES`](https://cmake.org/cmake/help/v4.4/prop_test/ATTACHED_FILES.html#prop_test:ATTACHED_FILES) or [`ATTACHED_FILES_ON_FAIL`](https://cmake.org/cmake/help/v4.4/prop_test/ATTACHED_FILES_ON_FAIL.html#prop_test:ATTACHED_FILES_ON_FAIL) test properties instead.

### Custom Details

Added in version 3.21.

The following example demonstrates how to specify a custom value for the `Test Details` field displayed on CDash.

~~~
std::cout <<
  "<CTestDetails>My Custom Details Value</CTestDetails>" << std::endl;
~~~

### Additional Labels

Added in version 3.22.

The following example demonstrates how to add additional labels to a test at runtime.

~~~
std::cout <<
  "<CTestLabel>Custom Label 1</CTestLabel>\n" <<
  "<CTestLabel>Custom Label 2</CTestLabel>"   << std::endl;
~~~

Use the [`LABELS`](https://cmake.org/cmake/help/v4.4/prop_test/LABELS.html#prop_test:LABELS) test property instead for labels that can be determined at configure time.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
